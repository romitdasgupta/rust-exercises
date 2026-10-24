//! LESSON 04 — Send and Sync contracts
//! Run: cargo test --test chapter19 l04_
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider:
//! 1. Is Arc<RefCell<T>> automatically thread safe?
//! 2. Is &T Send when T is Sync?
//!
//! IMPLEMENT the tasks; keep the tests.
//! EXPLAIN: Explain auto traits, ownership transfer, and why a mutex does not make every inner value Send.
//! COMPILER LAB (examples/ch19_scratch.rs): Write require_send::<Rc<u8>>() and require_sync::<RefCell<u8>>() in scratch; record diagnostics. Repair using Arc and Mutex; do not add unsafe impl.

use std::{
    sync::{Arc, Mutex},
    thread,
};
fn require_send<T: Send>() {}
fn require_sync<T: Sync>() {}
/// A. Demonstrate the appropriate bounds for u32, Arc<Vec<u8>>, Mutex<Vec<u8>>,
/// and &str by calling both bound-checking helpers for each. Return "checked".
fn check_bounds() -> &'static str {
    todo!("A: implement the contract")
}
/// B. Share one allocation between two readers; return their observed lengths
/// and the original Arc (which must remain the same allocation).
fn shared_readers(data: Arc<Vec<u8>>) -> (usize, usize, Arc<Vec<u8>>) {
    todo!("B: implement the contract")
}

#[cfg(test)]
mod tests {
    use super::*;
    use practice::guess;
    #[test]
    fn a_allowed_types_satisfy_bounds() {
        assert_eq!(check_bounds(), "checked");
    }
    #[test]
    fn b_readers_share_one_allocation() {
        let a = Arc::new(vec![1, 2, 3]);
        let (x, y, returned) = shared_readers(Arc::clone(&a));
        assert_eq!((x, y), (3, 3));
        assert!(Arc::ptr_eq(&a, &returned));
        assert_eq!(Arc::strong_count(&a), 2);
    }
    #[test]
    fn b_readers_release_their_clones() {
        let (x, y, returned) = shared_readers(Arc::new(Vec::new()));
        assert_eq!((x, y), (0, 0));
        assert_eq!(
            Arc::strong_count(&returned),
            1,
            "join the readers so their clones are dropped before returning"
        );
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    #[test]
    fn p_predictions() {
        let data = Arc::new(vec![7u8; 3]);

        // P1. Strong count seen inside the worker, which owns one clone.
        let clone = Arc::clone(&data);
        let inside = thread::spawn(move || Arc::strong_count(&clone))
            .join()
            .unwrap();
        // P2. Strong count after the worker has been joined.
        let after = Arc::strong_count(&data);
        let p1: usize = guess();
        let p2: usize = guess();
        assert_eq!(p1, inside, "P1: owners while the worker runs");
        assert_eq!(p2, after, "P2: who dropped the worker's clone?");

        // P3. Arc::try_unwrap returns the Vec only for the last owner. Does the
        // first attempt fail, and does the second (after `other` is dropped) succeed?
        let other = Arc::clone(&data);
        let attempt = Arc::try_unwrap(data);
        let first_failed = attempt.is_err();
        let data = attempt.unwrap_err();
        drop(other);
        let second_ok = Arc::try_unwrap(data).is_ok();
        let p3: (bool, bool) = guess();
        assert_eq!(p3, (first_failed, second_ok), "P3: Arc::try_unwrap");
    }
}
