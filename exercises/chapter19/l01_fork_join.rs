//! LESSON 01 — Fork, join, and worker failures
//! Run: cargo test --test chapter19 l01_
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider:
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
    use practice::guess;
    #[test]
    fn a_output_order_matches_input() {
        assert_eq!(
            parallel_lengths(vec!["é".into(), "rust".into(), "".into()]),
            [1, 4, 0]
        );
        assert!(parallel_lengths(vec![]).is_empty());
    }
    #[test]
    fn a_many_workers_still_report_in_input_order() {
        // Later inputs are shorter, so they tend to finish first.
        let items: Vec<String> = (0..32).rev().map(|n| "🦀".repeat(n * 50)).collect();
        let expected: Vec<usize> = (0..32).rev().map(|n| n * 50).collect();
        assert_eq!(parallel_lengths(items), expected);
    }
    #[test]
    fn b_worker_results_and_panics_are_distinct() {
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

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    /// A wrong prediction is useful: read the failure, then explain it in NOTES.md.
    #[test]
    fn p_predictions() {
        // P1. What type is the payload of each panic? (literal message, formatted message)
        let literal = thread::spawn(|| panic!("disk full")).join().unwrap_err();
        let disk = String::from("sda");
        let formatted = thread::spawn(move || panic!("{disk} full"))
            .join()
            .unwrap_err();
        let p1: (bool, bool) = guess();
        assert_eq!(
            p1,
            (
                literal.downcast_ref::<&str>().is_some(),
                formatted.downcast_ref::<&str>().is_some()
            ),
            "P1: is the payload a &'static str?"
        );

        // P2. A named worker reports its own name.
        let name = thread::Builder::new()
            .name("worker-7".into())
            .spawn(|| thread::current().name().map(str::to_owned))
            .unwrap()
            .join()
            .unwrap();
        let p2: Option<String> = guess();
        assert_eq!(p2, name, "P2: thread names");

        // P3. The JoinHandle is dropped before the worker is allowed to start.
        // Does the worker still run and report back?
        let (go_tx, go_rx) = std::sync::mpsc::channel::<()>();
        let (done_tx, done_rx) = std::sync::mpsc::channel();
        let handle = thread::spawn(move || {
            go_rx.recv().unwrap();
            done_tx.send("ran").unwrap();
        });
        drop(handle);
        go_tx.send(()).unwrap();
        let p3: bool = guess();
        assert_eq!(
            p3,
            done_rx.recv() == Ok("ran"),
            "P3: dropping a JoinHandle detaches, it does not cancel"
        );
    }
}
