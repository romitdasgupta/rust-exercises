//! MILESTONE m11 — Traits: Sink, Display, FromStr, From, TryFrom (Chapters 11–13)
//! Run: cargo test --test m11_traits
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider: why does `Sink::accept` take
//! `&Record<'_>` and not `Record<'a>`? Which sinks need generics, which `dyn`?
//! IMPLEMENT `src/level.rs`: `FromStr`, `Display`, `TryFrom<u8>`, `From<Level>`;
//! `src/record.rs`: `Display`; `src/sink.rs`: every `Sink` impl.
//! EXPLAIN afterward: when would you choose `&mut dyn Sink` over `S: Sink`? What do the
//! blanket impls for `&mut S` and `Box<S>` buy callers?

mod common;

use common::{count, guess};
use loglens::level::UnknownLevel;
use loglens::sink::{Count, LineWriter, Tee};
use loglens::{Level, Record, Sink, parse_record};
use std::io::{self, Write};

const LINES: [&str; 3] = [
    r#"2026-10-23T17:03:00Z INFO auth user=ada latency_ms=42 msg="login ok""#,
    r#"2026-10-23T17:03:02Z WARN auth user=bob msg="say \"hi\"" note="""#,
    "2026-10-23T17:03:09Z DEBUG db",
];

fn records() -> Vec<Record<'static>> {
    LINES.iter().map(|l| parse_record(l).unwrap()).collect()
}

#[test]
fn a_level_from_str() {
    assert_eq!("warn".parse::<Level>(), Ok(Level::Warn));
    assert_eq!("WARNING".parse::<Level>(), Ok(Level::Warn));
    assert_eq!(
        "loud".parse::<Level>(),
        Err(UnknownLevel("loud".to_string()))
    );
    assert_eq!(
        "loud".parse::<Level>().unwrap_err().to_string(),
        r#"unknown level "loud""#
    );
}

#[test]
fn b_level_display_honors_padding() {
    assert_eq!(Level::Info.to_string(), "INFO");
    assert_eq!(format!("[{:<5}]", Level::Info), "[INFO ]");
    assert_eq!(format!("[{:>6}]", Level::Warn), "[  WARN]");
    assert_eq!(format!("[{:^7}]", Level::Error), "[ ERROR ]");
}

#[test]
fn c_numeric_severities() {
    for (n, level) in (0u8..).zip(Level::ALL) {
        assert_eq!(Level::try_from(n), Ok(level));
        assert_eq!(u8::from(level), n);
    }
    assert_eq!(Level::try_from(5), Err(5));
    assert_eq!(Level::try_from(255), Err(255));
    let level: Level = 2u8.try_into().unwrap();
    assert_eq!(level, Level::Info);
}

#[test]
fn d_record_display_is_canonical() {
    let messy = "  2026-10-23T17:03:00Z   warn\tauth  user=ada   msg=\"login ok\"  ";
    let r = parse_record(messy).unwrap();
    assert_eq!(
        r.to_string(),
        r#"2026-10-23T17:03:00Z WARN auth user=ada msg="login ok""#
    );
    for line in LINES {
        let r = parse_record(line).unwrap();
        let text = r.to_string();
        assert_eq!(text, line, "canonical lines print unchanged");
        assert_eq!(parse_record(&text).unwrap(), r, "display round-trips");
    }
}

/// Accepts and discards bytes. (Unlike `io::sink()`, it makes `write!` format.)
struct Discard;

