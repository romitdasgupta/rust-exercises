//! LESSON 03 — Deref, DerefMut, and Default
//! Run: cargo test --test chapter13 l03_
//!
//! PREDICT before executing: Does Deref transfer ownership? Can deref coercion change
//! &Wrapper into &[i32]? Is Default necessarily all zero bytes?
//!
//! IMPLEMENT the documented tasks, then keep the tests and add an edge case.
//! EXPLAIN afterward: When is a transparent wrapper a suitable Deref design? Why should a
//! domain object avoid surprising deref-based behavior?
//!
//! COMPILER / MANUAL LAB (use examples/ch13_scratch.rs):
//! Try moving a Vec out through *wrapper and inspect the error. Then add a constructor
//! requiring an invariant and explain why blindly deriving Default could break it.
//!
//! TRANSFER: invent a new edge case, predict it, then add a check. A lesson is
//! complete only when its tests, predictions, lab, and explanation are done.

use std::ops::{Deref, DerefMut};
#[derive(Debug, PartialEq, Eq)]
struct Samples(Vec<i32>);
impl Deref for Samples {
    type Target = [i32];
    /// A. Borrow all samples as a slice.
    fn deref(&self) -> &[i32] {
        todo!("Task 1: implement the documented contract")
    }
}
impl DerefMut for Samples {
    /// B. Borrow all samples mutably.
    fn deref_mut(&mut self) -> &mut [i32] {
        todo!("Task 2: implement the documented contract")
    }
}
impl Default for Samples {
    /// C. The meaningful default is an empty sample sequence.
    fn default() -> Self {
        todo!("Task 3: implement the documented contract")
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn first(xs: &[i32]) -> Option<i32> {
        xs.first().copied()
    }
    fn sort(xs: &mut [i32]) {
        xs.sort();
    }
    #[test]
    fn a_deref_coercion_and_method_lookup_work() {
        let s = Samples(vec![4, 2]);
        assert_eq!(first(&s), Some(4));
        assert_eq!(s.len(), 2);
    }
    #[test]
    fn b_mutable_coercion_edits_storage() {
        let mut s = Samples(vec![3, 1, 2]);
        sort(&mut s);
        s[0] = 8;
        assert_eq!(s.0, [8, 2, 3]);
    }
    #[test]
    fn c_default_has_a_meaningful_empty_state() {
        assert_eq!(Samples::default(), Samples(vec![]));
    }
}
