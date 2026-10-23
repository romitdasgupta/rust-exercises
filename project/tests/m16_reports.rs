//! MILESTONE m16 — Aggregation with maps and a bounded heap (Chapters 12 and 16)
//! Run: cargo test --test m16_reports
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider: which collection gives a sorted report
//! for free, which gives the fastest lookup, and what does top-k cost with each?
//! IMPLEMENT `src/report.rs`: `Group::key_of`, `From<&str> for Group`, `GroupStats`
//! (`mean_latency`, `+=`), `Summary::add`, `Summary::top`, `+=` for `Summary`,
//! `Sink for Summary`, and `render`.
//! EXPLAIN afterward: why does `add` look the group up by `&str` before inserting a
//! `String`? Why must merging summaries give exactly the sequential result?

mod common;

use common::{count, fixture_text, guess};
use loglens::pipeline::{Records, process};
use loglens::report::{GroupStats, render};
use loglens::{Filter, Group, Level, Record, Summary, parse_record};
use std::collections::{BTreeMap, BinaryHeap, HashMap};

fn summarize(text: &str, group: Group) -> Summary {
    let mut summary = Summary::new(group);
    for r in Records::new(text).filter_map(Result::ok) {
        summary.add(&r);
    }
    summary
}

fn stats(count: u64, errors: u64, latencies: &[u64]) -> GroupStats {
    GroupStats {
        count,
        errors,
        latency_sum: latencies.iter().sum(),
        latency_count: latencies.len() as u64,
        latency_max: latencies.iter().copied().max(),
    }
}

#[test]
fn a_key_of() {
    let line = "2026-10-23T17:03:00Z WARN auth user=ada".to_string();
    let r = parse_record(&line).unwrap();
    assert_eq!(Group::Target.key_of(&r), "auth");
    assert_eq!(Group::Level.key_of(&r), "WARN");
    assert_eq!(Group::Field("user".into()).key_of(&r), "ada");
    assert_eq!(Group::Field("region".into()).key_of(&r), "-");
    let key = {
        let group = Group::Target;
        group.key_of(&r)
    };
    drop(r);
    assert_eq!(
        key, "auth",
        "the name borrows the line, not the record or group"
    );
}

#[test]
fn b_group_from_str() {
    assert_eq!(Group::from("target"), Group::Target);
    assert_eq!(Group::from("level"), Group::Level);
    assert_eq!(Group::from("user"), Group::Field("user".into()));
    assert_eq!(Group::from("Target"), Group::Field("Target".into()));
    assert_eq!(Group::Field("user".into()).to_string(), "user");
}

#[test]
fn c_mean_latency() {
    assert_eq!(stats(3, 0, &[]).mean_latency(), None);
    assert_eq!(stats(2, 0, &[10, 15]).mean_latency(), Some(12.5));
}

#[test]
fn d_group_stats_merge() {
    let mut a = stats(2, 1, &[10, 30]);
    a += &stats(1, 0, &[20]);
    assert_eq!(a, stats(3, 1, &[10, 30, 20]));
    let mut b = stats(1, 0, &[]);
    b += &stats(1, 1, &[]);
    assert_eq!(b, stats(2, 1, &[]));
    let mut c = GroupStats {
        latency_sum: u64::MAX - 1,
        ..stats(1, 0, &[5])
    };
    c += &stats(1, 0, &[5]);
    assert_eq!(c.latency_sum, u64::MAX, "sums saturate");
}

#[test]
fn e_add_counts_levels_groups_and_latency() {
    let s = summarize(&fixture_text("app.log"), Group::Target);
    assert_eq!(s.records, 8);
    let levels: Vec<(Level, u64)> = s.levels.iter().map(|(&l, &n)| (l, n)).collect();
    assert_eq!(
        levels,
        [
            (Level::Debug, 1),
            (Level::Info, 4),
            (Level::Warn, 1),
            (Level::Error, 2)
        ],
        "only levels that occurred, in severity order"
    );
    let expected: HashMap<String, GroupStats> = [
        ("auth", stats(4, 1, &[42, 310, 97, 39])),
        ("db", stats(2, 1, &[4, 5012])),
        ("cache", stats(1, 0, &[])),
        ("billing", stats(1, 0, &[])),
    ]
    .into_iter()
    .map(|(k, v)| (k.to_string(), v))
    .collect();
    assert_eq!(s.groups, expected);
}

#[test]
fn e_add_ignores_bad_latency_and_groups_missing_fields() {
    let text = "2026-10-23T17:03:00Z INFO a latency_ms=fast\n\
                2026-10-23T17:03:00Z INFO b user=ada latency_ms=5\n";
    let s = summarize(text, Group::Field("user".into()));
    assert_eq!(s.groups["-"], stats(1, 0, &[]));
    assert_eq!(s.groups["ada"], stats(1, 0, &[5]));
}

#[test]
fn e_add_allocates_only_for_new_groups() {
    let lines = [
        "2026-10-23T17:03:00Z INFO auth latency_ms=42",
        "2026-10-23T17:03:01Z ERROR db latency_ms=7",
        "2026-10-23T17:03:02Z WARN cache",
    ];
    let records: Vec<Record<'_>> = lines
        .iter()
        .cycle()
        .take(3000)
        .map(|l| parse_record(l).unwrap())
        .collect();
    let mut summary = Summary::new(Group::Target);
    let ((), allocations) = count(|| {
        for r in &records {
            summary.add(r);
        }
    });
    assert_eq!(summary.records, 3000);
    assert!(
        allocations <= 16,
        "3000 records in 3 groups made {allocations} allocations"
    );
}

