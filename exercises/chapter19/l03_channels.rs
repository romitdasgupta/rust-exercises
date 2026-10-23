//! LESSON 03 — Ownership and channel closure
//! Run: cargo test --test chapter19 l03_
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider:
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
    use practice::guess;
    use std::sync::mpsc::{TryRecvError, TrySendError};
    #[test]
    fn a_ordered_worker_terminates_on_input_close() {
        assert_eq!(
            uppercase_worker(vec!["one".into(), "ß".into()]),
            ["ONE", "SS"]
        );
        assert!(uppercase_worker(vec![]).is_empty());
        let many: Vec<String> = (0..200).map(|i| format!("item{i}")).collect();
        let expected: Vec<String> = (0..200).map(|i| format!("ITEM{i}")).collect();
        assert_eq!(uppercase_worker(many), expected);
    }
    #[test]
    fn b_empty_and_disconnected_are_different() {
        let (tx, rx) = mpsc::channel();
        tx.send(1).unwrap();
        tx.send(2).unwrap();
        assert_eq!(drain_now(&rx), (vec![1, 2], false));
        assert_eq!(drain_now(&rx), (vec![], false));
        drop(tx);
        assert_eq!(drain_now(&rx), (vec![], true));
    }
    #[test]
    fn b_buffered_messages_arrive_before_disconnection() {
        let (tx, rx) = mpsc::channel();
        tx.send("last words").unwrap();
        drop(tx);
        assert_eq!(drain_now(&rx), (vec!["last words"], true));
        assert_eq!(drain_now(&rx), (vec![], true));
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    #[test]
    fn p_predictions() {
        // P1. The only sender sends one value and is dropped. Two recv calls follow.
        let (tx, rx) = mpsc::channel();
        tx.send(5).unwrap();
        drop(tx);
        let first = rx.recv().ok();
        let second_failed = rx.recv().is_err();
        let p1: (Option<i32>, bool) = guess();
        assert_eq!(
            p1,
            (first, second_failed),
            "P1: buffered values outlive senders"
        );

        // P2. A sync_channel of capacity 1 with nobody receiving: do two try_sends succeed?
        let (tx, rx) = mpsc::sync_channel(1);
        let first_ok = tx.try_send('a').is_ok();
        let second_full = matches!(tx.try_send('b'), Err(TrySendError::Full('b')));
        let p2: (bool, bool) = guess();
        assert_eq!(p2, (first_ok, second_full), "P2: bounded capacity");

        // P3. The receiver is gone. What does send hand back?
        let (tx, rx) = mpsc::channel::<String>();
        drop(rx);
        let returned = tx.send("lost?".to_string()).err().map(|e| e.0);
        let p3: Option<String> = guess();
        assert_eq!(p3, returned, "P3: a failed send returns the value");

        // P4. A clone of the sender is still alive after the original is dropped.
        let (tx, rx) = mpsc::channel::<u8>();
        let tx2 = tx.clone();
        drop(tx);
        let p4: bool = guess();
        assert_eq!(
            p4,
            rx.try_recv() == Err(TryRecvError::Disconnected),
            "P4: is the channel disconnected?"
        );
        drop(tx2);
    }
}
