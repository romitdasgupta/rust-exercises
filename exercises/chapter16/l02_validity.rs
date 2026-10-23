//! LESSON 02 — Growth, removal, and borrow validity
//! Run: cargo test --test chapter16 l02_
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider: Can a reference to v[0] survive v.push(...) while used
//! afterward? Is unused reserved capacity enough to convince the borrow checker?
//!
//! IMPLEMENT the documented tasks, then keep the tests and add an edge case.
//! EXPLAIN afterward: Why does Rust reject potential invalidation without proving whether
//! this particular push reallocates? How does splitting prove disjoint mutable access?
//!
//! COMPILER / MANUAL LAB (use examples/ch16_scratch.rs):
//! Try let first=&v[0]; v.push(4); println!("{first}"); even after reserve. Repair by
//! copying the element, by ending the borrow, and by retaining an index then reborrowing.
//! Try holding an iterator while inserting. Compare all diagnostics.
//!
//! TRANSFER: invent a new edge case, predict it, then add a check. A lesson is
//! complete only when its tests, predictions, lab, and explanation are done.

/// A. Swap arbitrary valid indices using disjoint mutable slices, not unsafe.
/// Equal indices leave data unchanged.
fn swap_disjoint(values: &mut [i32], a: usize, b: usize) {
    todo!("Task 1: implement the documented contract")
}
/// B. Retain only nonnegative entries, keeping their relative order.
/// Edit in place: no new vector.
fn retain_nonnegative(values: &mut Vec<i32>) {
    todo!("Task 2: implement the documented contract")
}
/// C. Remove a valid index without preserving order, returning removed value.
fn fast_remove(values: &mut Vec<i32>, index: usize) -> i32 {
    todo!("Task 3: implement the documented contract")
}
#[cfg(test)]
mod tests {
    use super::*;
    use practice::{alloc::count, guess};
    #[test]
    fn a_split_mutability_is_disjoint() {
        let mut a = [1, 2, 3, 4];
        swap_disjoint(&mut a, 3, 1);
        assert_eq!(a, [1, 4, 3, 2]);
        swap_disjoint(&mut a, 0, 0);
        assert_eq!(a, [1, 4, 3, 2]);
        swap_disjoint(&mut a, 0, 3);
        assert_eq!(a, [2, 4, 3, 1]);
        swap_disjoint(&mut a, 1, 2);
        assert_eq!(a, [2, 3, 4, 1]);
    }
    #[test]
    fn b_retain_is_stable() {
        let mut a = vec![-1, 3, 0, -2, 3];
        retain_nonnegative(&mut a);
        assert_eq!(a, [3, 0, 3]);
        let mut none = vec![-5, -1];
        retain_nonnegative(&mut none);
        assert!(none.is_empty());
    }
    #[test]
    fn b_retain_edits_in_place() {
        let mut a: Vec<i32> = (-50..50).collect();
        let p = a.as_ptr();
        let (_, allocations) = count(|| retain_nonnegative(&mut a));
        assert_eq!((a.len(), a[0], a.as_ptr()), (50, 0, p));
        assert_eq!(allocations, 0, "retain needs no second vector");
    }
    #[test]
    fn c_swap_remove_fills_the_hole_with_last() {
        let mut a = vec![10, 20, 30, 40];
        assert_eq!(fast_remove(&mut a, 1), 20);
        assert_eq!(a, [10, 40, 30]);
        assert_eq!(fast_remove(&mut a, 2), 30);
        assert_eq!(a, [10, 40]);
        assert_eq!(fast_remove(&mut a, 0), 10);
        assert_eq!(a, [40]);
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    #[test]
    fn p_predictions() {
        // P1. drain removes a range and hands it back. What is drained, and what stays?
        let mut v = vec![1, 2, 3, 4, 5];
        let drained: Vec<i32> = v.drain(1..3).collect();
        let p1: (Vec<i32>, Vec<i32>) = guess();
        assert_eq!(p1, (drained, v), "P1: drain");

        // P2. Pushing within reserved capacity: does the buffer move?
        let mut v = Vec::with_capacity(4);
        v.push(1);
        let before = v.as_ptr();
        v.push(2);
        let p2: bool = guess(); // true if the buffer stayed put
        assert_eq!(p2, v.as_ptr() == before, "P2: push within capacity");

        // P3. clear() on 100 elements: the length, and whether capacity >= 100 remains.
        let mut v = vec![0u8; 100];
        v.clear();
        let p3: (usize, bool) = guess();
        assert_eq!(
            p3,
            (v.len(), v.capacity() >= 100),
            "P3: clear keeps the buffer"
        );
    }
}
