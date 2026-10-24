//! LESSON 10 — Implementing a double-ended iterator
//! Run: cargo test --test chapter15 l10_
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider: Which invariant relates next, next_back,
//! len, and size_hint? What happens when the two ends meet?
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
    use practice::guess;
    #[test]
    fn a_next_is_fused_and_constructor_handles_reversed_bounds() {
        assert_eq!(Counter::new(2, 5).collect::<Vec<_>>(), [2, 3, 4]);
        let mut c = Counter::new(5, 2);
        assert_eq!(c.next(), None);
        assert_eq!(c.next(), None);
        assert_eq!(
            Counter::new(u32::MAX - 1, u32::MAX).collect::<Vec<_>>(),
            [u32::MAX - 1]
        );
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
        assert_eq!(Counter::new(0, u32::MAX).len(), u32::MAX as usize);
        let mut empty = Counter::new(3, 3);
        assert_eq!(empty.size_hint(), (0, Some(0)));
        empty.next();
        empty.next_back();
        assert_eq!(empty.size_hint(), (0, Some(0)));
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
        let mut c = Counter::new(0, 2);
        assert_eq!(
            (c.next_back(), c.next_back(), c.next()),
            (Some(1), Some(0), None)
        );
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    /// A wrong prediction is useful: read the failure, then explain it in NOTES.md.
    #[test]
    fn p_predictions() {
        // P1. size_hint of a from_fn source, and of 0..10 after filter.
        let mut n = 0;
        let source = std::iter::from_fn(move || {
            n += 1;
            (n < 4).then_some(n)
        });
        let evens = (0..10).filter(|x| x % 2 == 0);
        let p1: ((usize, Option<usize>), (usize, Option<usize>)) = guess();
        assert_eq!(
            p1,
            (source.size_hint(), evens.size_hint()),
            "P1: hints are bounds, not counts"
        );

        // P2. Taking from both ends of 1..4.
        let mut r = 1..4;
        let taken = (r.next(), r.next_back(), r.next(), r.next_back());
        let p2: (Option<i32>, Option<i32>, Option<i32>, Option<i32>) = guess();
        assert_eq!(p2, taken, "P2: the ends meet in the middle");

        // P3. size_hint of (0..3).chain(10..12).
        let p3: (usize, Option<usize>) = guess();
        assert_eq!(
            p3,
            (0..3).chain(10..12).size_hint(),
            "P3: chain adds its parts' hints"
        );
    }
}
