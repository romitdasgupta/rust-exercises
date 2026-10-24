//! LESSON 03 — Mapping, filtering, and flattening
//! Run: cargo test --test chapter15 l03_
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider: Does filter receive an owned item or a
//! reference to it? How do filter_map and flat_map differ?
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
    use practice::guess;
    use std::cell::Cell;
    #[test]
    fn a_filter_map_distinguishes_invalid_negative_and_zero() {
        assert_eq!(
            nonnegative_squares(&[" 3 ", "x", "-2", "0", "2147483647"]),
            [9, 0, 4_611_686_014_132_420_609]
        );
        assert!(nonnegative_squares(&[]).is_empty());
        assert_eq!(nonnegative_squares(&["+4", "1.0", "-0"]), [16, 0]);
    }
    #[test]
    fn b_flat_map_preserves_order() {
        assert_eq!(all_words(&[" a\tb ", "", "🦀 a"]), ["a", "b", "🦀", "a"]);
        assert!(all_words(&["  ", "\n"]).is_empty());
    }
    #[test]
    fn c_flatten_removes_only_missing_layers() {
        assert_eq!(
            present(vec![vec![Some(0), None], vec![], vec![Some(-1)]]),
            [0, -1]
        );
        assert!(present(vec![vec![None], vec![]]).is_empty());
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    /// A wrong prediction is useful: read the failure, then explain it in NOTES.md.
    #[test]
    fn p_predictions() {
        // P1. Calls to map's closure: (before consuming, after one next(), after count()).
        let calls = Cell::new(0);
        let mut doubled = [1, 2, 3].iter().map(|x| {
            calls.set(calls.get() + 1);
            x * 2
        });
        let before = calls.get();
        doubled.next();
        let after_next = calls.get();
        let rest = doubled.count();
        let p1: (u32, u32, u32) = guess();
        assert_eq!(
            p1,
            (before, after_next, calls.get()),
            "P1: adapters run on demand"
        );
        assert_eq!(rest, 2);

        // P2. Result implements IntoIterator too. What does flat_map keep?
        let parsed: Vec<i32> = ["1", "x", "3"]
            .iter()
            .flat_map(|s| s.parse::<i32>())
            .collect();
        let p2: Vec<i32> = guess();
        assert_eq!(p2, parsed, "P2: an Err yields no items, like None");

        // P3. Word counts for "a b  c" (two spaces): (split(' '), split_whitespace()).
        let text = "a b  c";
        let p3: (usize, usize) = guess();
        assert_eq!(
            p3,
            (text.split(' ').count(), text.split_whitespace().count()),
            "P3: split keeps empty pieces"
        );
    }
}
