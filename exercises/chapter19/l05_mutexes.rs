//! LESSON 05 — Mutex invariants and poisoning
//! Run: cargo test --test chapter19 l05_
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider:
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
    use practice::guess;
    #[test]
    fn a_shared_updates_are_not_lost() {
        assert_eq!(count(4, 250), 1000);
        assert_eq!(count(0, 10), 0);
        assert_eq!(count(3, 0), 0);
        assert_eq!(count(8, 10_000), 80_000);
    }
    #[test]
    fn b_recovery_reestablishes_an_invariant() {
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
    #[test]
    fn b_reset_works_on_a_healthy_mutex() {
        let m = Mutex::new(vec![1, 2, 3]);
        reset(&m, vec![4]);
        assert!(!m.is_poisoned());
        assert_eq!(*m.lock().unwrap(), [4]);
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    #[test]
    fn p_predictions() {
        // P1. Is the lock free right after this statement? While `guard` is alive?
        let m = Mutex::new(vec![1, 2, 3]);
        let len = m.lock().unwrap().len();
        let free_after_statement = m.try_lock().is_ok();
        let guard = m.lock().unwrap();
        let free_while_guarded = m.try_lock().is_ok();
        drop(guard);
        let p1: (bool, bool) = guess();
        assert_eq!(
            p1,
            (free_after_statement, free_while_guarded),
            "P1: when a guard is dropped"
        );

        // P2. A worker pushes 4, then panics while holding the guard.
        // Is the mutex poisoned, and what length does recovery reveal?
        let shared = Arc::new(Mutex::new(vec![1, 2, 3]));
        let copy = Arc::clone(&shared);
        let _ = thread::spawn(move || {
            copy.lock().unwrap().push(4);
            let _guard = copy.lock().unwrap();
            panic!("fixture: panic while locked");
        })
        .join();
        let poisoned = shared.is_poisoned();
        let len = shared.lock().unwrap_or_else(|e| e.into_inner()).len();
        let p2: (bool, usize) = guess();
        assert_eq!(p2, (poisoned, len), "P2: poisoning does not roll back");

        // P3. This worker panics while it holds NO guard. Poisoned?
        let calm = Arc::new(Mutex::new(0));
        let copy = Arc::clone(&calm);
        let _ = thread::spawn(move || {
            *copy.lock().unwrap() += 1;
            panic!("fixture: panic after unlocking");
        })
        .join();
        let p3: bool = guess();
        assert_eq!(p3, calm.is_poisoned(), "P3: only a panicking guard poisons");
    }
}
