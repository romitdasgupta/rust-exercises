//! MILESTONE m04 — Zero-copy fields borrowed from the line (Chapters 4–5)
//! Run: cargo test --test m04_borrowing
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider: who owns the bytes of a `Field`?
//! What would have to change if `Field` held `String`s?
//! IMPLEMENT `src/scan.rs`: `split_word`, `split_field`, `find_field`.
//! EXPLAIN afterward: draw a line in memory and the slices pointing into it. Why may
//! `find_field`'s result outlive the vector of fields but not the line?

mod common;

use common::{borrows_from, guess};
use loglens::scan::{Field, find_field, split_field, split_word};

fn field<'a>(key: &'a str, raw: &'a str, quoted: bool) -> Field<'a> {
    Field { key, raw, quoted }
}

#[test]
fn a_split_word() {
    assert_eq!(split_word("  INFO   auth x=1"), Some(("INFO", "auth x=1")));
    assert_eq!(split_word("auth"), Some(("auth", "")));
    assert_eq!(split_word("\tTRACE\r"), Some(("TRACE", "")));
    assert_eq!(split_word("héllo  wörld "), Some(("héllo", "wörld ")));
    assert_eq!(split_word(""), None);
    assert_eq!(split_word(" \t \r"), None);
}

#[test]
fn a_split_word_borrows_from_its_input() {
    let line = String::from("  2026-10-23T17:03:00Z INFO auth");
    let (word, rest) = split_word(&line).unwrap();
    assert!(borrows_from(&line, word) && borrows_from(&line, rest));
    assert_eq!(
        word.as_ptr(),
        line[2..].as_ptr(),
        "the word starts after the spaces"
    );
    assert_eq!(
        rest.as_ptr(),
        line[23..].as_ptr(),
        "the rest starts at INFO"
    );
}

#[test]
fn b_split_field_bare_and_quoted() {
    let cases = [
        (
            "user=ada latency_ms=42",
            field("user", "ada", false),
            "latency_ms=42",
        ),
        (
            r#"msg="login ok" user=ada"#,
            field("msg", "login ok", true),
            "user=ada",
        ),
        (
            r#"msg="say \"hi\"" x=1"#,
            field("msg", r#"say \"hi\""#, true),
            "x=1",
        ),
        (
            r#"path="C:\\temp\\""#,
            field("path", r#"C:\\temp\\"#, true),
            "",
        ),
        ("empty= next=1", field("empty", "", false), "next=1"),
        (r#"q="""#, field("q", "", true), ""),
        ("  \ta=b  ", field("a", "b", false), ""),
        ("a=b=c", field("a", "b=c", false), ""),
        (r#"odd=a"b"#, field("odd", r#"a"b"#, false), ""),
        (
            "ключ=значение 🦀=1",
            field("ключ", "значение", false),
            "🦀=1",
        ),
    ];
    for (input, expected, rest) in cases {
        assert_eq!(
            split_field(input),
            Some((expected, rest)),
            "input {input:?}"
        );
    }
}

#[test]
fn b_split_field_rejects_invalid_fields() {
    let cases = [
        "",
        "   ",
        "user",
        "user ada=1",
        "=x",
        r#"msg="open"#,
        r#"msg="a\""#,
        r#"msg="a"b"#,
    ];
    for input in cases {
        assert_eq!(split_field(input), None, "input {input:?}");
    }
}

#[test]
fn b_fields_borrow_from_the_line() {
    let line = String::from(r#"user=ada msg="login ok" latency_ms=42"#);
    let mut rest = line.as_str();
    let mut seen = 0;
    while let Some((f, next)) = split_field(rest) {
        assert!(borrows_from(&line, f.key) && borrows_from(&line, f.raw));
        assert!(borrows_from(&line, next));
        rest = next;
        seen += 1;
    }
    assert_eq!(seen, 3);
    let (msg, _) = split_field(&line[9..]).unwrap();
    assert_eq!(
        msg.raw.as_ptr(),
        line[14..].as_ptr(),
        "raw starts after the quote"
    );
}

#[test]
fn c_find_field() {
    let fields = [
        field("user", "ada", false),
        field("latency_ms", "42", false),
        field("user", "bob", false),
    ];
    assert_eq!(
        find_field(&fields, "user"),
        Some("ada"),
        "the first one wins"
    );
    assert_eq!(find_field(&fields, "latency_ms"), Some("42"));
    assert_eq!(find_field(&fields, "msg"), None);
    assert_eq!(find_field(&[], "user"), None);
}

#[test]
fn c_found_values_outlive_the_fields_and_the_key() {
    let line = String::from("user=ada user=bob");
    let value = {
        let key = String::from("user");
        let mut fields = Vec::new();
        let mut rest = line.as_str();
        while let Some((f, next)) = split_field(rest) {
            fields.push(f);
            rest = next;
        }
        find_field(&fields, &key)
        // `fields` and `key` are dropped here; `value` borrows only `line`.
    };
    assert_eq!(value, Some("ada"));
    assert!(borrows_from(&line, value.unwrap()));
}

/// P. Predict BEFORE running: replace each `guess()` with your answer.
/// The sizes assume a 64-bit target.
#[test]
fn p_predictions() {
    // P1. How many bytes is a `&str` (the reference, not the text)?
    let p1: usize = guess();
    assert_eq!(p1, size_of::<&str>(), "P1: a fat pointer");

    // P2. How many bytes is a `Field` (two `&str` and a `bool`)?
    let p2: usize = guess();
    assert_eq!(p2, size_of::<Field>(), "P2: fields plus alignment padding");

    // P3. How many bytes long is "naïve"?
    let p3: usize = guess();
    assert_eq!(p3, "naïve".len(), "P3: len counts UTF-8 bytes");

    // P4. Does `to_string` share the original bytes?
    let s = "auth";
    let owned = s.to_string();
    let p4: bool = guess();
    assert_eq!(p4, owned.as_ptr() == s.as_ptr(), "P4: to_string copies");
}
