//! LESSON 02 — Growth, removal, and borrow validity
//! Run: cargo test --test chapter16 l02_
//!
//! PREDICT before executing: Can a reference to v[0] survive v.push(...) while used
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
    #[test]
    fn a_split_mutability_is_disjoint() {
        let mut a = [1, 2, 3, 4];
        swap_disjoint(&mut a, 3, 1);
        assert_eq!(a, [1, 4, 3, 2]);
        swap_disjoint(&mut a, 0, 0);
        assert_eq!(a, [1, 4, 3, 2]);
    }
    #[test]
    fn b_retain_is_stable() {
        let mut a = vec![-1, 3, 0, -2, 3];
        retain_nonnegative(&mut a);
        assert_eq!(a, [3, 0, 3]);
    }
    #[test]
    fn c_swap_remove_fills_the_hole_with_last() {
        let mut a = vec![10, 20, 30, 40];
        assert_eq!(fast_remove(&mut a, 1), 20);
        assert_eq!(a, [10, 40, 30]);
        assert_eq!(fast_remove(&mut a, 2), 30);
    }
}