impl Write for Discard {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        Ok(buf.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[test]
fn d_record_display_does_not_allocate() {
    let records = records();
    let (result, allocations) = count(|| {
        let mut out = Discard;
        for r in &records {
            write!(out, "{r}")?;
        }
        io::Result::Ok(())
    });
    result.unwrap();
    assert_eq!(allocations, 0, "format into the Formatter directly");
}

#[test]
fn e_vec_sink_keeps_lines() {
    let mut lines: Vec<String> = Vec::new();
    for r in &records() {
        lines.accept(r).unwrap();
    }
    lines.finish().unwrap();
    assert_eq!(lines, LINES);
}

#[test]
fn f_count_sink() {
    let mut n = Count::default();
    for r in &records() {
        n.accept(r).unwrap();
    }
    assert_eq!(n, Count(3));
}

/// Records whether `flush` was called.
#[derive(Default)]
struct Flushes {
    bytes: Vec<u8>,
    flushed: bool,
}

impl Write for Flushes {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.bytes.extend_from_slice(buf);
        Ok(buf.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        self.flushed = true;
        Ok(())
    }
}

#[test]
fn g_line_writer() {
    let mut sink = LineWriter::new(Flushes::default());
    for r in &records() {
        sink.accept(r).unwrap();
    }
    assert!(!sink.get_ref().flushed);
    sink.finish().unwrap();
    let out = sink.into_inner();
    assert!(out.flushed, "finish flushes");
    assert_eq!(
        String::from_utf8(out.bytes).unwrap(),
        LINES.join("\n") + "\n"
    );
}

#[test]
fn g_line_writer_does_not_allocate_per_record() {
    let records = records();
    let mut sink = LineWriter::new(Discard);
    let (result, allocations) = count(|| {
        for _ in 0..100 {
            for r in &records {
                sink.accept(r)?;
            }
        }
        sink.finish()
    });
    result.unwrap();
    assert_eq!(allocations, 0, "writeln! straight into the writer");
}

/// A sink whose operations fail, recording what was called.
#[derive(Default)]
struct Failing {
    accepts: u32,
    finishes: u32,
}

impl Sink for Failing {
    fn accept(&mut self, _: &Record<'_>) -> io::Result<()> {
        self.accepts += 1;
        Err(io::Error::other("accept failed"))
    }
    fn finish(&mut self) -> io::Result<()> {
        self.finishes += 1;
        Err(io::Error::other("finish failed"))
    }
}

#[test]
fn h_tee_sends_to_both() {
    let mut tee = Tee(Vec::<String>::new(), Count::default());
    for r in &records() {
        tee.accept(r).unwrap();
    }
    tee.finish().unwrap();
    assert_eq!(tee.0.len(), 3);
    assert_eq!(tee.1, Count(3));
}

#[test]
fn h_tee_errors() {
    let r = &records()[0];
    let mut tee = Tee(Failing::default(), Count::default());
    assert_eq!(tee.accept(r).unwrap_err().to_string(), "accept failed");
    assert_eq!(tee.1, Count(0), "accept stops at the first error");

    let mut tee = Tee(Failing::default(), Failing::default());
    assert_eq!(tee.finish().unwrap_err().to_string(), "finish failed");
    assert_eq!(
        (tee.0.finishes, tee.1.finishes),
        (1, 1),
        "both are finished"
    );
}

fn feed_all<S: Sink + ?Sized>(sink: &mut S, records: &[Record<'_>]) {
    for r in records {
        sink.accept(r).unwrap();
    }
    sink.finish().unwrap();
}

fn feed_owned<S: Sink>(mut sink: S, records: &[Record<'_>]) {
    feed_all(&mut sink, records);
}

#[test]
fn i_sinks_compose() {
    let records = records();
    let mut sinks: Vec<Box<dyn Sink>> = vec![
        Box::new(Count::default()),
        Box::new(Vec::<String>::new()),
        Box::new(LineWriter::new(io::sink())),
    ];
    for sink in &mut sinks {
        feed_all(sink, &records);
    }
    let mut n = Count::default();
    feed_owned(&mut n, &records);
    assert_eq!(n, Count(3), "a &mut S lends the sink");
    let boxed: Box<Count> = Box::default();
    feed_owned(boxed, &records);
    let dynamic: &mut dyn Sink = &mut n;
    feed_all(dynamic, &records);
    assert_eq!(n, Count(6));
}

/// P. Predict BEFORE running: replace each `guess()` with your answer.
/// The sizes assume a 64-bit target.
#[test]
fn p_predictions() {
    // P1. Width and alignment with a plain &str.
    let p1: String = guess();
    assert_eq!(
        p1,
        format!("[{:^7}]", "INFO"),
        "P1: center, extra space on the right"
    );

    // P2. Converting 300i32 to u8 with TryFrom: does it fail?
    let p2: bool = guess();
    assert_eq!(
        p2,
        u8::try_from(300i32).is_err(),
        "P2: TryFrom checks the range"
    );

    // P3. The sizes of a boxed trait object and a boxed concrete type.
    let p3: (usize, usize) = guess();
    assert_eq!(
        p3,
        (size_of::<Box<dyn Sink>>(), size_of::<Box<Count>>()),
        "P3: dyn pointers carry a vtable pointer"
    );

    // P4. Default for a tuple.
    let p4: (u8, String, Option<i32>) = guess();
    assert_eq!(p4, Default::default(), "P4: Default is defined per field");
}
