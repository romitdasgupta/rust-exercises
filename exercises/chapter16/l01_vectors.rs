//! LESSON 01 — Vector access and structural editing
//! Run: cargo test --test chapter16 l01_
//!
//! PREDICT before executing: Does reserve add elements? Which of remove and swap_remove
//! preserves order? What does split_off leave behind?
//!
//! IMPLEMENT the documented tasks, then keep the tests and add an edge case.
//! EXPLAIN afterward: Explain length versus capacity and how each selected operation
//! changes both ownership and ordering.
//!
//! COMPILER / MANUAL LAB (use examples/ch16_scratch.rs):
//! Build Vec::with_capacity(8), record len/capacity, push three values, reserve(20),
//! shrink_to_fit, and record observations. Assert only capacity>=len; do not assume a
//! growth factor or that shrinking is exact.
//!
//! TRANSFER: invent a new edge case, predict it, then add a check. A lesson is
//! complete only when its tests, predictions, lab, and explanation are done.

/// A. Checked copied access; no panic on an invalid index.
fn at(values: &[i32], index: usize) -> Option<i32> {
    todo!("Task 1: implement the documented contract")
}
/// B. Split at a valid index, reverse the owned tail, return (head,tail).
fn split_tail(mut values: Vec<i32>, index: usize) -> (Vec<i32>, Vec<i32>) {
    todo!("Task 2: implement the documented contract")
}
/// C. On nonempty data swap first and last, then fill the interior with zero.
/// Lengths zero, one, and two have no interior.
fn border_swap(values: &mut [i32]) {
    todo!("Task 3: implement the documented contract")
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a_checked_access_handles_end_and_empty() {
        assert_eq!(at(&[2, 4], 1), Some(4));
        assert_eq!(at(&[2, 4], 2), None);
        assert_eq!(at(&[], usize::MAX), None);
    }
    #[test]
    fn b_split_off_moves_an_independent_tail() {
        assert_eq!(split_tail(vec![1, 2, 3, 4], 2), (vec![1, 2], vec![4, 3]));
        assert_eq!(split_tail(vec![1], 0), (vec![], vec![1]));
    }
    #[test]
    fn c_swapping_and_filling_leave_length_unchanged() {
        let mut a = [1, 2, 3, 4, 5];
        border_swap(&mut a);
        assert_eq!(a, [5, 0, 0, 0, 1]);
        let mut b = [9];
        border_swap(&mut b);
        assert_eq!(b, [9]);
        border_swap(&mut []);
    }
}
