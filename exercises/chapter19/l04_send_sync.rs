//! LESSON 04 — Send and Sync contracts
//! Run: cargo test --test chapter19 l04_
//!
//! PREDICT before running:
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
    #[test]
    fn allowed_types_satisfy_bounds() {
        assert_eq!(check_bounds(), "checked");
    }
    #[test]
    fn readers_share_one_allocation() {
        let a = Arc::new(vec![1, 2, 3]);
        let (x, y, returned) = shared_readers(Arc::clone(&a));
        assert_eq!((x, y), (3, 3));
        assert!(Arc::ptr_eq(&a, &returned));
        assert_eq!(Arc::strong_count(&a), 2);
    }
}
