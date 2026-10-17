//! LESSON 03 — Ownership and channel closure
//! Run: cargo test --test chapter19 l03_
//!
//! PREDICT before running:
//! 1. When does recv report disconnection?
//! 2. Does send guarantee the receiver processed the message?
//!
//! IMPLEMENT the tasks; keep the tests.
//! EXPLAIN: Explain FIFO per sender, nondeterministic ordering across senders, and bounded-channel backpressure.
//! COMPILER LAB (examples/ch19_scratch.rs): Clone a sender and drop the original. Use try_recv to distinguish Empty from Disconnected, then drop the clone.

use std::{sync::mpsc, thread};
/// A. Move Strings through an unbounded channel to a worker which uppercases
/// them. Close the input sender before joining. Preserve input order.
fn uppercase_worker(items: Vec<String>) -> Vec<String> {
    todo!("A: implement the contract")
}
/// B. Drain currently available messages without blocking. Return messages and
/// whether the channel is disconnected. Empty != disconnected.
fn drain_now<T>(input: &mpsc::Receiver<T>) -> (Vec<T>, bool) {
    todo!("B: implement the contract")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ordered_worker_terminates_on_input_close() {
        assert_eq!(
            uppercase_worker(vec!["one".into(), "ß".into()]),
            ["ONE", "SS"]
        );
        assert!(uppercase_worker(vec![]).is_empty());
    }
    #[test]
    fn empty_and_disconnected_are_different() {
        let (tx, rx) = mpsc::channel();
        tx.send(1).unwrap();
        tx.send(2).unwrap();
        assert_eq!(drain_now(&rx), (vec![1, 2], false));
        assert_eq!(drain_now(&rx), (vec![], false));
        drop(tx);
        assert_eq!(drain_now(&rx), (vec![], true));
    }
}
