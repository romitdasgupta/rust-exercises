//! MILESTONE m19 — Many files on worker threads, same results (Chapter 19)
//! Run: cargo test --test m19_parallel   (then re-run: cargo test --test m18_cli)
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider: which part of the work can run in any
//! order, and which part must happen in input order?
//! IMPLEMENT `src/parallel.rs`: `map_ordered`, `summarize_files`.
//! REFACTOR `src/cli.rs` `run`: with `--jobs` other than 1 and several FILEs, process
//! them with `map_ordered`. The m18 CLI tests and the equivalence tests below must pass.
//! EXPLAIN afterward: why scoped threads can borrow `filter` and `paths`; what would
//! break with `std::thread::spawn`; and why the merge order matters for `grep` but not
//! for `count`.

mod common;

use common::{TempDir, fixture, guess};
use loglens::cli::{Command, Input, Options, run};
use loglens::parallel::{map_ordered, summarize_files};
use loglens::pipeline::process;
use loglens::{Error, Filter, Group, Summary, Tally};
use std::collections::HashSet;
use std::io::Cursor;
use std::path::PathBuf;
use std::process::{Command as Process, Stdio};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Condvar, Mutex, mpsc};
use std::thread::{self, ThreadId};
use std::time::{Duration, Instant};

#[test]
fn a_map_ordered_keeps_input_order() {
    let items: Vec<u64> = (0..100).collect();
    let expected: Vec<u64> = items.iter().map(|x| x * x).collect();
    for jobs in [0, 1, 2, 3, 8, 200] {
        assert_eq!(
            map_ordered(&items, jobs, |x| x * x),
            expected,
            "jobs = {jobs}"
        );
    }
    assert_eq!(map_ordered(&[] as &[u64], 8, |x| *x), Vec::<u64>::new());
}

#[test]
fn a_map_ordered_borrows_its_inputs() {
    let names = vec![String::from("auth"), String::from("db")];
    let prefix = String::from("svc-");
    let out = map_ordered(&names, 2, |n| format!("{prefix}{n}"));
    assert_eq!(out, ["svc-auth", "svc-db"]);
    assert_eq!(prefix, "svc-", "still ours");
}

#[test]
fn a_one_worker_spawns_no_thread() {
    let me = thread::current().id();
    let ids = map_ordered(&[1, 2, 3], 1, |_| thread::current().id());
    assert!(ids.iter().all(|&id| id == me));
    let ids = map_ordered(&[1], 8, |_| thread::current().id());
    assert_eq!(ids, [me], "one item needs one worker");
}

/// Each call waits (up to a deadline) until at least two threads are inside
/// `f` at the same time. A sequential implementation fails after the deadline
/// instead of hanging.
#[test]
fn a_several_workers_run_at_once() {
    let inside: Mutex<HashSet<ThreadId>> = Mutex::new(HashSet::new());
    let changed = Condvar::new();
    let deadline = Instant::now() + Duration::from_secs(10);
    let items: Vec<u32> = (0..8).collect();
    let ids = map_ordered(&items, 4, |_| {
        let mut set = inside.lock().unwrap();
        set.insert(thread::current().id());
        changed.notify_all();
        while set.len() < 2 && Instant::now() < deadline {
            set = changed
                .wait_timeout(set, Duration::from_millis(50))
                .unwrap()
                .0;
        }
        thread::current().id()
    });
    let distinct: HashSet<ThreadId> = ids.into_iter().collect();
    assert!(distinct.len() >= 2, "work ran on only one thread");
    assert!(distinct.len() <= 4, "more threads than jobs");
}

#[test]
fn a_work_is_handed_out_dynamically() {
    // Item 0 is slow. With 2 workers, the other worker must take the rest
    // while item 0 runs, instead of waiting for a fixed share.
    let done_while_slow = AtomicUsize::new(0);
    let slow_done = AtomicUsize::new(0);
    let items: Vec<u32> = (0..6).collect();
    map_ordered(&items, 2, |&i| {
        if i == 0 {
            let deadline = Instant::now() + Duration::from_secs(10);
            while done_while_slow.load(Ordering::SeqCst) < 5 && Instant::now() < deadline {
                thread::sleep(Duration::from_millis(5));
            }
            slow_done.store(1, Ordering::SeqCst);
        } else if slow_done.load(Ordering::SeqCst) == 0 {
            done_while_slow.fetch_add(1, Ordering::SeqCst);
        }
    });
    assert_eq!(done_while_slow.load(Ordering::SeqCst), 5);
}

#[test]
#[should_panic]
fn a_panics_propagate() {
    map_ordered(
        &[1, 2, 3, 4],
        2,
        |&x| if x == 3 { panic!("boom") } else { x },
    );
}

/// Writes `n` generated log files with varied sizes and some malformed lines.
fn generated_files(dir: &TempDir, n: usize) -> Vec<PathBuf> {
    let targets = ["auth", "db", "cache", "http", "billing"];
    let levels = ["TRACE", "DEBUG", "INFO", "WARN", "ERROR"];
    (0..n)
        .map(|f| {
            let mut text = String::new();
            for i in 0..(50 + f * 37) {
                let k = f * 1000 + i;
                if k % 41 == 0 {
                    text.push_str("this line is malformed\n");
                    continue;
                }
                text.push_str(&format!(
                    "2026-10-23T17:{:02}:{:02}Z {} {} user=u{} latency_ms={}\n",
                    k / 60 % 60,
                    k % 60,
                    levels[k % 5],
                    targets[k * 7 % 5],
                    k % 13,
                    k * 31 % 997
                ));
            }
            dir.file(&format!("gen{f}.log"), text)
        })
        .collect()
}

