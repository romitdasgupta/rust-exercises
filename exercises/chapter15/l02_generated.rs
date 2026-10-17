//! LESSON 02 — Generating finite sequences
//! Run: cargo test --test chapter15 l02_
//!
//! PREDICT before executing: When does a from_fn closure execute? Does successors include
//! its initial value before invoking its step?
//!
//! IMPLEMENT the documented tasks, then keep the tests and add an edge case.
//! EXPLAIN afterward: How do you guarantee termination and avoid overflow at the final step?
//!
//! COMPILER / MANUAL LAB (use examples/ch15_scratch.rs):
//! Generate 5 values with std::iter::repeat_n and once().chain(...). Create an
//! intentionally infinite repeat/cycle and demonstrate that take bounds the work; never
//! collect an unbounded iterator.
//!
//! TRANSFER: invent a new edge case, predict it, then add a check. A lesson is
//! complete only when its tests, predictions, lab, and explanation are done.

/// A. Return powers of two <= limit, starting at 1. limit=0 returns empty.
/// Use successors and checked multiplication, including u32::MAX safely.
fn powers(limit: u32) -> Vec<u32> {
    todo!("Task 1: implement the documented contract")
}
/// B. Use from_fn to yield start,start-1,...,0 exactly once each.
fn countdown(start: u8) -> impl Iterator<Item = u8> {
    let mut current = Some(start);
    std::iter::from_fn(move || -> Option<u8> { todo!("Task 2: implement the documented contract") })
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a_successors_stops_at_limit_or_overflow() {
        assert!(powers(0).is_empty());
        assert_eq!(powers(10), [1, 2, 4, 8]);
        assert_eq!(powers(1), [1]);
        let p = powers(u32::MAX);
        assert_eq!(p.len(), 32);
        assert_eq!(p.last(), Some(&(1 << 31)));
    }
    #[test]
    fn b_from_fn_includes_zero_and_stays_exhausted() {
        assert_eq!(countdown(3).collect::<Vec<_>>(), [3, 2, 1, 0]);
        let mut i = countdown(0);
        assert_eq!(i.next(), Some(0));
        assert_eq!(i.next(), None);
        assert_eq!(i.next(), None);
    }
}
