//! MILESTONE m18 (part 1) — Streaming input and following a growing file (Chapter 18)
//! Run: cargo test --test m18_io   (then re-run: cargo test --test m15_pipelines)
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider: what does `read_until` do with a
//! buffer that is not empty, and what does it return at the end of the input?
//! IMPLEMENT `src/pipeline.rs`: `process_reader`; `src/follow.rs`: `Follower::poll`,
//! `Follower::finish`.
//! REFACTOR: give `process` and `process_reader` one private per-line helper. The
//! m15 tests and the equivalence tests below are the safety net.
//! EXPLAIN afterward: why can a `Record` not be kept after the next `read_until`?
//! What does a reader that returns a partial line mean for a follower?

mod common;

use common::{TempDir, count, guess};
use loglens::follow::Follower;
use loglens::pipeline::{process, process_reader};
use loglens::sink::Count;
use loglens::{ErrorKind, Filter, ParseError, Tally};
use std::cell::RefCell;
use std::io::{self, BufRead, BufReader, Cursor, Read, Write};
use std::rc::Rc;

const TEXT: &str = concat!(
    "# input\n",
    "2026-10-23T17:03:00Z INFO auth user=ada latency_ms=42\n",
    "\n",
    "2026-10-23T17:03:02Z WARN auth user=bob msg=\"slow\"\r\n",
    "garbage\n",
    "2026-10-23T17:03:05Z ERROR db msg=\"reset\"\n",
    "2026-10-23T17:03:06Z DEBUG cache hit=true",
);

/// Everything a run produces.
fn run_text(text: &str, filter: &Filter) -> (Tally, Vec<String>, Vec<ParseError>) {
    let (mut lines, mut errors) = (Vec::new(), Vec::new());
    let tally = process(text, filter, &mut lines, &mut |e| errors.push(e)).unwrap();
    (tally, lines, errors)
}

fn run_reader(reader: impl BufRead, filter: &Filter) -> (Tally, Vec<String>, Vec<ParseError>) {
    let (mut lines, mut errors) = (Vec::new(), Vec::new());
    let tally = process_reader(reader, filter, &mut lines, &mut |e| errors.push(e)).unwrap();
    (tally, lines, errors)
}

#[test]
fn a_process_reader_equals_process() {
    for filter in ["", "level>=warn", "target=auth", "level=trace"] {
        let filter = Filter::parse(filter).unwrap();
        let expected = run_text(TEXT, &filter);
        assert_eq!(run_reader(TEXT.as_bytes(), &filter), expected);
        // A tiny buffer forces lines to span many reads.
        let tiny = BufReader::with_capacity(3, TEXT.as_bytes());
        assert_eq!(run_reader(tiny, &filter), expected);
    }
    let (tally, lines, _) = run_reader(TEXT.as_bytes(), &Filter::all());
    assert_eq!(
        tally,
        Tally {
            records: 4,
            matched: 4,
            malformed: 1
        }
    );
    assert_eq!(
        lines[1], r#"2026-10-23T17:03:02Z WARN auth user=bob msg="slow""#,
        "no \\r"
    );
    assert_eq!(
        lines[3], "2026-10-23T17:03:06Z DEBUG cache hit=true",
        "no final newline"
    );
}

#[test]
fn a_invalid_utf8_is_a_malformed_line() {
    let mut bytes = b"2026-10-23T17:03:00Z INFO upload file=caf".to_vec();
    bytes.extend_from_slice(b"\xe9.txt\n2026-10-23T17:03:01Z INFO upload x=\xc3\xa9\xff\n");
    bytes.extend_from_slice(b"2026-10-23T17:03:02Z INFO upload ok=1\n");
    let (tally, lines, errors) = run_reader(&bytes[..], &Filter::all());
    assert_eq!(
        tally,
        Tally {
            records: 1,
            matched: 1,
            malformed: 2
        }
    );
    assert_eq!(lines, ["2026-10-23T17:03:02Z INFO upload ok=1"]);
    assert_eq!(
        errors,
        [
            ParseError::new(ErrorKind::InvalidUtf8, 42).at_line(1),
            ParseError::new(ErrorKind::InvalidUtf8, 37).at_line(2),
        ],
        "one past the valid characters"
    );
}

#[test]
fn a_process_reader_reuses_one_buffer() {
    let text = "2026-10-23T17:03:00Z INFO auth\n".repeat(2000);
    let filter = Filter::all();
    let mut sink = Count::default();
    let (tally, allocations) =
        count(|| process_reader(text.as_bytes(), &filter, &mut sink, &mut |_| {}));
    assert_eq!(tally.unwrap().records, 2000);
    assert!(
        allocations < 50,
        "2000 lines made {allocations} allocations: reuse the line buffer"
    );
}

/// Returns some data, then fails.
struct FailsAfter(&'static [u8]);

impl Read for FailsAfter {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if self.0.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::ConnectionReset,
                "peer went away",
            ));
        }
        let n = self.0.len().min(buf.len());
        buf[..n].copy_from_slice(&self.0[..n]);
        self.0 = &self.0[n..];
        Ok(n)
    }
}

#[test]
fn a_reader_errors_are_returned() {
    let reader = BufReader::new(FailsAfter(b"2026-10-23T17:03:00Z INFO auth\n"));
    let mut lines: Vec<String> = Vec::new();
    let err = process_reader(reader, &Filter::all(), &mut lines, &mut |_| {}).unwrap_err();
    assert_eq!(err.kind(), io::ErrorKind::ConnectionReset);
    assert_eq!(
        lines.len(),
        1,
        "the complete line before the error was processed"
    );
}

