//! MILESTONE m07 — A real error type with positions (Chapters 6–7)
//! Run: cargo test --test m07_errors   (then re-run: cargo test --test m04_borrowing)
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider: `split_field` returns `None` for five
//! different problems. Which ones would a user need told apart?
//! IMPLEMENT `src/scan.rs`: `scan_field`, `column_of`, `parse_fields`; `src/error.rs`:
//! `Display` for `ErrorKind` and `ParseError`.
//! REFACTOR `split_field` into `scan_field(s).ok()`. The m04 tests are your safety net.
//! EXPLAIN afterward: when do you return `Option`, when `Result`, and when do you panic?

mod common;

use common::{borrows_from, guess};
use loglens::scan::{Field, column_of, parse_fields, scan_field, split_field};
use loglens::{ErrorKind, ParseError};

#[test]
fn a_scan_field_accepts_what_split_field_accepts() {
    let (f, rest) = scan_field(r#"  msg="a \"b\"" user=ada"#).unwrap();
    assert_eq!(
        f,
        Field {
            key: "msg",
            raw: r#"a \"b\""#,
            quoted: true
        }
    );
    assert_eq!(rest, "user=ada");
    let (f, rest) = scan_field("empty=").unwrap();
    assert_eq!((f.key, f.raw, f.quoted, rest), ("empty", "", false, ""));
}

#[test]
fn a_scan_field_says_what_is_wrong() {
    use ErrorKind::*;
    let cases = [
        ("", Empty),
        ("  \t", Empty),
        ("user", MissingEquals),
        ("user ada=1", MissingEquals),
        ("=x", EmptyKey),
        ("= x", EmptyKey),
        (r#"msg="open"#, UnterminatedQuote),
        (r#"msg="a\""#, UnterminatedQuote),
        (r#"msg="a"b"#, TextAfterQuote),
        (r#"msg="a""b""#, TextAfterQuote),
    ];
    for (input, kind) in cases {
        assert_eq!(scan_field(input).map(|_| ()), Err(kind), "input {input:?}");
    }
}

/// The refactoring safety net: one scanner, two interfaces.
#[test]
fn a_split_field_agrees_with_scan_field() {
    let inputs = [
        "user=ada x=1",
        r#"msg="a b" c=d"#,
        r#"path="C:\\""#,
        "",
        "user",
        "=x",
        r#"msg="open"#,
        r#"msg="a"b"#,
        "é=ü",
    ];
    for input in inputs {
        assert_eq!(
            split_field(input),
            scan_field(input).ok(),
            "input {input:?}"
        );
    }
}

#[test]
fn b_column_of_counts_characters() {
    let line = "é x=1";
    assert_eq!(column_of(line, line), 1);
    assert_eq!(
        column_of(line, &line[3..]),
        3,
        "é is one character, two bytes"
    );
    assert_eq!(column_of(line, &line[line.len()..]), 6, "one past the end");
    let ascii = "2026-10-23T17:03:00Z INFO auth";
    assert_eq!(column_of(ascii, &ascii[21..]), 22);
}

#[test]
fn c_parse_fields() {
    let line = r#"2026-10-23T17:03:00Z INFO auth user=ada msg="hi there""#;
    let rest = &line[line.find("user").unwrap()..];
    let fields = parse_fields(line, rest).unwrap();
    assert_eq!(fields.len(), 2);
    assert_eq!((fields[0].key, fields[0].raw), ("user", "ada"));
    assert_eq!((fields[1].key, fields[1].raw), ("msg", "hi there"));
    for f in &fields {
        assert!(borrows_from(line, f.key) && borrows_from(line, f.raw));
    }
    assert_eq!(parse_fields(line, &line[line.len()..]), Ok(vec![]));
    let padded = "x   ";
    assert_eq!(parse_fields(padded, &padded[1..]), Ok(vec![]));
}

#[test]
fn c_parse_fields_reports_the_failing_field() {
    let line = r#"2026-10-23T17:03:00Z INFO auth user=ada broken msg="x""#;
    let rest = &line[line.find("user").unwrap()..];
    let column = line.find("broken").unwrap() + 1;
    assert_eq!(
        parse_fields(line, rest),
        Err(ParseError::new(ErrorKind::MissingEquals, column))
    );

    // Columns count characters: "café" has 4 characters and 5 bytes.
    let line = r#"2026-10-23T17:03:00Z INFO auth note=café  msg="unclosed"#;
    let rest = &line[line.find("note").unwrap()..];
    assert_eq!(
        parse_fields(line, rest),
        Err(ParseError::new(ErrorKind::UnterminatedQuote, 43))
    );
}

#[test]
fn d_error_kind_messages() {
    use ErrorKind::*;
    let expected = [
        (Empty, "empty line"),
        (BadTimestamp, "invalid timestamp"),
        (MissingLevel, "missing level"),
        (UnknownLevel, "unknown level"),
        (MissingTarget, "missing target"),
        (MissingEquals, "expected key=value"),
        (EmptyKey, "empty key"),
        (UnterminatedQuote, "unterminated quoted value"),
        (TextAfterQuote, "unexpected text after closing quote"),
        (InvalidUtf8, "line is not valid UTF-8"),
    ];
    for (kind, message) in expected {
        assert_eq!(kind.to_string(), message);
    }
}

#[test]
fn e_parse_error_display() {
    let e = ParseError::new(ErrorKind::UnterminatedQuote, 17);
    assert_eq!(e.to_string(), "column 17: unterminated quoted value");
    let e = e.at_line(3);
    assert_eq!(
        e.to_string(),
        "line 3, column 17: unterminated quoted value"
    );
    let boxed: Box<dyn std::error::Error> = Box::new(e);
    assert_eq!(
        boxed.to_string(),
        "line 3, column 17: unterminated quoted value"
    );
}

/// P. Predict BEFORE running: replace each `guess()` with your answer.
#[test]
fn p_predictions() {
    // P1. Does "256" parse as a u8?
    let p1: bool = guess();
    assert_eq!(
        p1,
        "256".parse::<u8>().is_ok(),
        "P1: parse checks the range"
    );

    // P2. How many times does the closure run?
    let mut calls = 0;
    let _ = Some(1).ok_or_else(|| {
        calls += 1;
        "missing"
    });
    let p2: i32 = guess();
    assert_eq!(p2, calls, "P2: ok_or_else is lazy");

    // P3. Collecting an iterator of Results into one Result.
    let collected: Result<Vec<i32>, String> = ["1", "x", "3", "y"]
        .iter()
        .map(|s| s.parse::<i32>().map_err(|_| s.to_string()))
        .collect();
    let p3: Result<Vec<i32>, String> = guess();
    assert_eq!(p3, collected, "P3: collect stops at the first Err");
}
