//! MILESTONE m09 — The `Level` enum and the `Record` struct (Chapters 9–10)
//! Run: cargo test --test m09_records
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider: which order does `#[derive(PartialOrd)]`
//! give an enum? What can a slice pattern express that `split` and `parse` cannot?
//! IMPLEMENT `src/level.rs`: `from_name`, `as_str`; `src/record.rs`: `is_timestamp`,
//! `parse_record`, `Record::get`, `Record::number`.
//! EXPLAIN afterward: why does `Record` have a lifetime parameter, and what would a
//! `Record` that owned its strings cost per line?

mod common;

use common::{borrows_from, guess};
use loglens::num::NumError;
use loglens::record::is_timestamp;
use loglens::{ErrorKind, Level, ParseError, Record, parse_record};

const LOGIN: &str = r#"2026-10-23T17:03:00Z INFO auth user=ada latency_ms=42 msg="login ok""#;

#[test]
fn a_level_from_name() {
    let cases = [
        ("TRACE", Some(Level::Trace)),
        ("debug", Some(Level::Debug)),
        ("Info", Some(Level::Info)),
        ("WARN", Some(Level::Warn)),
        ("warning", Some(Level::Warn)),
        ("eRrOr", Some(Level::Error)),
        ("", None),
        (" INFO", None),
        ("INF", None),
        ("ERR", None),
        ("İNFO", None),
    ];
    for (name, level) in cases {
        assert_eq!(Level::from_name(name), level, "name {name:?}");
    }
}

#[test]
fn b_level_names_round_trip() {
    let names: Vec<&str> = Level::ALL.iter().map(|l| l.as_str()).collect();
    assert_eq!(names, ["TRACE", "DEBUG", "INFO", "WARN", "ERROR"]);
    for level in Level::ALL {
        assert_eq!(Level::from_name(level.as_str()), Some(level));
    }
}

#[test]
fn b_levels_are_ordered_by_severity() {
    assert!(Level::ALL.is_sorted());
    assert!(Level::Warn > Level::Info);
    assert_eq!(Level::ALL.iter().max(), Some(&Level::Error));
}

#[test]
fn c_is_timestamp() {
    let valid = [
        "2026-10-23T17:03:00Z",
        "1999-12-31T23:59:60Z",
        "2026-02-31T00:00:00Z",
        "0000-01-01T00:00:00Z",
    ];
    for s in valid {
        assert!(is_timestamp(s), "{s:?} is valid");
    }
    let invalid = [
        "",
        "2026-10-23",
        "2026-10-23 17:03:00Z",
        "2026-10-23t17:03:00Z",
        "2026-10-23T17:03:00",
        "2026-10-23T17:03:00Z ",
        "2026-10-23T17:03:00.5Z",
        "2026-13-01T00:00:00Z",
        "2026-00-10T00:00:00Z",
        "2026-10-00T00:00:00Z",
        "2026-10-32T00:00:00Z",
        "2026-10-23T24:00:00Z",
        "2026-10-23T17:60:00Z",
        "2026-10-23T17:03:61Z",
        "2026-1a-23T17:03:00Z",
        "２026-10-23T17:03:00Z",
    ];
    for s in invalid {
        assert!(!is_timestamp(s), "{s:?} is invalid");
    }
}

#[test]
fn d_parse_record() {
    let r = parse_record(LOGIN).unwrap();
    assert_eq!(r.timestamp, "2026-10-23T17:03:00Z");
    assert_eq!(r.level, Level::Info);
    assert_eq!(r.target, "auth");
    let fields: Vec<(&str, &str, bool)> =
        r.fields.iter().map(|f| (f.key, f.raw, f.quoted)).collect();
    assert_eq!(
        fields,
        [
            ("user", "ada", false),
            ("latency_ms", "42", false),
            ("msg", "login ok", true)
        ]
    );

    let r = parse_record("  2026-10-23T17:03:00Z\twarning   db  \r").unwrap();
    assert_eq!((r.level, r.target, r.fields.len()), (Level::Warn, "db", 0));
}

