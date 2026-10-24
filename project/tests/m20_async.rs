//! OPTIONAL MILESTONE m20 — A std-only async reader and executor (Chapters 20–21)
//! Run: cargo test --test m20_async
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider: who calls `poll`, and what must a future
//! arrange before it returns `Pending`?
//! IMPLEMENT `src/aio.rs`: `Future for NextLine`, `block_on`, `process_async`.
//! EXPLAIN afterward: what `Pin` protects here, and why a real program would use an
//! async runtime (and non-blocking files or sockets) instead of this executor.

mod common;

use common::guess;
use loglens::aio::{AsyncLines, Trickle, block_on, process_async};
use loglens::pipeline::process;
use loglens::{Filter, Tally};
use std::cell::Cell;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::task::{Context, Poll};
use std::time::Duration;

async fn all_lines(lines: &mut AsyncLines) -> Vec<Vec<u8>> {
    let mut out = Vec::new();
    while let Some(line) = lines.next_line().await {
        out.push(line);
    }
    out
}

#[test]
fn a_next_line_reads_lines_across_chunks() {
    let mut lines = AsyncLines::new(Trickle::new("a\nbb\r\n\nlast", 2));
    let got = block_on(all_lines(&mut lines));
    assert_eq!(got, [&b"a"[..], b"bb", b"", b"last"]);
    assert_eq!(block_on(lines.next_line()), None, "still done");
    assert!(
        lines.source().pendings() >= 6,
        "the source was not ready many times"
    );
}

#[test]
fn a_next_line_edges() {
    let mut lines = AsyncLines::new(Trickle::new("", 4));
    assert_eq!(block_on(lines.next_line()), None);
    let mut lines = AsyncLines::new(Trickle::new("x\n", 64));
    assert_eq!(
        block_on(all_lines(&mut lines)),
        [b"x".to_vec()],
        "no empty last line"
    );
}

#[test]
fn b_block_on_ready_futures() {
    assert_eq!(block_on(async { 1 + 2 }), 3);
    assert_eq!(block_on(std::future::ready("x")), "x");
}

/// Pending until another thread sets the flag and wakes the task. Counts polls.
struct WokenLater {
    flag: Arc<AtomicBool>,
    started: bool,
    polls: u32,
}

impl Future for WokenLater {
    type Output = u32;
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<u32> {
        self.polls += 1;
        if self.flag.load(Ordering::SeqCst) {
            return Poll::Ready(self.polls);
        }
        if !self.started {
            self.started = true;
            let (flag, waker) = (Arc::clone(&self.flag), cx.waker().clone());
            std::thread::spawn(move || {
                std::thread::sleep(Duration::from_millis(50));
                flag.store(true, Ordering::SeqCst);
                waker.wake();
            });
        }
        Poll::Pending
    }
}

#[test]
fn b_block_on_sleeps_until_woken() {
    let polls = block_on(WokenLater {
        flag: Arc::new(AtomicBool::new(false)),
        started: false,
        polls: 0,
    });
    assert!(polls >= 2);
    assert!(
        polls <= 10,
        "polled {polls} times: park instead of spinning"
    );
}

const TEXT: &str = concat!(
    "# input\n",
    "2026-10-23T17:03:00Z INFO auth user=ada latency_ms=42\n",
    "garbage\n",
    "2026-10-23T17:03:02Z WARN auth user=bob msg=\"slow\"\r\n",
    "2026-10-23T17:03:05Z ERROR db msg=\"reset\"",
);

#[test]
fn c_process_async_equals_process() {
    for filter in ["", "level>=warn", "target=nobody"] {
        let filter = Filter::parse(filter).unwrap();
        let mut expected_lines: Vec<String> = Vec::new();
        let expected = process(TEXT, &filter, &mut expected_lines, &mut |_| {}).unwrap();

        let mut lines = AsyncLines::new(Trickle::new(TEXT, 7));
        let mut got_lines: Vec<String> = Vec::new();
        let got = block_on(process_async(&mut lines, &filter, &mut got_lines)).unwrap();
        assert_eq!((got, got_lines), (expected, expected_lines));
    }
    let mut lines = AsyncLines::new(Trickle::new(
        b"2026-10-23T17:03:00Z INFO a x=\xff\n".to_vec(),
        3,
    ));
    let mut sink: Vec<String> = Vec::new();
    let tally = block_on(process_async(&mut lines, &Filter::all(), &mut sink)).unwrap();
    assert_eq!(
        tally,
        Tally {
            records: 0,
            matched: 0,
            malformed: 1
        }
    );
}

/// Pending `n` times (waking itself each time), then Ready.
struct PendingTimes(u32);

impl Future for PendingTimes {
    type Output = ();
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        if self.0 == 0 {
            return Poll::Ready(());
        }
        self.0 -= 1;
        cx.waker().wake_by_ref();
        Poll::Pending
    }
}

/// P. Predict BEFORE running: replace each `guess()` with your answer.
/// The sizes assume a 64-bit target.
#[test]
fn p_predictions() {
    // P1. Does an async block run before it is awaited or polled?
    let ran = Cell::new(false);
    let future = async { ran.set(true) };
    let before = ran.get();
    block_on(future);
    let p1: (bool, bool) = guess();
    assert_eq!(p1, (before, ran.get()), "P1: futures are lazy");

    // P2. The size of `std::future::ready(5u64)`.
    let p2: usize = guess();
    assert_eq!(
        p2,
        size_of_val(&std::future::ready(5u64)),
        "P2: Ready<T> holds an Option<T>"
    );

    // P3. How many times does an async block awaiting PendingTimes(2) get polled?
    let polls = Cell::new(0);
    let counted = CountPolls(&polls, Box::pin(PendingTimes(2)));
    block_on(counted);
    let p3: u32 = guess();
    assert_eq!(
        p3,
        polls.get(),
        "P3: one poll per Pending, plus the final one"
    );
}

/// Counts how often the inner future is polled.
struct CountPolls<'a>(&'a Cell<u32>, Pin<Box<PendingTimes>>);

impl Future for CountPolls<'_> {
    type Output = ();
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        self.0.set(self.0.get() + 1);
        self.1.as_mut().poll(cx)
    }
}
