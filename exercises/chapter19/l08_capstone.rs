//! LESSON 08 — Capstone: bounded parallel pipeline
//! Run: cargo test --test chapter19 l08_
//!
//! PREDICT before running:
//! 1. Can joining a producer before draining its full bounded channel deadlock?
//! 2. Why is output order from multiple workers unspecified?
//!
//! IMPLEMENT the tasks; keep the tests.
//! EXPLAIN: Trace ownership and closure of every sender. Explain when sorted output is a contract versus hiding a bug.
//! COMPILER LAB (examples/ch19_scratch.rs): Extend to two stages and inject a worker Result error. Use completion channels, bounded recv_timeout, and joined handles; never use sleep as coordination.

use std::{sync::mpsc, thread};
/// A. Spawn one producer per nonempty chunk. Send (original_index, squared i64)
/// through sync_channel with capacity 2. Drain while producers run, join all,
/// then sort by index and return squares in input order. Empty input is valid.
/// Use thread::scope to borrow input. chunk_size must be nonzero; like
/// `slice::chunks`, panic if it is 0.
fn squared_pipeline(input: &[i32], chunk_size: usize) -> Vec<i64> {
    todo!("A: implement the contract")
}
/// B. Return received items until channel closure, propagating the first
/// application error as soon as it arrives (do not wait for closure after an error). Dropping the receiver is the caller's cancellation signal;
/// producers must handle send errors instead of unwrap when cancellation is possible.
fn collect_results<T, E>(input: mpsc::Receiver<Result<T, E>>) -> Result<Vec<T>, E> {
    todo!("B: implement the contract")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    /// Runs the pipeline on another thread so a deadlock fails the test instead of hanging it.
    fn pipeline_within(input: Vec<i32>, chunk_size: usize, secs: u64) -> Vec<i64> {
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || tx.send(squared_pipeline(&input, chunk_size)).unwrap());
        rx.recv_timeout(Duration::from_secs(secs)).expect(
            "no result in time: drain the capacity-2 channel while producers run, then join",
        )
    }
    #[test]
    fn a_bounded_pipeline_retains_input_order() {
        assert_eq!(
            pipeline_within(vec![3, -2, 0, i32::MIN, 7], 2, 10),
            [9, 4, 0, 4611686018427387904, 49]
        );
        assert_eq!(squared_pipeline(&[], 3), []);
        assert_eq!(squared_pipeline(&[8], 9), [64]);
    }
    #[test]
    fn a_many_producers_drain_without_deadlock() {
        let input: Vec<i32> = (-1500..1500).collect();
        let expected: Vec<i64> = input.iter().map(|&x| i64::from(x) * i64::from(x)).collect();
        assert_eq!(pipeline_within(input, 7, 20), expected);
    }
    #[test]
    fn b_application_failure_stops_collection() {
        let (tx, rx) = mpsc::channel();
        tx.send(Ok::<_, &str>(1)).unwrap();
        tx.send(Err("bad row")).unwrap();
        drop(tx);
        assert_eq!(collect_results(rx), Err("bad row"));
        let (tx, rx) = mpsc::channel();
        tx.send(Ok::<_, &str>(2)).unwrap();
        drop(tx);
        assert_eq!(collect_results(rx), Ok(vec![2]));
        let (tx, rx) = mpsc::channel::<Result<u8, &str>>();
        drop(tx);
        assert_eq!(collect_results(rx), Ok(vec![]));
    }
    #[test]
    fn b_first_error_returns_without_waiting_for_closure() {
        let (tx, rx) = mpsc::channel();
        tx.send(Ok::<u8, &str>(1)).unwrap();
        tx.send(Err("first")).unwrap();
        tx.send(Err("second")).unwrap();
        tx.send(Ok(3)).unwrap();
        // `tx` stays alive, like a producer that is still running.
        let (done_tx, done_rx) = mpsc::channel();
        let collector = thread::spawn(move || done_tx.send(collect_results(rx)).unwrap());
        let got = done_rx.recv_timeout(Duration::from_secs(5));
        drop(tx);
        collector.join().unwrap();
        assert_eq!(
            got,
            Ok(Err("first")),
            "return the first error immediately; dropping the receiver is the cancellation"
        );
    }
}