#[test]
fn d_parse_record_borrows_everything() {
    let line = LOGIN.to_string();
    let r = parse_record(&line).unwrap();
    assert!(borrows_from(&line, r.timestamp) && borrows_from(&line, r.target));
    for f in &r.fields {
        assert!(borrows_from(&line, f.key) && borrows_from(&line, f.raw));
    }
}

#[test]
fn d_parse_record_errors_point_at_the_problem() {
    use ErrorKind::*;
    let cases = [
        ("", Empty, 1),
        ("   ", Empty, 1),
        ("yesterday INFO auth", BadTimestamp, 1),
        ("  2026-10-23 INFO auth", BadTimestamp, 3),
        ("2026-10-23T17:03:00Z", MissingLevel, 21),
        ("2026-10-23T17:03:00Z   ", MissingLevel, 24),
        ("2026-10-23T17:03:00Z LOUD auth", UnknownLevel, 22),
        ("2026-10-23T17:03:00Z INFO", MissingTarget, 26),
        ("2026-10-23T17:03:00Z INFO user=ada", MissingTarget, 27),
        (
            r#"2026-10-23T17:03:00Z INFO auth msg="oops"#,
            UnterminatedQuote,
            32,
        ),
        (
            "2026-10-23T17:03:00Z INFO auth ok=1 broken",
            MissingEquals,
            37,
        ),
        ("2026-10-23T17:03:00Z INFO ünïcode x", MissingEquals, 35),
    ];
    for (line, kind, column) in cases {
        assert_eq!(
            parse_record(line),
            Err(ParseError::new(kind, column)),
            "line {line:?}"
        );
    }
}

#[test]
fn e_get_returns_the_first_raw_value() {
    let r = parse_record("2026-10-23T17:03:00Z INFO auth user=ada user=bob msg=\"a b\"").unwrap();
    assert_eq!(r.get("user"), Some("ada"));
    assert_eq!(r.get("msg"), Some("a b"));
    assert_eq!(r.get("nope"), None);
}

#[test]
fn e_values_outlive_the_record() {
    let line = LOGIN.to_string();
    let user = {
        let record: Record<'_> = parse_record(&line).unwrap();
        record.get("user")
    };
    assert_eq!(user, Some("ada"));
}

#[test]
fn f_number() {
    let r = parse_record(
        r#"2026-10-23T17:03:00Z INFO auth latency_ms=42 speed=fast quoted="7" big=99999999999999999999"#,
    )
    .unwrap();
    assert_eq!(r.number("latency_ms"), Ok(Some(42)));
    assert_eq!(r.number("missing"), Ok(None));
    assert_eq!(r.number("speed"), Err(NumError::InvalidDigit { index: 0 }));
    assert_eq!(r.number("quoted"), Ok(Some(7)));
    assert_eq!(r.number("big"), Ok(Some(u64::MAX)), "saturates");
}

/// P. Predict BEFORE running: replace each `guess()` with your answer.
/// The sizes assume a 64-bit target.
#[test]
fn p_predictions() {
    // P1. An enum variant cast to its discriminant.
    let p1: u8 = guess();
    assert_eq!(p1, Level::Warn as u8, "P1: discriminants count from 0");

    // P2. How many levels are at least Info?
    let p2: usize = guess();
    assert_eq!(
        p2,
        Level::ALL.iter().filter(|&&l| l >= Level::Info).count(),
        "P2: derived Ord follows declaration order"
    );

    // P3. The size of Option<Level>.
    let p3: usize = guess();
    assert_eq!(
        p3,
        size_of::<Option<Level>>(),
        "P3: None uses a spare bit pattern"
    );

    // P4. The size of Record: two &str, a Vec, and a Level.
    let p4: usize = guess();
    assert_eq!(
        p4,
        size_of::<Record>(),
        "P4: padding to the largest alignment"
    );
}
