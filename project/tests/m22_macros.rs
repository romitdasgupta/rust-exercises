//! MILESTONE m22 — `field!` and `log_line!` (Chapter 22)
//! Run: cargo test --test m22_macros
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider: what does a macro see: values, types,
//! or tokens? What could go wrong if the expansion said `quote(...)` instead of
//! `$crate::text::quote(...)`?
//! IMPLEMENT `src/macros.rs`: the expansions of `field!` and `log_line!`.
//! REFACTOR (optional): build the log lines of your own new tests with `log_line!`.
//! Write out by hand, in your notes, what one `log_line!` call expands to.
//! EXPLAIN afterward: hygiene, `$crate`, and why each argument must be evaluated once.

mod common;

use common::guess;
use loglens::{field, log_line, parse_record};

const TS: &str = "2026-10-23T17:03:00Z";

#[test]
fn a_field() {
    let line = r#"2026-10-23T17:03:00Z INFO http user=ada latency_ms=42 content-type="text/plain""#;
    let rec = parse_record(line).unwrap();
    assert_eq!(field!(rec, user), Some("ada"));
    assert_eq!(field!(&rec, user), Some("ada"), "a reference works too");
    assert_eq!(field!(rec, "content-type"), Some("text/plain"));
    assert_eq!(field!(rec, missing), None);
    assert_eq!(field!(rec, latency_ms as u64), Some(42));
    assert_eq!(field!(rec, user as u64), None, "not a number");
    assert_eq!(field!(rec, nothing as u64), None);
}

#[test]
fn a_field_evaluates_the_record_once() {
    let rec = parse_record("2026-10-23T17:03:00Z INFO auth user=ada n=7").unwrap();
    let mut calls = 0;
    let user = field!(
        {
            calls += 1;
            &rec
        },
        user
    );
    let n = field!(
        {
            calls += 1;
            &rec
        },
        n as u64
    );
    assert_eq!((user, n, calls), (Some("ada"), Some(7), 2));
}

#[test]
fn b_log_line() {
    assert_eq!(
        log_line!(TS, Info, "auth", user = "ada", latency_ms = 42),
        "2026-10-23T17:03:00Z INFO auth user=ada latency_ms=42"
    );
    assert_eq!(log_line!(TS, Error, "db"), "2026-10-23T17:03:00Z ERROR db");
    assert_eq!(
        log_line!(TS, Warn, "auth", msg = "login failed", ok = false,),
        r#"2026-10-23T17:03:00Z WARN auth msg="login failed" ok=false"#
    );
    let target = String::from("billing");
    assert_eq!(
        log_line!(TS, Debug, target, amount = 40 + 2, note = r#"say "hi""#),
        r#"2026-10-23T17:03:00Z DEBUG billing amount=42 note="say \"hi\"""#
    );
}

#[test]
fn b_log_line_round_trips() {
    let line = log_line!(
        TS,
        Trace,
        "cache",
        key = "a b",
        empty = "",
        path = r"C:\temp"
    );
    let r = parse_record(&line).unwrap();
    assert_eq!(r.text("key").as_deref(), Some("a b"));
    assert_eq!(r.text("empty").as_deref(), Some(""));
    assert_eq!(r.text("path").as_deref(), Some(r"C:\temp"));
}

mod hygiene {
    //! Names here look like the ones the macro uses internally.
    use loglens::log_line;

    #[allow(dead_code)]
    fn quote(_: &str) -> &'static str {
        "WRONG"
    }

    #[allow(dead_code)]
    mod text {
        pub fn quote(_: &str) -> &'static str {
            "WRONG"
        }
    }

    #[allow(dead_code)]
    enum Level {
        Info,
    }

    #[test]
    fn b_log_line_is_hygienic() {
        let line = "caller's";
        let value = 7;
        let out = String::from("not the output");
        assert_eq!(
            log_line!(super::TS, Info, "auth", who = line, n = value, s = out),
            "2026-10-23T17:03:00Z INFO auth who=caller's n=7 s=\"not the output\""
        );
    }
}

#[test]
fn b_log_line_evaluates_each_argument_once() {
    let mut calls = 0;
    let mut next = || {
        calls += 1;
        calls
    };
    let line = log_line!(TS, Info, format!("t{}", next()), a = next(), b = next());
    assert_eq!(line, "2026-10-23T17:03:00Z INFO t1 a=2 b=3");
    assert_eq!(calls, 3);
}

/// P. Predict BEFORE running: replace each `guess()` with your answer.
#[test]
fn p_predictions() {
    // P1. stringify! of an expression.
    let p1: &str = guess();
    assert_eq!(p1, stringify!(1 + 2), "P1: tokens, not values");

    // P2. A recursive macro that counts token trees.
    macro_rules! count {
        () => { 0usize };
        ($head:tt $($rest:tt)*) => { 1usize + count!($($rest)*) };
    }
    let p2: usize = guess();
    assert_eq!(
        p2,
        count!(a (b c) [d] e),
        "P2: a group in brackets is one token tree"
    );

    // P3. concat! of literals.
    let p3: &str = guess();
    assert_eq!(
        p3,
        concat!("a", 1, true, 'c'),
        "P3: concat! joins literals at compile time"
    );
}
