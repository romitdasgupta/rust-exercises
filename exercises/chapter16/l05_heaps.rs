//! LESSON 05 — Priority queues and heap ordering
//! Run: cargo test --test chapter16 l05_
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider: Does BinaryHeap iteration produce sorted order? Is the default
//! heap a min-heap or a max-heap?
//!
//! IMPLEMENT the documented tasks, then keep the tests and add an edge case.
//! EXPLAIN afterward: Why should you pop to obtain priority order? How does Reverse adapt
//! ordering without writing a second collection?
//!
//! COMPILER / MANUAL LAB (use examples/ch16_scratch.rs):
//! Build a heap, compare iter(), into_vec(), and repeated pop(), recording only documented
//! guarantees. Use peek_mut to lower the top element and inspect reheapification after the
//! guard drops.
//!
//! TRANSFER: invent a new edge case, predict it, then add a check. A lesson is
//! complete only when its tests, predictions, lab, and explanation are done.

use std::cmp::Reverse;
use std::collections::BinaryHeap;
/// A. Extract the largest k numbers, descending; keep duplicates.
fn largest(values: &[i32], k: usize) -> Vec<i32> {
    todo!("Task 1: implement the documented contract")
}
/// B. Extract all numbers ascending using BinaryHeap<Reverse<i32>>.
fn smallest_first(values: &[i32]) -> Vec<i32> {
    todo!("Task 2: implement the documented contract")
}
#[cfg(test)]
mod tests {
    use super::*;
    use practice::guess;
    #[test]
    fn a_max_heap_pops_priority_order() {
        assert_eq!(largest(&[3, 9, 2, 9], 3), [9, 9, 3]);
        assert_eq!(largest(&[1], 8), [1]);
        assert!(largest(&[1], 0).is_empty());
        assert!(largest(&[], 3).is_empty());
        assert_eq!(largest(&[-5, -1, -3], 2), [-1, -3]);
    }
    #[test]
    fn b_reverse_turns_a_max_heap_into_min_priority() {
        assert_eq!(smallest_first(&[3, -1, 3, 0]), [-1, 0, 3, 3]);
        assert!(smallest_first(&[]).is_empty());
        assert_eq!(smallest_first(&[i32::MAX, i32::MIN]), [i32::MIN, i32::MAX]);
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    #[test]
    fn p_predictions() {
        let heap = BinaryHeap::from(vec![3, 1, 4, 1, 5]);
        // P1. What does peek see?
        let p1: Option<i32> = guess();
        assert_eq!(p1, heap.peek().copied(), "P1: default heap order");

        // P2. into_sorted_vec: which direction?
        let p2: Vec<i32> = guess();
        assert_eq!(p2, heap.into_sorted_vec(), "P2: into_sorted_vec");

        // P3. The first pop from a heap of Reverse values.
        let mut heap = BinaryHeap::from([Reverse(3), Reverse(1), Reverse(2)]);
        let p3: Option<i32> = guess();
        assert_eq!(p3, heap.pop().map(|Reverse(x)| x), "P3: Reverse");

        // P4. Tuples compare field by field. Which pops first?
        let mut heap = BinaryHeap::from([(2, "a"), (1, "z"), (2, "b")]);
        let p4: Option<(i32, &str)> = guess();
        assert_eq!(p4, heap.pop(), "P4: ties in a max-heap");
    }
}
