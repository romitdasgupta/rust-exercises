//! LESSON 03 — Sorting, binary search, and slices
//! Run: cargo test --test chapter16 l03_
//!
//! PREDICT before executing: Does binary_search choose the first duplicate? Can
//! binary_search be used on an unsorted slice?
//!
//! IMPLEMENT the documented tasks, then keep the tests and add an edge case.
//! EXPLAIN afterward: Why does stable sort preserve equal-key ordering? When can an
//! unstable sort be preferable?
//!
//! COMPILER / MANUAL LAB (use examples/ch16_scratch.rs):
//! Compare sort_by_key and sort_unstable_by_key on equal keys: describe the guarantee
//! rather than assuming one observed permutation. Test slice equality and lexical
//! comparison on unequal lengths. Add binary_search_by_key for records.
//!
//! TRANSFER: invent a new edge case, predict it, then add a check. A lesson is
//! complete only when its tests, predictions, lab, and explanation are done.

/// A. Stable ascending score order; retain original order for equal scores.
fn rank(records: &mut [(i32, String)]) {
    todo!("Task 1: implement the documented contract")
}
/// B. On sorted input, return the half-open range containing all target matches.
/// If absent, return the empty range at the insertion point.
fn equal_range(sorted: &[i32], target: i32) -> std::ops::Range<usize> {
    todo!("Task 2: implement the documented contract")
}
/// C. Sort and remove duplicates in place, preserving ascending order.
fn unique_sorted(values: &mut Vec<i32>) {
    todo!("Task 3: implement the documented contract")
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a_stable_sort_preserves_equal_score_input_order() {
        let mut r = vec![(2, "x".into()), (1, "b".into()), (1, "a".into())];
        rank(&mut r);
        assert_eq!(r, [(1, "b".into()), (1, "a".into()), (2, "x".into())]);
    }
    #[test]
    fn b_duplicate_boundaries_are_deterministic() {
        let a = [1, 2, 2, 2, 4];
        assert_eq!(equal_range(&a, 2), 1..4);
        assert_eq!(equal_range(&a, 3), 4..4);
        assert_eq!(equal_range(&[], 9), 0..0);
    }
    #[test]
    fn c_dedup_requires_sorting_for_global_uniqueness() {
        let mut a = vec![3, 1, 3, 2, 1];
        unique_sorted(&mut a);
        assert_eq!(a, [1, 2, 3]);
    }
}
