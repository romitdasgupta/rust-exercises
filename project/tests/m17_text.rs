//! MILESTONE m17 — Quoted values and Unicode-safe truncation (Chapter 17)
//! Run: cargo test --test m17_text   (then re-run: cargo test --test m14_filters)
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider: what is the length of a string: bytes,
//! `char`s, or what a reader sees? When can `unescape` avoid allocating?
//! IMPLEMENT `src/text.rs`: `unescape`, `quote`, `truncate`; `src/scan.rs`:
//! `Field::text`; `src/record.rs`: `Record::text`.
//! REFACTOR `src/filter.rs`: compare `=`, `!=`, and `~` against the decoded text
//! (`record.text(key)`), not the raw value. `cargo test --test m14_filters` must stay
//! green, including its zero-allocation check.
//! EXPLAIN afterward: what `Cow` buys a parser, and why truncating at a byte index is
//! a bug even when it does not panic.

mod common;

use common::{borrows_from, guess};
use loglens::scan::scan_field;
use loglens::text::{quote, truncate, unescape};
use loglens::{Filter, parse_record};
use std::borrow::Cow;

macro_rules! is_borrowed {
    ($cow:expr) => {
        matches!($cow, Cow::Borrowed(_))
    };
}

#[test]
fn a_unescape() {
    let cases = [
        ("plain text", "plain text"),
        (r#"say \"hi\""#, r#"say "hi""#),
        (r"C:\\temp", r"C:\temp"),
        (r"a\nb\tc\rd", "a\nb\tc\rd"),
        (r"keep \x and \é", r"keep \x and \é"),
        (r"trailing \", r"trailing \"),
        (r#"\\\""#, r#"\""#),
        (r#"caf\"é\""#, r#"caf"é""#),
        ("", ""),
    ];
    for (raw, text) in cases {
        assert_eq!(unescape(raw), text, "raw {raw:?}");
    }
}

#[test]
fn a_unescape_borrows_when_it_can() {
    let raw = "no escapes here".to_string();
    let text = unescape(&raw);
    assert!(is_borrowed!(text));
    assert_eq!(text.as_ptr(), raw.as_ptr());
    assert!(!is_borrowed!(unescape(r"a\nb")));
}

#[test]
fn b_quote() {
    let cases = [
        ("ada", "ada"),
        ("café", "café"),
        ("", r#""""#),
        ("login ok", r#""login ok""#),
        (r#"say "hi""#, r#""say \"hi\"""#),
        ("a=b", r#""a=b""#),
        (r"C:\temp", r#""C:\\temp""#),
        ("tab\there", r#""tab\there""#),
        ("two\nlines\r", r#""two\nlines\r""#),
        ("non\u{a0}breaking", "\"non\u{a0}breaking\""),
    ];
    for (value, quoted) in cases {
        assert_eq!(quote(value), quoted, "value {value:?}");
    }
    assert!(is_borrowed!(quote("ada")));
}

#[test]
fn b_quote_round_trips_through_the_scanner() {
    let values = [
        "",
        "ada",
        "a b",
        r"\",
        r#"""#,
        r#"\""#,
        "x\n",
        "=",
        "ü ö",
        r"\\",
        r"end\",
        "\t",
        "🦀 crab",
        "k=v w=x",
    ];
    for value in values {
        let line = format!("k={} next=1", quote(value));
        let (field, rest) = scan_field(&line).unwrap();
        assert_eq!(field.text(), value, "line {line:?}");
        assert_eq!(rest, "next=1");
    }
}

#[test]
fn c_field_text() {
    let line = r#"path=C:\temp msg="a \"b\"" user=ada"#.to_string();
    let (path, rest) = scan_field(&line).unwrap();
    let path_text = path.text();
    assert_eq!(path_text, r"C:\temp", "bare values are not unescaped");
    assert!(is_borrowed!(path_text) && borrows_from(&line, &path_text));
    let (msg, rest) = scan_field(rest).unwrap();
    assert_eq!(msg.text(), r#"a "b""#);
    let (user, _) = scan_field(rest).unwrap();
    let text = {
        let copy = user;
        copy.text()
    };
    assert!(
        is_borrowed!(text),
        "lives as long as the line, not the Field"
    );
}

#[test]
fn d_record_text() {
    let line = r#"2026-10-23T17:03:00Z INFO auth msg="login \"ok\"" user=ada"#;
    let r = parse_record(line).unwrap();
    assert_eq!(r.text("msg").as_deref(), Some(r#"login "ok""#));
    let user = r.text("user").unwrap();
    assert!(is_borrowed!(user) && borrows_from(line, &user));
    assert_eq!(r.text("missing"), None);
}

#[test]
fn d_filters_compare_decoded_text() {
    let r = parse_record(
        r#"2026-10-23T17:03:00Z INFO auth msg="say \"hi\"" path="C:\\temp" user="ada""#,
    )
    .unwrap();
    for term in [r#"msg~"hi""#, r"path=C:\temp", "user=ada", "path!=x"] {
        assert!(
            Filter::parse(term).unwrap().matches(&r),
            "{term} should match"
        );
    }
    for term in [r#"msg~\"hi"#, r"path=C:\\temp", "user!=ada"] {
        assert!(
            !Filter::parse(term).unwrap().matches(&r),
            "{term} should not match"
        );
    }
}

#[test]
fn e_truncate() {
    let cases = [
        ("hello", 10, "hello"),
        ("hello", 5, "hello"),
        ("hello world", 5, "hell…"),
        ("héllo wörld", 7, "héllo …"),
        ("🦀🦀🦀", 2, "🦀…"),
        ("abc", 1, "…"),
        ("abc", 0, ""),
        ("", 0, ""),
        // A combining accent is its own char, so it can be cut off.
        ("e\u{301}x", 2, "e…"),
    ];
    for (s, max, expected) in cases {
        let out = truncate(s, max);
        assert_eq!(out, expected, "truncate({s:?}, {max})");
        assert!(out.chars().count() <= max);
    }
    assert!(is_borrowed!(truncate("fits", 4)));
    assert!(is_borrowed!(truncate("", 0)));
}

/// P. Predict BEFORE running: replace each `guess()` with your answer.
#[test]
fn p_predictions() {
    // P1. Bytes and chars of a precomposed "é".
    let p1: (usize, usize) = guess();
    assert_eq!(
        p1,
        ("é".len(), "é".chars().count()),
        "P1: one char, two bytes"
    );

    // P2. Chars in "e" followed by a combining acute accent (it looks the same).
    let p2: usize = guess();
    assert_eq!(p2, "e\u{301}".chars().count(), "P2: a char is a code point");

    // P3. Upper-casing can change the length.
    let p3: String = guess();
    assert_eq!(
        p3,
        "Straße".to_uppercase(),
        "P3: ß has no single uppercase char"
    );

    // P4. Lower-casing "İ" (capital I with dot above): how many chars?
    let p4: usize = guess();
    assert_eq!(
        p4,
        "İ".to_lowercase().chars().count(),
        "P4: i plus a combining dot"
    );
}