#[test]
fn b_summarize_files_is_the_same_for_every_jobs() {
    let dir = TempDir::new("par");
    let mut paths = generated_files(&dir, 9);
    paths.extend(["app.log", "web.log", "worker.log"].map(fixture));
    for group in [Group::Target, Group::Field("user".into())] {
        for filter in ["", "level>=warn", "latency_ms>500"] {
            let filter = Filter::parse(filter).unwrap();
            let sequential = summarize_files(&paths, &filter, &group, 1).unwrap();
            for jobs in [0, 2, 3, 16] {
                let parallel = summarize_files(&paths, &filter, &group, jobs).unwrap();
                assert_eq!(parallel, sequential, "jobs = {jobs}");
            }

            // And the same as one pass over all the text.
            let mut whole = Summary::new(group.clone());
            let mut tally = Tally::default();
            for p in &paths {
                let text = std::fs::read_to_string(p).unwrap();
                tally += process(&text, &filter, &mut whole, &mut |_| {}).unwrap();
            }
            assert_eq!(sequential, (tally, whole));
        }
    }
}

#[test]
fn b_summarize_files_reports_the_first_missing_file() {
    let paths = [
        fixture("app.log"),
        fixture("missing-1.log"),
        fixture("web.log"),
        fixture("missing-2.log"),
    ];
    for jobs in [1, 4] {
        match summarize_files(&paths, &Filter::all(), &Group::Target, jobs) {
            Err(Error::Open { path, .. }) => assert_eq!(path, paths[1], "jobs = {jobs}"),
            other => panic!("expected Error::Open, got {other:?}"),
        }
    }
}

fn run_in_memory(options: &Options) -> (u8, String, String) {
    let (mut out, mut err) = (Vec::new(), Vec::new());
    let status = run(options, &mut Cursor::new(""), &mut out, &mut err);
    (
        status,
        String::from_utf8(out).unwrap(),
        String::from_utf8(err).unwrap(),
    )
}

#[test]
fn c_cli_output_is_identical_for_every_jobs() {
    let dir = TempDir::new("par-cli");
    let mut inputs: Vec<Input> = generated_files(&dir, 6)
        .into_iter()
        .map(Input::Path)
        .collect();
    inputs.push(Input::Path(fixture("worker.log")));
    inputs.push(Input::Path(dir.path().join("missing.log")));
    inputs.push(Input::Path(fixture("app.log")));
    for command in [Command::Grep, Command::Count, Command::Report] {
        let sequential = Options {
            command,
            filter: "level>=info".into(),
            inputs: inputs.clone(),
            ..Options::default()
        };
        let expected = run_in_memory(&sequential);
        assert_eq!(expected.0, 2, "the missing file");
        for jobs in [0, 3, 8] {
            let parallel = Options {
                jobs,
                ..sequential.clone()
            };
            assert_eq!(
                run_in_memory(&parallel),
                expected,
                "{command:?} with jobs = {jobs}"
            );
        }
    }
}

#[test]
fn c_binary_jobs_flag() {
    let run = |jobs: &str, command: &str| {
        let out = Process::new(env!("CARGO_BIN_EXE_loglens"))
            .args([
                command,
                "--jobs",
                jobs,
                "fixtures/app.log",
                "fixtures/web.log",
                "fixtures/worker.log",
            ])
            .current_dir(env!("CARGO_MANIFEST_DIR"))
            .stdin(Stdio::null())
            .output()
            .unwrap();
        (out.status.code(), out.stdout, out.stderr)
    };
    for command in ["grep", "count", "report"] {
        let expected = run("1", command);
        assert_eq!(expected.0, Some(0));
        assert_eq!(run("4", command), expected, "{command}");
        assert_eq!(run("0", command), expected, "{command}");
    }
}

/// P. Predict BEFORE running: replace each `guess()` with your answer.
#[test]
fn p_predictions() {
    // P1. How many values arrive once every sender is dropped?
    let (tx, rx) = mpsc::channel();
    for i in 0..3 {
        let tx = tx.clone();
        thread::spawn(move || tx.send(i).unwrap());
    }
    drop(tx);
    let p1: usize = guess();
    assert_eq!(
        p1,
        rx.iter().count(),
        "P1: the loop ends when all senders are gone"
    );

    // P2. The strong count after a scoped thread cloned and dropped an Arc.
    let shared = Arc::new(5);
    thread::scope(|s| {
        s.spawn(|| {
            let mine = Arc::clone(&shared);
            assert_eq!(*mine, 5);
        });
    });
    let p2: usize = guess();
    assert_eq!(
        p2,
        Arc::strong_count(&shared),
        "P2: the clone died with the thread"
    );

    // P3. What does fetch_add return?
    let n = AtomicUsize::new(5);
    let p3: usize = guess();
    assert_eq!(
        p3,
        n.fetch_add(2, Ordering::SeqCst),
        "P3: the previous value"
    );
}
