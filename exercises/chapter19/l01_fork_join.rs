//! LESSON 01 — Fork, join, and worker failures
//! Run: cargo test --test chapter19 l01_
//!
//! PREDICT before running:
//! 1. Does dropping JoinHandle wait for the worker?
//! 2. Does join turn a panic into your application error type?
//!
//! IMPLEMENT the tasks; keep the tests.
//! EXPLAIN: Explain result nesting and ownership crossing a move closure.
//! COMPILER LAB (examples/ch19_scratch.rs): Try spawning a closure borrowing a local String, then repair with move. Next return a borrowed value using thread::scope.

use std::thread;
/// A. Move each String into its own worker, return Unicode scalar counts in
/// input order (not completion order). Join every worker; these workers do not panic.
fn parallel_lengths(items: Vec<String>) -> Vec<usize> {
    todo!("A: implement the contract")
}
/// B. Join an existing worker and distinguish panic from application failure.
/// Preserve Ok/Err from the worker; map panic to "worker panicked".
fn join_result(worker: thread::JoinHandle<Result<u32, &'static str>>) -> Result<u32, &'static str> {
    todo!("B: implement the contract")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn output_order_matches_input() {
        assert_eq!(
            parallel_lengths(vec!["é".into(), "rust".into(), "".into()]),
            [1, 4, 0]
        );
        assert!(parallel_lengths(vec![]).is_empty());
    }
    #[test]
    fn worker_results_and_panics_are_distinct() {
        assert_eq!(join_result(thread::spawn(|| Ok(9))), Ok(9));
        assert_eq!(
            join_result(thread::spawn(|| Err("invalid"))),
            Err("invalid")
        );
        assert_eq!(
            join_result(thread::spawn(|| panic!("worker only"))),
            Err("worker panicked")
        );
    }
}
