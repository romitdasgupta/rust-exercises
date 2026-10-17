//! LESSON 03 — Mapping, filtering, and flattening
//! Run: cargo test --test chapter15 l03_
//!
//! PREDICT before executing: Does filter receive an owned item or a reference to it? How do
//! filter_map and flat_map differ?
//!
//! IMPLEMENT the documented tasks, then keep the tests and add an edge case.
//! EXPLAIN afterward: What values vanish in each pipeline? Why does flatten on Option
//! discard None?
//!
//! COMPILER / MANUAL LAB (use examples/ch15_scratch.rs):
//! Build a lazy map that increments a Cell counter. Inspect the counter before consumption,
//! after next(), and after collect(). Compare flattening nested vectors by iter versus
//! into_iter.
//!
//! TRANSFER: invent a new edge case, predict it, then add a check. A lesson is
//! complete only when its tests, predictions, lab, and explanation are done.

/// A. Trim, parse i32, retain nonnegative values, then widen and square into i64.
fn nonnegative_squares(tokens: &[&str]) -> Vec<i64> {
    todo!("Task 1: implement the documented contract")
}
/// B. Flatten whitespace-separated words from multiple lines into owned strings.
/// Preserve line/word order and duplicates; empty lines contribute no words.
fn all_words(lines: &[&str]) -> Vec<String> {
    todo!("Task 2: implement the documented contract")
}
/// C. Flatten nested optional integers, discarding None only.
fn present(groups: Vec<Vec<Option<i32>>>) -> Vec<i32> {
    todo!("Task 3: implement the documented contract")
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a_filter_map_distinguishes_invalid_negative_and_zero() {
        assert_eq!(
            nonnegative_squares(&[" 3 ", "x", "-2", "0", "2147483647"]),
            [9, 0, 4_611_686_014_132_420_609]
        );
    }
    #[test]
    fn b_flat_map_preserves_order() {
        assert_eq!(all_words(&[" a\tb ", "", "🦀 a"]), ["a", "b", "🦀", "a"]);
    }
    #[test]
    fn c_flatten_removes_only_missing_layers() {
        assert_eq!(
            present(vec![vec![Some(0), None], vec![], vec![Some(-1)]]),
            [0, -1]
        );
    }
}
