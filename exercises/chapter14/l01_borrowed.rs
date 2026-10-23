//! LESSON 01 — Borrowed captures and sorting
//! Run: cargo test --test chapter14 l01_
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider: Can a closure borrow a lookup table while
//! sorting another vector? Does sort_by_key move the lookup table?
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
/// B. Sort by UTF-8 byte length, then lexical text. Mutate only the vector order:
/// the comparison closure borrows the strings and never clones them.
fn shortest_first(words: &mut [String]) {
    todo!("Task 2: implement the documented contract")
}
#[cfg(test)]
mod tests {
    use super::*;
    use practice::{alloc::count, guess};
    use std::mem::size_of_val;
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
        let mut empty: Vec<String> = vec![];
        shortest_first(&mut empty);
        assert!(empty.is_empty());
    }

    #[test]
    fn b_sorting_moves_strings_without_cloning() {
        let mut v: Vec<String> = (0..100).rev().map(|i| format!("w{i}")).collect();
        let first = v[99].as_ptr(); // "w0" sorts first
        let (_, allocations) = count(|| shortest_first(&mut v));
        assert_eq!(v[0], "w0");
        assert_eq!(
            v[0].as_ptr(),
            first,
            "sorting should move the String headers"
        );
        assert!(
            allocations <= 2,
            "sorting 100 words cost {allocations} allocations; borrow in the comparison"
        );
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    /// A wrong prediction is useful: read the failure, then explain it in NOTES.md.
    #[test]
    fn p_predictions() {
        // P1. A closure is a value holding its captures. How many bytes does each take?
        let table = String::from("lookup");
        let reads_table = || table.len();
        let captures_nothing = |n: usize| n + 1;
        let p1: (usize, usize) = guess();
        assert_eq!(
            p1,
            (size_of_val(&captures_nothing), size_of_val(&reads_table)),
            "P1: a shared-borrow capture is one pointer"
        );

        // P2. This closure reads two fields of `point`. Bytes of captured state?
        let point = (3i32, 4i32);
        let sum_fields = || point.0 + point.1;
        let p2: usize = guess();
        assert_eq!(
            p2,
            size_of_val(&sum_fields),
            "P2: closures capture places (fields), not whole variables"
        );
        assert_eq!(sum_fields(), 7);

        // P3. sort_by_key is stable. What order results?
        let mut words = vec!["bb", "a", "cc", "d"];
        words.sort_by_key(|w| w.len());
        let p3: Vec<&str> = guess();
        assert_eq!(p3, words, "P3: equal keys keep their input order");
    }
}
