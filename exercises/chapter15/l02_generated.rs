//! LESSON 02 — Generating finite sequences
//! Run: cargo test --test chapter15 l02_
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider: When does a from_fn closure execute? Does
//! successors include its initial value before invoking its step?
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
    use practice::guess;
    use std::cell::Cell;
    #[test]
    fn a_successors_stops_at_limit_or_overflow() {
        assert!(powers(0).is_empty());
        assert_eq!(powers(10), [1, 2, 4, 8]);
        assert_eq!(powers(1), [1]);
        assert_eq!(powers(3), [1, 2]);
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
        assert_eq!(countdown(u8::MAX).count(), 256);
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    /// A wrong prediction is useful: read the failure, then explain it in NOTES.md.
    #[test]
    fn p_predictions() {
        // P1. Calls to the from_fn closure: (before anything consumes, after take(3)).
        let calls = Cell::new(0);
        let mut naturals = std::iter::from_fn(|| {
            calls.set(calls.get() + 1);
            Some(calls.get())
        });
        let before = calls.get();
        let first: Vec<u32> = naturals.by_ref().take(3).collect();
        let p1: (u32, u32) = guess();
        assert_eq!(p1, (before, calls.get()), "P1: generators are lazy");
        assert_eq!(first, [1, 2, 3]);

        // P2. Powers of 3 in u8, stopping at overflow: (how many, last value).
        let powers: Vec<u8> = std::iter::successors(Some(1u8), |&x| x.checked_mul(3)).collect();
        let p2: (usize, u8) = guess();
        assert_eq!(
            p2,
            (powers.len(), powers[powers.len() - 1]),
            "P2: successors stops at None"
        );

        // P3. Calls to the successors step: (after creation, after the first next()).
        let steps = Cell::new(0);
        let mut doubling = std::iter::successors(Some(1u32), |&x| {
            steps.set(steps.get() + 1);
            x.checked_mul(2)
        });
        let created = steps.get();
        doubling.next();
        let p3: (u32, u32) = guess();
        assert_eq!(
            p3,
            (created, steps.get()),
            "P3: successors computes the next item when it yields one"
        );
    }
}
