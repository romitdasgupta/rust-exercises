//! LESSON 06 — Moving values and initialization
//! Run: cargo test --test chapter23 l06_
//!
//! PREDICT before running:
//! 1. Does ptr::read leave its source logically owned?
//! 2. Can a moved-from String be dropped a second time?
//!
//! IMPLEMENT the tasks; keep the tests.
//! EXPLAIN: Explain why no panicking code may occur between a raw ownership move and updating initialized-length bookkeeping.
//! COMPILER LAB (examples/ch23_scratch.rs): Write a raw-move ownership ledger on paper. Do not run a double-drop experiment. Compare Vec::pop to your implementation.

use std::mem::MaybeUninit;
/// A. Implement pop without Vec::pop: reduce the vector length and ptr::read
/// its former last initialized element. Empty returns None. Never expose a
/// reference to the now-uninitialized former slot or run fallible code mid-move.
fn raw_pop<T>(values: &mut Vec<T>) -> Option<T> {
    todo!("A: implement the contract")
}
/// B. Move two owned Strings into MaybeUninit slots, then return a fully
/// initialized array. No default placeholders, clones, or raw byte copies.
fn initialized_pair(first: String, second: String) -> [String; 2] {
    todo!("B: implement the contract")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn moved_values_drop_once() {
        use std::sync::{
            Arc,
            atomic::{AtomicUsize, Ordering},
        };
        struct Spy(Arc<AtomicUsize>);
        impl Drop for Spy {
            fn drop(&mut self) {
                self.0.fetch_add(1, Ordering::Relaxed);
            }
        }
        let count = Arc::new(AtomicUsize::new(0));
        let mut v = vec![Spy(Arc::clone(&count)), Spy(Arc::clone(&count))];
        let last = raw_pop(&mut v).unwrap();
        assert_eq!(v.len(), 1);
        drop(v);
        assert_eq!(count.load(Ordering::Relaxed), 1);
        drop(last);
        assert_eq!(count.load(Ordering::Relaxed), 2);
        assert!(raw_pop::<String>(&mut vec![]).is_none());
    }
    #[test]
    fn initialized_values_are_owned_by_returned_array() {
        assert_eq!(initialized_pair("a".into(), "β".into()), ["a", "β"]);
    }
}