/// An in-memory "file" that another part of the test appends to.
#[derive(Clone, Default)]
struct Growing(Rc<RefCell<(Vec<u8>, usize)>>);

impl Growing {
    fn append(&self, bytes: &str) {
        self.0.borrow_mut().0.extend_from_slice(bytes.as_bytes());
    }
}

impl Read for Growing {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let mut state = self.0.borrow_mut();
        let (data, pos) = &mut *state;
        let n = (data.len() - *pos).min(buf.len());
        buf[..n].copy_from_slice(&data[*pos..*pos + n]);
        *pos += n;
        Ok(n)
    }
}

#[test]
fn b_follower_processes_only_complete_lines() {
    let file = Growing::default();
    let mut follower = Follower::new(BufReader::new(file.clone()));
    let filter = Filter::all();
    let (mut lines, mut errors) = (Vec::<String>::new(), Vec::new());

    file.append("2026-10-23T17:03:00Z INFO auth user=ada\n2026-10-23T17:03:01Z WARN au");
    let t = follower
        .poll(&filter, &mut lines, &mut |e| errors.push(e))
        .unwrap();
    assert_eq!(
        t,
        Tally {
            records: 1,
            matched: 1,
            malformed: 0
        }
    );
    assert_eq!(follower.pending(), b"2026-10-23T17:03:01Z WARN au");

    let t = follower
        .poll(&filter, &mut lines, &mut |e| errors.push(e))
        .unwrap();
    assert_eq!(t, Tally::default(), "nothing new");

    file.append("th user=bob\noops\n");
    let t = follower
        .poll(&filter, &mut lines, &mut |e| errors.push(e))
        .unwrap();
    assert_eq!(
        t,
        Tally {
            records: 1,
            matched: 1,
            malformed: 1
        }
    );
    assert_eq!(
        lines[1], "2026-10-23T17:03:01Z WARN auth user=bob",
        "completed, not split"
    );
    assert_eq!(
        errors,
        [ParseError::new(ErrorKind::BadTimestamp, 1).at_line(3)]
    );
    assert!(follower.pending().is_empty());

    file.append("2026-10-23T17:03:02Z ERROR db");
    let t = follower
        .poll(&filter, &mut lines, &mut |e| errors.push(e))
        .unwrap();
    assert_eq!(t, Tally::default());
    let t = follower
        .finish(&filter, &mut lines, &mut |e| errors.push(e))
        .unwrap();
    assert_eq!(
        t,
        Tally {
            records: 1,
            matched: 1,
            malformed: 0
        }
    );
    assert_eq!(lines.len(), 3);
    assert!(follower.pending().is_empty());
    let t = follower
        .finish(&filter, &mut lines, &mut |e| errors.push(e))
        .unwrap();
    assert_eq!(t, Tally::default());
}

#[test]
fn b_follower_reads_what_is_appended_to_a_real_file() {
    let dir = TempDir::new("follow");
    let path = dir.file("app.log", "2026-10-23T17:03:00Z INFO auth\n");
    let mut follower = Follower::new(BufReader::new(std::fs::File::open(&path).unwrap()));
    let filter = Filter::parse("level>=warn").unwrap();
    let mut lines: Vec<String> = Vec::new();

    let t = follower.poll(&filter, &mut lines, &mut |_| {}).unwrap();
    assert_eq!((t.records, t.matched), (1, 0));

    let mut writer = std::fs::OpenOptions::new()
        .append(true)
        .open(&path)
        .unwrap();
    write!(writer, "2026-10-23T17:03:01Z WARN auth slow=").unwrap();
    writer.flush().unwrap();
    let t = follower.poll(&filter, &mut lines, &mut |_| {}).unwrap();
    assert_eq!(t, Tally::default(), "half a line waits");

    writeln!(writer, "yes").unwrap();
    writer.flush().unwrap();
    let t = follower.poll(&filter, &mut lines, &mut |_| {}).unwrap();
    assert_eq!((t.records, t.matched), (1, 1));
    assert_eq!(lines, ["2026-10-23T17:03:01Z WARN auth slow=yes"]);
}

/// P. Predict BEFORE running: replace each `guess()` with your answer.
#[test]
fn p_predictions() {
    let mut reader = Cursor::new("ab\ncd");
    let mut buf = Vec::new();

    // P1. What does the first read_until return?
    let n1 = reader.read_until(b'\n', &mut buf).unwrap();
    let p1: usize = guess();
    assert_eq!(p1, n1, "P1: the count includes the delimiter");

    // P2. Without clearing, what does `buf` hold after the second call?
    reader.read_until(b'\n', &mut buf).unwrap();
    let p2: &str = guess();
    assert_eq!(
        p2,
        std::str::from_utf8(&buf).unwrap(),
        "P2: read_until appends"
    );

    // P3. And a third call, at the end of the input?
    let p3: usize = guess();
    assert_eq!(
        p3,
        reader.read_until(b'\n', &mut buf).unwrap(),
        "P3: 0 means EOF"
    );

    // P4. How many bytes of b"ok\xff!" are valid UTF-8 before the error?
    let bytes = vec![b'o', b'k', 0xff, b'!'];
    let p4: usize = guess();
    assert_eq!(
        p4,
        std::str::from_utf8(&bytes).unwrap_err().valid_up_to(),
        "P4: valid_up_to counts bytes"
    );
}
