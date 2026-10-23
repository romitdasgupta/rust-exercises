//! LESSON 06 — RwLock and condition predicates
//! Run: cargo test --test chapter19 l06_
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider:
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
    use practice::guess;
    use std::{
        sync::{Barrier, mpsc},
        thread,
        time::Duration,
    };
    #[test]
    fn a_snapshots_are_owned() {
        let lock = RwLock::new(vec![1, 2]);
        let old = snapshot(&lock);
        lock.write().unwrap().push(3);
        assert_eq!(old, [1, 2]);
        assert_eq!(snapshot(&lock), [1, 2, 3]);
    }
    #[test]
    fn a_snapshot_shares_the_lock_with_other_readers() {
        let lock = Arc::new(RwLock::new(vec![4, 5]));
        let reader = lock.read().unwrap();
        let (tx, rx) = mpsc::channel();
        let copy = Arc::clone(&lock);
        let worker = thread::spawn(move || tx.send(snapshot(&copy)).unwrap());
        let got = rx.recv_timeout(Duration::from_secs(5));
        drop(reader);
        worker.join().unwrap();
        assert_eq!(
            got.ok(),
            Some(vec![4, 5]),
            "snapshot waited for another reader: take a read lock, not a write lock"
        );
    }
    #[test]
    fn b_c_publication_survives_notification_before_wait() {
        let slot = Arc::new((Mutex::new(None), Condvar::new()));
        publish(&slot, 7);
        assert_eq!(receive(&slot), 7);
        assert_eq!(*slot.0.lock().unwrap(), None);
    }
    #[test]
    fn c_consumer_takes_an_existing_value() {
        let slot = Arc::new((Mutex::new(Some(19)), Condvar::new()));
        assert_eq!(receive(&slot), 19);
        assert_eq!(*slot.0.lock().unwrap(), None);
    }
    #[test]
    fn b_c_waiting_consumer_is_woken() {
        let slot = Arc::new((Mutex::new(None), Condvar::new()));
        let (started_tx, started_rx) = mpsc::channel();
        let (done_tx, done_rx) = mpsc::channel();
        let copy = Arc::clone(&slot);
        let consumer = thread::spawn(move || {
            started_tx.send(()).unwrap();
            done_tx.send(receive(&copy)).unwrap();
        });
        // The consumer has started and is probably waiting. Correctness must not
        // depend on that: publish stores the value before it notifies.
        started_rx.recv().unwrap();
        publish(&slot, 42);
        let got = done_rx.recv_timeout(Duration::from_secs(10));
        assert_eq!(got, Ok(42), "the waiting consumer was never woken");
        consumer.join().unwrap();
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    #[test]
    fn p_predictions() {
        // P1. One read guard is held. Can a second reader get in? Can a writer?
        let lock = RwLock::new(0);
        let first = lock.read().unwrap();
        let reader_ok = lock.try_read().is_ok();
        let writer_ok = lock.try_write().is_ok();
        drop(first);
        let p1: (bool, bool) = guess();
        assert_eq!(p1, (reader_ok, writer_ok), "P1: shared versus exclusive");

        // P2. notify_all happens BEFORE anyone waits. The waiter then waits up to
        // 20 ms for `ready` to become true. Does its wait time out?
        let pair = (Mutex::new(false), Condvar::new());
        pair.1.notify_all();
        let (_guard, result) = pair
            .1
            .wait_timeout_while(pair.0.lock().unwrap(), Duration::from_millis(20), |ready| {
                !*ready
            })
            .unwrap();
        let p2: bool = guess();
        assert_eq!(p2, result.timed_out(), "P2: is a notification stored?");

        // P3. Three threads meet at a Barrier. How many are told they are the leader?
        let barrier = Barrier::new(3);
        let leaders = thread::scope(|s| {
            let handles: Vec<_> = (0..3)
                .map(|_| s.spawn(|| barrier.wait().is_leader()))
                .collect();
            handles
                .into_iter()
                .map(|h| h.join().unwrap())
                .filter(|&leader| leader)
                .count()
        });
        let p3: usize = guess();
        assert_eq!(p3, leaders, "P3: barrier leaders");
    }
}
