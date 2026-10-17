//! LESSON 06 — RwLock and condition predicates
//! Run: cargo test --test chapter19 l06_
//!
//! PREDICT before running:
//! 1. Why can a Condvar notification not be treated as stored data?
//! 2. Can a reader upgrade an RwLock guard directly?
//!
//! IMPLEMENT the tasks; keep the tests.
//! EXPLAIN: Explain the predicate loop, spurious wakeups, and releasing a mutex while waiting.
//! COMPILER LAB (examples/ch19_scratch.rs): Use a Barrier for two workers to reach a phase boundary. Explain why a barrier cannot replace a general condition predicate.

use std::sync::{Arc, Condvar, Mutex, RwLock};
/// A. Clone a snapshot under a read lock, then release the guard before return.
fn snapshot(data: &RwLock<Vec<u32>>) -> Vec<u32> {
    todo!("A: implement the contract")
}
/// B. Publish a value under the mutex, then notify one waiter. Only one producer
/// and one consumer use this slot; publish happens once while the slot is empty.
fn publish(slot: &Arc<(Mutex<Option<u32>>, Condvar)>, value: u32) {
    todo!("B: implement the contract")
}
/// C. Wait for Some, rechecking the predicate after every wake, then take it.
/// The producer is responsible for eventually calling publish.
fn receive(slot: &Arc<(Mutex<Option<u32>>, Condvar)>) -> u32 {
    todo!("C: implement the contract")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn snapshots_are_owned() {
        let lock = RwLock::new(vec![1, 2]);
        let old = snapshot(&lock);
        lock.write().unwrap().push(3);
        assert_eq!(old, [1, 2]);
        assert_eq!(snapshot(&lock), [1, 2, 3]);
    }
    #[test]
    fn publication_survives_notification_before_wait() {
        let slot = Arc::new((Mutex::new(None), Condvar::new()));
        publish(&slot, 7);
        assert_eq!(receive(&slot), 7);
        assert_eq!(*slot.0.lock().unwrap(), None);
    }
    #[test]
    fn consumer_takes_an_existing_value() {
        let slot = Arc::new((Mutex::new(Some(19)), Condvar::new()));
        assert_eq!(receive(&slot), 19);
    }
}
