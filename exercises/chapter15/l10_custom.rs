//! LESSON 10 — Implementing a double-ended iterator
//! Run: cargo test --test chapter15 l10_
//!
//! PREDICT before executing: Which invariant relates next, next_back, len, and size_hint?
//! What happens when the two ends meet?
//!
//! IMPLEMENT the documented tasks, then keep the tests and add an edge case.
//! EXPLAIN afterward: Why does ExactSizeIterator promise more than a useful hint? Why
//! should an empty iterator remain empty?
//!
//! COMPILER / MANUAL LAB (use examples/ch15_scratch.rs):
//! Add IntoIterator for &your_container returning borrowed items. Separately implement
//! FromIterator for a wrapper. Explain why implementing IntoIterator for an Iterator can
//! conflict with the blanket implementation.
//!
//! TRANSFER: invent a new edge case, predict it, then add a check. A lesson is
//! complete only when its tests, predictions, lab, and explanation are done.

/// Inclusive lower bound, exclusive upper bound; constructor normalizes reversed ranges.
#[derive(Debug, Clone)]
struct Counter {
    front: u32,
    back: u32,
}
impl Counter {
    fn new(start: u32, end: u32) -> Self {
        Self {
            front: start.min(end),
            back: end,
        }
    }
}
impl Iterator for Counter {
    type Item = u32;
    /// A. Yield from front, or None once empty.
    fn next(&mut self) -> Option<u32> {
        todo!("Task 1: implement the documented contract")
    }
    /// B. Exact lower and upper bounds on the remaining number of values.
    fn size_hint(&self) -> (usize, Option<usize>) {
        todo!("Task 2: implement the documented contract")
    }
}
impl DoubleEndedIterator for Counter {
    /// C. Yield from the back without duplicating front-consumed values.
    fn next_back(&mut self) -> Option<u32> {
        todo!("Task 3: implement the documented contract")
    }
}
impl ExactSizeIterator for Counter {}
impl std::iter::FusedIterator for Counter {}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a_next_is_fused_and_constructor_handles_reversed_bounds() {
        assert_eq!(Counter::new(2, 5).collect::<Vec<_>>(), [2, 3, 4]);
        let mut c = Counter::new(5, 2);
        assert_eq!(c.next(), None);
        assert_eq!(c.next(), None);
    }
    #[test]
    fn b_size_tracks_remaining_work() {
        let mut c = Counter::new(2, 6);
        assert_eq!(c.size_hint(), (4, Some(4)));
        assert_eq!(c.len(), 4);
        c.next();
        assert_eq!(c.len(), 3);
        c.next_back();
        assert_eq!(c.len(), 2);
    }
    #[test]
    fn c_both_ends_share_one_remaining_interval() {
        let mut c = Counter::new(10, 13);
        assert_eq!(c.next(), Some(10));
        assert_eq!(c.next_back(), Some(12));
        assert_eq!(c.next_back(), Some(11));
        assert_eq!(c.next(), None);
        assert_eq!(c.next_back(), None);
        assert_eq!(Counter::new(1, 4).rev().collect::<Vec<_>>(), [3, 2, 1]);
    }
}
