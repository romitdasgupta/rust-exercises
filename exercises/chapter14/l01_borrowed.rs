//! LESSON 01 — Borrowed captures and sorting
//! Run: cargo test --test chapter14 l01_
//!
//! PREDICT before executing: Can a closure borrow a lookup table while sorting another
//! vector? Does sort_by_key move the lookup table?
//!
//! IMPLEMENT the documented tasks, then keep the tests and add an edge case.
//! EXPLAIN afterward: Why is the captured prefix shared rather than mutable? When is the
//! borrow released?
//!
//! COMPILER / MANUAL LAB (use examples/ch14_scratch.rs):
//! Create a closure borrowing a String; try mutating the String before a later closure
//! call. Remove that later call and observe non-lexical lifetime analysis. Explain why
//! capture precision can borrow only one struct field.
//!
//! TRANSFER: invent a new edge case, predict it, then add a check. A lesson is
//! complete only when its tests, predictions, lab, and explanation are done.

/// A. Keep words starting with prefix, preserving order and duplicates.
/// Use a closure capturing prefix and return owned copies.
fn matching(words: &[&str], prefix: &str) -> Vec<String> {
    todo!("Task 1: implement the documented contract")
}
/// B. Sort by UTF-8 byte length, then lexical text. Mutate only the vector order.
fn shortest_first(words: &mut [String]) {
    todo!("Task 2: implement the documented contract")
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a_closure_borrows_the_prefix() {
        assert_eq!(matching(&["air", "sea", "air", "α"], "a"), ["air", "air"]);
        assert_eq!(matching(&["a", ""], ""), ["a", ""]);
        assert!(matching(&[], "x").is_empty());
    }
    #[test]
    fn b_sorting_closure_has_a_deterministic_tie_break() {
        let mut v = vec!["zz".into(), "é".into(), "a".into(), "bb".into()];
        shortest_first(&mut v);
        assert_eq!(v, ["a", "bb", "zz", "é"]);
    }
}
