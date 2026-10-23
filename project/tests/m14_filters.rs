//! MILESTONE m14 — A filter language compiled to closures (Chapter 14)
//! Run: cargo test --test m14_filters
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider: what must each closure own so that a
//! `Filter` outlives the string it was parsed from and can be shared by threads?
//! IMPLEMENT `src/filter.rs`: `Filter::parse` and `Filter::matches`.
//! EXPLAIN afterward: compare "parse once into closures" with "re-read the filter text
//! for every record". Why `Box<dyn Fn ...>` and not a generic parameter?

mod common;

use common::{count, guess};
use loglens::{Filter, FilterError, Record, parse_record};

const LINES: [&str; 5] = [
    r#"2026-10-23T17:03:00Z INFO auth user=ada latency_ms=42 msg="login ok""#,
    r#"2026-10-23T17:03:02Z WARN auth user=bob latency_ms=310 msg="slow password check""#,
    r#"2026-10-23T17:03:05Z ERROR db latency_ms=fast msg="connection reset""#,
    "2026-10-23T17:04:00Z DEBUG cache hit=true user=",
    r#"2026-10-23T17:05:00Z TRACE cache expr="a>=b""#,
];

fn records() -> Vec<Record<'static>> {
    LINES.iter().map(|l| parse_record(l).unwrap()).collect()
}

/// The 1-based numbers of the lines that match `filter`.
fn matching(filter: &str) -> Vec<usize> {
    let filter = Filter::parse(filter).unwrap();
    records()
        .iter()
        .enumerate()
        .filter(|(_, r)| filter.matches(r))
        .map(|(i, _)| i + 1)
        .collect()
}

#[test]
fn a_empty_filter_matches_everything() {
    assert_eq!(matching(""), [1, 2, 3, 4, 5]);
    assert_eq!(matching(" \t "), [1, 2, 3, 4, 5]);
    let all = Filter::all();
    assert!(records().iter().all(|r| all.matches(r)));
}

#[test]
fn a_level_terms() {
    assert_eq!(matching("level>=warn"), [2, 3]);
    assert_eq!(matching("level=info"), [1]);
    assert_eq!(matching("level!=INFO"), [2, 3, 4, 5]);
    assert_eq!(matching("level<warn"), [1, 4, 5]);
    assert_eq!(matching("level<=debug"), [4, 5]);
    assert_eq!(matching("level>Warning"), [3]);
}

#[test]
fn a_target_terms() {
    assert_eq!(matching("target=auth"), [1, 2]);
    assert_eq!(matching("target!=auth"), [3, 4, 5]);
    assert_eq!(matching("target~ach"), [4, 5]);
}

#[test]
fn a_field_terms() {
    assert_eq!(matching("user=ada"), [1]);
    assert_eq!(
        matching("user!=bob"),
        [1, 3, 4, 5],
        "a missing field is not equal"
    );
    assert_eq!(matching("user="), [4], "the empty value");
    assert_eq!(matching("msg~reset"), [3]);
    assert_eq!(
        matching("msg~"),
        [1, 2, 3],
        "contains the empty string: exists"
    );
    assert_eq!(
        matching("expr=a>=b"),
        [5],
        "the value may contain operators"
    );
}

#[test]
fn a_numeric_terms() {
    assert_eq!(matching("latency_ms>100"), [2]);
    assert_eq!(matching("latency_ms<=42"), [1]);
    assert_eq!(
        matching("latency_ms>=0"),
        [1, 2],
        "\"fast\" is not a number"
    );
    assert_eq!(matching("latency_ms<18446744073709551615"), [1, 2]);
}

#[test]
fn a_timestamp_terms() {
    assert_eq!(matching("ts>=2026-10-23T17:03:02Z"), [2, 3, 4, 5]);
    assert_eq!(matching("ts<2026-10-23T17:03:05Z"), [1, 2]);
    assert_eq!(matching("ts=2026-10-23T17:04:00Z"), [4]);
}

