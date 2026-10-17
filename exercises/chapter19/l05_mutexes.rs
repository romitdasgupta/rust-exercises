//! LESSON 05 — Mutex invariants and poisoning
//! Run: cargo test --test chapter19 l05_
//!
//! PREDICT before running:
//! 1. Can a non-mut Mutex binding still allow mutation inside its guard?
//! 2. Does poisoning prove the protected value is corrupt?
//!
//! IMPLEMENT the tasks; keep the tests.
//! EXPLAIN: State the counter invariant. Explain guard lifetimes, panic poisoning, lock order, and why recursive locking can deadlock.
//! COMPILER LAB (examples/ch19_scratch.rs): Create two locks and write down a conflicting lock-order trace without running it. Repair every path to acquire in the same order.

use std::{
    sync::{Arc, Mutex},
    thread,
};
/// A. Spawn workers threads, each adding increments to one shared counter.
/// Join all; return the final value. Inputs guarantee no usize overflow.
fn count(workers: usize, increments: usize) -> usize {
    todo!("A: implement the contract")
}
/// B. Reset an ordinary or poisoned mutex to replacement, then clear poison.
/// This explicit recovery discards the old value; it does not trust its invariant.
fn reset(value: &Mutex<Vec<u8>>, replacement: Vec<u8>) {
    todo!("B: implement the contract")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn shared_updates_are_not_lost() {
        assert_eq!(count(4, 250), 1000);
        assert_eq!(count(0, 10), 0);
    }
    #[test]
    fn recovery_reestablishes_an_invariant() {
        let v = Arc::new(Mutex::new(vec![1]));
        let copy = Arc::clone(&v);
        let panicker = thread::spawn(move || {
            let mut g = copy.lock().unwrap();
            g.push(2);
            panic!("poison fixture")
        });
        assert!(panicker.join().is_err());
        assert!(v.is_poisoned());
        reset(&v, vec![9]);
        assert!(!v.is_poisoned());
        assert_eq!(*v.lock().unwrap(), [9]);
        reset(&v, vec![]);
        assert!(v.lock().unwrap().is_empty());
    }
}