#[test]
fn f_top_orders_by_count_then_name() {
    let mut s = Summary::new(Group::Target);
    for (name, n) in [
        ("db", 2),
        ("auth", 4),
        ("zeta", 2),
        ("cache", 1),
        ("billing", 1),
    ] {
        s.groups.insert(name.to_string(), stats(n, 0, &[]));
    }
    let names = |k| s.top(k).into_iter().map(|(n, _)| n).collect::<Vec<_>>();
    assert_eq!(names(3), ["auth", "db", "zeta"]);
    assert_eq!(names(100), ["auth", "db", "zeta", "billing", "cache"]);
    assert_eq!(names(0), Vec::<&str>::new());
    let top = s.top(1);
    assert!(
        std::ptr::eq(top[0].1, &s.groups["auth"]),
        "borrowed, not cloned"
    );
}

#[test]
fn f_top_agrees_with_a_full_sort() {
    let mut s = Summary::new(Group::Target);
    for i in 0..1000u64 {
        s.groups
            .insert(format!("g{i:04}"), stats(i * 7919 % 37, 0, &[]));
    }
    let mut all: Vec<(&str, u64)> = s
        .groups
        .iter()
        .map(|(k, v)| (k.as_str(), v.count))
        .collect();
    all.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(b.0)));
    for k in [1, 10, 37, 999, 1000, 1001] {
        let top: Vec<(&str, u64)> = s.top(k).into_iter().map(|(n, g)| (n, g.count)).collect();
        assert_eq!(top, all[..k.min(1000)], "k = {k}");
    }
}

#[test]
fn g_merged_summaries_equal_the_sequential_summary() {
    let text = fixture_text("app.log") + &fixture_text("web.log");
    let lines: Vec<&str> = text.lines().collect();
    for group in [Group::Target, Group::Level, Group::Field("user".into())] {
        let whole = summarize(&text, group.clone());
        for split in [0, 1, 5, lines.len()] {
            let mut left = summarize(&lines[..split].join("\n"), group.clone());
            left += summarize(&lines[split..].join("\n"), group.clone());
            assert_eq!(left, whole, "split at {split}, group {group}");
        }
    }
}

#[test]
fn h_summary_is_a_sink() {
    let text = fixture_text("app.log");
    let mut s = Summary::new(Group::Target);
    let filter = Filter::parse("level>=info").unwrap();
    process(&text, &filter, &mut s, &mut |_| {}).unwrap();
    assert_eq!(s.records, 7, "the DEBUG record was filtered out");
    assert!(!s.groups.contains_key("cache"));
}

#[test]
fn i_render() {
    let s = summarize(&fixture_text("app.log"), Group::Target);
    assert_eq!(
        render(&s, 10),
        "records: 8\n\
         levels: TRACE=0 DEBUG=1 INFO=4 WARN=1 ERROR=2\n\
         top 10 by target:\n\
         \x20 auth    count=4 errors=1 mean_latency_ms=122.0 max_latency_ms=310\n\
         \x20 db      count=2 errors=1 mean_latency_ms=2508.0 max_latency_ms=5012\n\
         \x20 billing count=1 errors=0 mean_latency_ms=- max_latency_ms=-\n\
         \x20 cache   count=1 errors=0 mean_latency_ms=- max_latency_ms=-\n"
    );
    assert_eq!(
        render(&s, 1),
        "records: 8\n\
         levels: TRACE=0 DEBUG=1 INFO=4 WARN=1 ERROR=2\n\
         top 1 by target:\n\
         \x20 auth count=4 errors=1 mean_latency_ms=122.0 max_latency_ms=310\n"
    );
    let by_user = summarize(&fixture_text("app.log"), Group::Field("user".into()));
    assert_eq!(
        render(&by_user, 3),
        "records: 8\n\
         levels: TRACE=0 DEBUG=1 INFO=4 WARN=1 ERROR=2\n\
         top 3 by user:\n\
         \x20 -   count=3 errors=1 mean_latency_ms=2508.0 max_latency_ms=5012\n\
         \x20 ada count=2 errors=0 mean_latency_ms=40.5 max_latency_ms=42\n\
         \x20 bob count=2 errors=1 mean_latency_ms=203.5 max_latency_ms=310\n"
    );
    assert_eq!(
        render(&Summary::new(Group::Level), 5),
        "records: 0\n\
         levels: TRACE=0 DEBUG=0 INFO=0 WARN=0 ERROR=0\n\
         top 5 by level:\n\
         \x20 (none)\n"
    );
}

/// P. Predict BEFORE running: replace each `guess()` with your answer.
#[test]
fn p_predictions() {
    // P1. Iteration order of a BTreeMap.
    let mut m = BTreeMap::new();
    for k in ["db", "auth", "cache"] {
        m.insert(k, ());
    }
    let p1: Vec<&str> = guess();
    assert_eq!(
        p1,
        m.keys().copied().collect::<Vec<_>>(),
        "P1: sorted by key"
    );

    // P2. A BinaryHeap of Reverse values: what pops first?
    use std::cmp::Reverse;
    let mut heap = BinaryHeap::from(vec![Reverse(3), Reverse(1), Reverse(4)]);
    let p2: i32 = guess();
    assert_eq!(
        p2,
        heap.pop().unwrap().0,
        "P2: Reverse turns a max-heap into a min-heap"
    );

    // P3. The entry API: which default wins?
    let mut counts = HashMap::new();
    *counts.entry("a").or_insert(0) += 1;
    *counts.entry("a").or_insert(10) += 1;
    let p3: i32 = guess();
    assert_eq!(p3, counts["a"], "P3: or_insert only inserts when absent");

    // P4. The length of a vector created with capacity 10.
    let v: Vec<u8> = Vec::with_capacity(10);
    let p4: usize = guess();
    assert_eq!(p4, v.len(), "P4: capacity is not length");
}
