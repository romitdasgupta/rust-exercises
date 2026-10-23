//! LESSON E06 — Async services with tokio (after Chapters 20 and 21)
//! Run: cargo test --manifest-path ecosystem/Cargo.toml e06_
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider: When a `select!` branch wins, what
//! happens to the other branches' futures, and to any value they owned?
//! IMPLEMENT the tasks below. Preserve the tests and add an edge case.
//! The tests use `#[tokio::test(start_paused = true)]`: time is virtual and jumps
//! forward whenever every task is waiting on a timer, so "sleep 50ms" costs nothing
//! and elapsed times are exact. Never use `std::thread::sleep` in async code.
//! EXPLAIN afterward: What makes a future *cancellation safe*? Why does `enqueue_within`
//! need `reserve()` rather than `timeout(.., tx.send(job))`? Why must CPU-heavy work go
//! to `spawn_blocking`?
//! JUDGMENT: tokio is the de-facto async runtime for network services. Many libraries
//! (hyper, reqwest, tonic, sqlx) build on it. Use async for many concurrent waits (sockets,
//! timers); use threads, or rayon for CPU parallelism, when work is compute-bound or
//! there are few concurrent tasks. smol and embassy (embedded) are real alternatives
//! in narrower niches.

use std::future::Future;
use std::time::Duration;
use tokio::sync::{mpsc, oneshot};

/// A. Run `work(id)` for every id on a `JoinSet` with at most `limit` (≥ 1) tasks in
/// flight at once. Return `(id, result)` sorted by id. A task that panics gives
/// `Err("panicked".to_string())` for its id; the other tasks still finish.
/// Tip: `JoinSet::join_next_with_id` and `JoinError::id` link failures to tasks.
pub async fn run_limited<F, Fut>(
    ids: Vec<u32>,
    limit: usize,
    work: F,
) -> Vec<(u32, Result<u64, String>)>
where
    F: Fn(u32) -> Fut,
    Fut: Future<Output = u64> + Send + 'static,
{
    todo!("Implement this task")
}

/// Why a job was not accepted. The job is handed back, never silently lost.
#[derive(Debug, PartialEq, Eq)]
pub enum Rejected<T> {
    Busy(T),
    Closed(T),
}

/// B. Load shedding: enqueue without waiting. A full queue gives `Busy(job)`, a queue
/// whose receiver is gone gives `Closed(job)`.
pub fn try_enqueue<T>(tx: &mpsc::Sender<T>, job: T) -> Result<(), Rejected<T>> {
    todo!("Implement this task")
}

/// B. Backpressure with a deadline: wait at most `patience` for queue space.
/// Timeout gives `Busy(job)`, closed gives `Closed(job)`. The job must come back on
/// timeout, so it must not be moved into a future that the timeout cancels.
pub async fn enqueue_within<T>(
    tx: &mpsc::Sender<T>,
    job: T,
    patience: Duration,
) -> Result<(), Rejected<T>> {
    todo!("Implement this task")
}

/// C. Group items from `rx` into batches, returning `(time since start, batch)` for each
/// flush, in order. Flush the current batch when
/// - it reaches `max_items` items, or
/// - `max_wait` has passed since its *first* item arrived (later items must not push
///   the deadline back), or
/// - `shutdown` completes (a value or a dropped sender), or `rx` is closed and empty.
///   Then flush any partial batch and return.
///
/// Never flush an empty batch. Use one `select!` loop. Keep the batch and the
/// deadline outside the branch futures: they are dropped whenever another branch wins.
pub async fn batch(
    rx: mpsc::Receiver<u32>,
    max_items: usize,
    max_wait: Duration,
    shutdown: oneshot::Receiver<()>,
) -> Vec<(Duration, Vec<u32>)> {
    todo!("Implement this task")
}

