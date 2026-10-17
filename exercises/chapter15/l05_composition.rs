//! LESSON 05 — Reversal, pairing, inspection, and repetition
//! Run: cargo test --test chapter15 l05_
//!
//! PREDICT before executing: What happens when zip inputs have different lengths? Does
//! enumerate after rev preserve original indices? Can any iterator be reversed?
//!
//! IMPLEMENT the documented tasks, then keep the tests and add an edge case.
//! EXPLAIN afterward: Which adapters require DoubleEndedIterator or Clone? Why is inspect
//! useful for observing a lazy pipeline?
//!
//! COMPILER / MANUAL LAB (use examples/ch15_scratch.rs):
//! Compare enumerate().rev() with rev().enumerate() on a slice. Build an inspect counter
//! and show it runs only for consumed items. Try rev on an unbounded range and explain the
//! missing trait bound.
//!
//! TRANSFER: invent a new edge case, predict it, then add a check. A lesson is
//! complete only when its tests, predictions, lab, and explanation are done.

/// A. Return original (index,owned value) pairs in reverse order.
fn reverse_numbered(words: &[String]) -> Vec<(usize, String)> {
    todo!("Task 1: implement the documented contract")
}
/// B. Pair each input number with cyclic palette colors; empty palette yields empty.
fn colorize(values: &[i32], palette: &[&str]) -> Vec<(i32, String)> {
    todo!("Task 2: implement the documented contract")
}
/// C. Chain both slices, copy values, inspect each consumed item by incrementing
/// visits, and collect only the first limit items. Return (output,visits).
fn observed_prefix(left: &[i32], right: &[i32], limit: usize) -> (Vec<i32>, usize) {
    todo!("Task 3: implement the documented contract")
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a_enumeration_precedes_reversal() {
        assert_eq!(
            reverse_numbered(&["a".into(), "b".into(), "c".into()]),
            [(2, "c".into()), (1, "b".into()), (0, "a".into())]
        );
    }
    #[test]
    fn b_zip_and_cycle_pair_only_available_items() {
        assert_eq!(
            colorize(&[1, 2, 3], &["r", "b"]),
            [(1, "r".into()), (2, "b".into()), (3, "r".into())]
        );
        assert!(colorize(&[1], &[]).is_empty());
    }
    #[test]
    fn c_inspect_counts_only_requested_items() {
        assert_eq!(observed_prefix(&[1, 2], &[3, 4], 3), (vec![1, 2, 3], 3));
        assert_eq!(observed_prefix(&[1], &[2], 0), (vec![], 0));
        assert_eq!(observed_prefix(&[], &[7], 9), (vec![7], 1));
    }
}
