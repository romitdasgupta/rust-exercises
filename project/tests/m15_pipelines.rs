//! MILESTONE m15 — Iterator pipelines (Chapter 15)
//! Run: cargo test --test m15_pipelines
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider: which adapter keeps the line number
//! when a line is skipped? Where does a lazy pipeline actually start working?
//! IMPLEMENT `src/pipeline.rs`: `Iterator for Records`, `AddAssign for Tally`,
//! `matching`, `slowest`, `process`.
//! REFACTOR (optional): rewrite the loops of m04/m07/m09 (`find_field`,
//! `parse_fields`) as iterator chains or `std::iter::from_fn`; their tests must stay
//! green: `cargo test --test m04_borrowing --test m07_errors --test m09_records`.
//! EXPLAIN afterward: why `process` takes `&mut dyn FnMut(ParseError)` for errors
//! instead of returning a `Vec<ParseError>`.

mod common;

use common::{borrows_from, guess};
use loglens::pipeline::{Records, matching, process, slowest};
use loglens::{ErrorKind, Filter, ParseError, Record, Sink, Tally};
use std::io;

const TEXT: &str = concat!(
    "# loglens test input\n",                                       // 1
    "2026-10-23T17:03:00Z INFO auth user=ada latency_ms=42\n",      // 2
    "\n",                                                           // 3
    "2026-10-23T17:03:02Z WARN auth user=bob latency_ms=310\n",     // 4
    "   # an indented comment\n",                                   // 5
    "garbage line\n",                                               // 6
    "2026-10-23T17:03:05Z ERROR db latency_ms=310 msg=\"reset\"\n", // 7
    "2026-10-23T17:03:06Z INFO db latency_ms=fast\n",               // 8
    "2026-10-23T17:03:07Z INFO auth user=ada msg=\"unclosed\n",     // 9
    "2026-10-23T17:03:08Z DEBUG cache hit=true\r\n",                // 10
);

fn timestamps<'a>(records: impl IntoIterator<Item = Record<'a>>) -> Vec<&'a str> {
    records.into_iter().map(|r| r.timestamp).collect()
}

#[test]
fn a_records_yields_records_and_numbered_errors() {
    let items: Vec<_> = Records::new(TEXT).collect();
    assert_eq!(items.len(), 7);
    let oks: Vec<&str> = items
        .iter()
        .filter_map(|i| i.as_ref().ok())
        .map(|r| r.target)
        .collect();
    assert_eq!(oks, ["auth", "auth", "db", "db", "cache"]);
    let errors: Vec<(Option<u64>, ErrorKind)> = items
        .iter()
        .filter_map(|i| i.as_ref().err())
        .map(|e| (e.line, e.kind))
        .collect();
    assert_eq!(
        errors,
        [
            (Some(6), ErrorKind::BadTimestamp),
            (Some(9), ErrorKind::UnterminatedQuote)
        ]
    );
    assert_eq!(Records::new("").count(), 0);
    assert_eq!(Records::new("# only\n\n  \n").count(), 0);
}

#[test]
fn a_records_borrow_from_the_text() {
    let text = TEXT.to_string();
    let records: Vec<Record<'_>> = Records::new(&text).filter_map(Result::ok).collect();
    // The iterator is gone; the records still borrow `text`.
    assert_eq!(records.len(), 5);
    for r in &records {
        assert!(borrows_from(&text, r.timestamp) && borrows_from(&text, r.target));
    }
}

#[test]
fn b_tally_add_assign() {
    let mut t = Tally {
        records: 1,
        matched: 2,
        malformed: 3,
    };
    t += Tally {
        records: 10,
        matched: 20,
        malformed: 30,
    };
    assert_eq!(
        t,
        Tally {
            records: 11,
            matched: 22,
            malformed: 33
        }
    );
}

#[test]
fn c_matching() {
    let auth = Filter::parse("target=auth").unwrap();
    assert_eq!(
        timestamps(matching(TEXT, &auth)),
        ["2026-10-23T17:03:00Z", "2026-10-23T17:03:02Z"]
    );
    let all = Filter::all();
    assert_eq!(matching(TEXT, &all).count(), 5);
    assert_eq!(matching("", &all).count(), 0);
}

#[test]
fn d_slowest_returns_the_first_maximum() {
    let r = slowest(TEXT, "latency_ms").unwrap();
    assert_eq!(
        r.timestamp, "2026-10-23T17:03:02Z",
        "line 4 and line 7 tie at 310"
    );
    assert!(slowest(TEXT, "nope").is_none());
    assert!(slowest("", "latency_ms").is_none());
}

#[test]
fn e_process_counts_filters_and_reports() {
    let filter = Filter::parse("level>=info").unwrap();
    let mut lines: Vec<String> = Vec::new();
    let mut errors = Vec::new();
    let tally = process(TEXT, &filter, &mut lines, &mut |e| errors.push(e)).unwrap();
    assert_eq!(
        tally,
        Tally {
            records: 5,
            matched: 4,
            malformed: 2
        }
    );
    assert_eq!(lines.len(), 4);
    assert_eq!(lines[3], "2026-10-23T17:03:06Z INFO db latency_ms=fast");
    assert_eq!(
        errors,
        [
            ParseError::new(ErrorKind::BadTimestamp, 1).at_line(6),
            ParseError::new(ErrorKind::UnterminatedQuote, 41).at_line(9)
        ]
    );
}

/// Fails on the second record; records whether `finish` was called.
#[derive(Default)]
struct Picky {
    accepted: u32,
    finished: bool,
}

impl Sink for Picky {
    fn accept(&mut self, _: &Record<'_>) -> io::Result<()> {
        self.accepted += 1;
        if self.accepted == 2 {
            return Err(io::Error::other("disk full"));
        }
        Ok(())
    }
    fn finish(&mut self) -> io::Result<()> {
        self.finished = true;
        Ok(())
    }
}

#[test]
fn e_process_stops_at_a_sink_error_and_never_finishes_the_sink() {
    let mut sink = Picky::default();
    let err = process(TEXT, &Filter::all(), &mut sink, &mut |_| {}).unwrap_err();
    assert_eq!(err.to_string(), "disk full");
    assert_eq!(sink.accepted, 2, "processing stopped at the failing record");
    assert!(!sink.finished, "finishing is the caller's job");

    let mut sink = Picky::default();
    let text = "2026-10-23T17:03:00Z INFO auth\n";
    process(text, &Filter::all(), &mut sink, &mut |_| {}).unwrap();
    assert!(!sink.finished);
}

/// P. Predict BEFORE running: replace each `guess()` with your answer.
#[test]
fn p_predictions() {
    // P1. Which index does max_by_key return when two elements tie?
    let v = [1, 5, 5, 2];
    let p1: usize = guess();
    assert_eq!(
        p1,
        v.iter().enumerate().max_by_key(|&(_, x)| x).unwrap().0,
        "P1: max_by_key keeps the last maximum"
    );

    // P2. How many elements does `inspect` see?
    let mut seen = 0;
    let evens: Vec<i32> = (1..100)
        .inspect(|_| seen += 1)
        .filter(|n| n % 2 == 0)
        .take(2)
        .collect();
    assert_eq!(evens, [2, 4]);
    let p2: i32 = guess();
    assert_eq!(p2, seen, "P2: adapters are lazy; take stops pulling");

    // P3. Lines versus split: how many pieces?
    let text = "a\r\nb\n\nc\n";
    let p3: (usize, usize) = guess();
    assert_eq!(
        p3,
        (text.lines().count(), text.split('\n').count()),
        "P3: lines drops the final empty piece"
    );
}
