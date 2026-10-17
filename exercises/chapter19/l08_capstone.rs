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
/// Use thread::scope to borrow input; chunk_size must be nonzero.
fn squared_pipeline(input: &[i32], chunk_size: usize) -> Vec<i64> {
    todo!("A: implement the contract")
}
/// B. Return received items until channel closure, propagating the first
/// application error. Dropping the receiver is the caller's cancellation signal;
/// producers must handle send errors instead of unwrap when cancellation is possible.
fn collect_results<T, E>(input: mpsc::Receiver<Result<T, E>>) -> Result<Vec<T>, E> {
    todo!("B: implement the contract")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn bounded_pipeline_retains_input_order() {
        assert_eq!(
            squared_pipeline(&[3, -2, 0, i32::MIN, 7], 2),
            [9, 4, 0, 4611686018427387904, 49]
        );
        assert_eq!(squared_pipeline(&[], 3), []);
        assert_eq!(squared_pipeline(&[8], 9), [64]);
    }
    #[test]
    fn application_failure_stops_collection() {
        let (tx, rx) = mpsc::channel();
        tx.send(Ok::<_, &str>(1)).unwrap();
        tx.send(Err("bad row")).unwrap();
        drop(tx);
        assert_eq!(collect_results(rx), Err("bad row"));
        let (tx, rx) = mpsc::channel();
        tx.send(Ok::<_, &str>(2)).unwrap();
        drop(tx);
        assert_eq!(collect_results(rx), Ok(vec![2]));
    }
}