/// D. Compute `checksum(input)` for every input on tokio's blocking thread pool
/// (`spawn_blocking`), all concurrently, and return the results in input order.
/// A checksum that panics gives `Err("panicked".to_string())` for that input only.
pub async fn checksum_all<F>(inputs: Vec<Vec<u8>>, checksum: F) -> Vec<Result<u64, String>>
where
    F: Fn(&[u8]) -> u64 + Send + Sync + 'static,
{
    todo!("Implement this task")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::guess;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::{Arc, Condvar, Mutex};
    use tokio::task::JoinSet;
    use tokio::time::{self, Instant};

    const MS: Duration = Duration::from_millis(1);

    #[tokio::test(start_paused = true)]
    async fn a_limits_concurrency_and_sorts_results() {
        let active = Arc::new(AtomicUsize::new(0));
        let peak = Arc::new(AtomicUsize::new(0));
        let start = Instant::now();
        let results = run_limited(vec![4, 2, 3, 1], 2, |id| {
            let (active, peak) = (Arc::clone(&active), Arc::clone(&peak));
            async move {
                let now = active.fetch_add(1, Ordering::SeqCst) + 1;
                peak.fetch_max(now, Ordering::SeqCst);
                time::sleep(MS * 10 * id).await;
                active.fetch_sub(1, Ordering::SeqCst);
                u64::from(id) * 100
            }
        })
        .await;
        let expected: Vec<(u32, Result<u64, String>)> =
            (1..=4).map(|id| (id, Ok(u64::from(id) * 100))).collect();
        assert_eq!(results, expected);
        assert_eq!(
            peak.load(Ordering::SeqCst),
            2,
            "at most `limit`, and actually concurrent"
        );
        // 40|20 start; 20 ends at 20 → 30 starts (ends 50); 40 ends → 10 starts (ends 50).
        assert_eq!(start.elapsed(), MS * 50);
    }

    #[tokio::test(start_paused = true)]
    async fn a_panics_are_reported_per_task() {
        let results = run_limited(vec![1, 2, 3], 3, |id| async move {
            time::sleep(MS).await;
            assert_ne!(id, 2, "task 2 fails");
            u64::from(id)
        })
        .await;
        assert_eq!(
            results,
            [(1, Ok(1)), (2, Err("panicked".to_string())), (3, Ok(3))]
        );
        assert!(run_limited(vec![], 1, |_| async { 0 }).await.is_empty());
    }

    #[tokio::test(start_paused = true)]
    async fn b_try_enqueue_sheds_load() {
        let (tx, mut rx) = mpsc::channel(1);
        assert_eq!(try_enqueue(&tx, "a"), Ok(()));
        assert_eq!(try_enqueue(&tx, "b"), Err(Rejected::Busy("b")));
        assert_eq!(rx.recv().await, Some("a"));
        assert_eq!(try_enqueue(&tx, "c"), Ok(()));
        drop(rx);
        assert_eq!(try_enqueue(&tx, "d"), Err(Rejected::Closed("d")));
    }

    #[tokio::test(start_paused = true)]
    async fn b_enqueue_within_waits_then_gives_the_job_back() {
        let (tx, mut rx) = mpsc::channel(1);
        tx.send(String::from("first")).await.unwrap();

        let start = Instant::now();
        let result = enqueue_within(&tx, String::from("second"), MS * 50).await;
        assert_eq!(result, Err(Rejected::Busy(String::from("second"))));
        assert_eq!(start.elapsed(), MS * 50);

        let consumer = tokio::spawn(async move {
            time::sleep(MS * 20).await;
            let first = rx.recv().await;
            (first, rx)
        });
        let start = Instant::now();
        assert_eq!(
            enqueue_within(&tx, String::from("third"), MS * 50).await,
            Ok(())
        );
        assert_eq!(start.elapsed(), MS * 20, "space appeared after 20ms");
        let (first, mut rx) = consumer.await.unwrap();
        assert_eq!(first.as_deref(), Some("first"));
        assert_eq!(rx.recv().await.as_deref(), Some("third"));

        drop(rx);
        let result = enqueue_within(&tx, String::from("late"), MS * 50).await;
        assert_eq!(result, Err(Rejected::Closed(String::from("late"))));
    }

    #[tokio::test(start_paused = true)]
    async fn c_deadline_counts_from_the_first_item() {
        let (tx, rx) = mpsc::channel(8);
        let (_stop, shutdown) = oneshot::channel();
        tokio::spawn(async move {
            for i in 0..5 {
                if i > 0 {
                    time::sleep(MS * 30).await;
                }
                tx.send(i).await.unwrap();
            }
        });
        let batches = batch(rx, 10, MS * 50, shutdown).await;
        assert_eq!(
            batches,
            [
                (MS * 50, vec![0, 1]),
                (MS * 110, vec![2, 3]),
                (MS * 120, vec![4]),
            ]
        );
    }

    #[tokio::test(start_paused = true)]
    async fn c_full_batches_flush_immediately() {
        let (tx, rx) = mpsc::channel(8);
        let (_stop, shutdown) = oneshot::channel();
        for i in 0..5 {
            tx.send(i).await.unwrap();
        }
        drop(tx);
        let batches = batch(rx, 2, MS * 50, shutdown).await;
        assert_eq!(
            batches,
            [
                (Duration::ZERO, vec![0, 1]),
                (Duration::ZERO, vec![2, 3]),
                (Duration::ZERO, vec![4]),
            ]
        );
    }

    #[tokio::test(start_paused = true)]
    async fn c_shutdown_flushes_the_partial_batch() {
        let (tx, rx) = mpsc::channel(8);
        let (stop, shutdown) = oneshot::channel();
        tx.send(1).await.unwrap();
        tx.send(2).await.unwrap();
        tokio::spawn(async move {
            time::sleep(MS * 10).await;
            stop.send(()).unwrap();
        });
        let start = Instant::now();
        let batches = batch(rx, 10, MS * 50, shutdown).await;
        assert_eq!(batches, [(MS * 10, vec![1, 2])]);
        assert_eq!(
            start.elapsed(),
            MS * 10,
            "returns at shutdown even though tx is open"
        );
        drop(tx);

        let (tx, rx) = mpsc::channel::<u32>(1);
        let (stop, shutdown) = oneshot::channel();
        drop(stop);
        assert!(batch(rx, 10, MS * 50, shutdown).await.is_empty());
        drop(tx);
    }

    #[tokio::test]
    async fn d_checksums_run_off_the_runtime_thread() {
        let runtime_thread = std::thread::current().id();
        let threads = Arc::new(Mutex::new(Vec::new()));
        let seen = Arc::clone(&threads);
        let inputs = vec![b"abc".to_vec(), Vec::new(), vec![255; 1000]];
        let results = checksum_all(inputs, move |bytes| {
            seen.lock().unwrap().push(std::thread::current().id());
            bytes.iter().map(|&b| u64::from(b)).sum()
        })
        .await;
        assert_eq!(results, [Ok(294), Ok(0), Ok(255_000)]);
        let threads = threads.lock().unwrap();
        assert_eq!(threads.len(), 3);
        assert!(
            threads.iter().all(|&t| t != runtime_thread),
            "ran on the async thread"
        );
    }

    #[tokio::test]
    async fn d_checksums_run_concurrently_and_isolate_panics() {
        // Each checksum waits (up to 5s) until all three are running at once.
        let gate = Arc::new((Mutex::new(0usize), Condvar::new()));
        let inputs = vec![vec![1], vec![2], vec![3]];
        let results = checksum_all(inputs, move |bytes| {
            let (count, cvar) = &*gate;
            let mut running = count.lock().unwrap();
            *running += 1;
            cvar.notify_all();
            let (running, timeout) = cvar
                .wait_timeout_while(running, Duration::from_secs(5), |n| *n < 3)
                .unwrap();
            drop(running);
            assert!(!timeout.timed_out(), "checksums did not run concurrently");
            assert_ne!(bytes[0], 2, "input 2 is corrupt");
            u64::from(bytes[0])
        })
        .await;
        assert_eq!(results, [Ok(1), Err("panicked".to_string()), Ok(3)]);
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    #[tokio::test(start_paused = true)]
    async fn p_predictions() {
        // P1. Virtual time: how much time has `Instant` seen pass after `sleep(1 hour)`?
        let start = Instant::now();
        time::sleep(Duration::from_secs(3600)).await;
        let p1: Duration = guess();
        assert_eq!(p1, start.elapsed(), "P1: paused time auto-advances exactly");

        // P2. channel(2); a consumer takes one item every 10ms. When has the producer
        // finished sending 6 items?
        let (tx, mut rx) = mpsc::channel(2);
        tokio::spawn(async move {
            loop {
                time::sleep(MS * 10).await;
                if rx.recv().await.is_none() {
                    break;
                }
            }
        });
        let start = Instant::now();
        for i in 0..6 {
            tx.send(i).await.unwrap();
        }
        let p2: Duration = guess();
        assert_eq!(p2, start.elapsed(), "P2: a full channel makes send wait");

        // P3. Tasks 0, 1, 2 sleep 30ms, 10ms, 20ms. Order of `join_next` results?
        let mut set = JoinSet::new();
        for (id, ms) in [(0, 30), (1, 10), (2, 20)] {
            set.spawn(async move {
                time::sleep(MS * ms).await;
                id
            });
        }
        let order = set.join_all().await;
        let p3: Vec<i32> = guess();
        assert_eq!(p3, order, "P3: join order is completion order");

        // P4. A zero timeout around a future that is already ready: Ok or Err?
        let p4: bool = guess();
        assert_eq!(
            p4,
            time::timeout(Duration::ZERO, async { 5 }).await.is_ok(),
            "P4: timeout polls the inner future first"
        );
    }
}