#[test]
fn a_terms_are_combined_with_and() {
    assert_eq!(matching("target=auth latency_ms<100"), [1]);
    assert_eq!(matching("level>=warn\ttarget=db"), [3]);
    assert_eq!(matching("  target=cache   level<info  "), [4, 5]);
    assert_eq!(matching("target=auth target=db"), Vec::<usize>::new());
}

#[test]
fn a_invalid_terms_are_rejected_at_parse_time() {
    let cases = [
        ("level", "expected KEY OP VALUE"),
        ("user!ada", "expected KEY OP VALUE"),
        ("level!", "expected KEY OP VALUE"),
        ("=ada", "empty key"),
        ("~x", "empty key"),
        ("level>=loud", "unknown level"),
        ("ts>yesterday", "expected a timestamp"),
        ("latency_ms>fast", "expected a number"),
        ("latency_ms>", "expected a number"),
        ("latency_ms<99999999999999999999", "expected a number"),
        ("level~warn", "unsupported operator"),
        ("target>a", "unsupported operator"),
        ("ts~2026", "unsupported operator"),
    ];
    for (term, reason) in cases {
        let err = Filter::parse(term).unwrap_err();
        assert_eq!(
            err,
            FilterError {
                term: term.to_string(),
                reason
            },
            "term {term:?}"
        );
    }
    let err = Filter::parse("user=ada level>=loud target=db").unwrap_err();
    assert_eq!(err.term, "level>=loud", "the whole offending term");
}

#[test]
fn b_source_is_kept() {
    assert_eq!(Filter::parse(" user=ada ").unwrap().source(), " user=ada ");
}

fn assert_shareable<T: Send + Sync + 'static>() {}

#[test]
fn b_filter_owns_what_it_compares() {
    assert_shareable::<Filter>();
    let filter = {
        let text = String::from("user=ada level>=info");
        Filter::parse(&text).unwrap()
        // `text` is dropped here.
    };
    let r = records();
    assert!(filter.matches(&r[0]));
    assert!(!filter.matches(&r[1]));
}

#[test]
fn b_matching_does_not_allocate() {
    let filter = Filter::parse(
        "level>=debug target!=billing user!=bob latency_ms<100 msg~ok ts>=2026-10-23T00:00:00Z",
    )
    .unwrap();
    let records = records();
    let (matched, allocations) = count(|| {
        let mut matched = 0;
        for _ in 0..100 {
            matched += records.iter().filter(|r| filter.matches(r)).count();
        }
        matched
    });
    assert_eq!(matched, 100, "only the first record matches");
    assert_eq!(allocations, 0, "matching must not allocate");
}

/// P. Predict BEFORE running: replace each `guess()` with your answer.
/// The sizes assume a 64-bit target.
#[test]
fn p_predictions() {
    // P1. A closure that mutates what it captured.
    let mut n = 0;
    let mut bump = || n += 1;
    bump();
    bump();
    let p1: i32 = guess();
    assert_eq!(p1, n, "P1: FnMut borrows n mutably");

    // P2. A `move` closure captures a copy of a Copy value.
    let mut x = 1;
    let plus_one = move || x + 1;
    x = 10;
    assert_eq!(x, 10);
    let p2: i32 = guess();
    assert_eq!(p2, plus_one(), "P2: the closure has its own x");

    // P3. The size of a closure that captures a u64 and a &str by move.
    let k = 5u64;
    let s = "ab";
    let c = move || k as usize + s.len();
    let p3: usize = guess();
    assert_eq!(
        p3,
        size_of_val(&c),
        "P3: a closure is a struct of its captures"
    );
    assert_eq!(c(), 7);

    // P4. The size of a boxed closure.
    let p4: usize = guess();
    assert_eq!(
        p4,
        size_of::<Box<dyn Fn(&str) -> bool>>(),
        "P4: data pointer plus vtable pointer"
    );
}
