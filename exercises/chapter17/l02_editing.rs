//! LESSON 02 — Constructing and editing owned text
//! Run: cargo test --test chapter17 l02_
//!
//! PREDICT before executing: Which String operations take byte offsets? Is capacity the
//! number of characters that fit? Can pop remove only part of a UTF-8 scalar?
//!
//! IMPLEMENT the documented tasks, then keep the tests and add an edge case.
//! EXPLAIN afterward: Why must insertion boundaries be checked? Which operations keep the
//! original allocation versus return a new String?
//!
//! COMPILER / MANUAL LAB (use examples/ch17_scratch.rs):
//! Experiment with String::new/with_capacity/from, push/push_str, insert/insert_str,
//! remove, truncate, clear, drain, and replace_range on "aé🦀". For each, predict byte
//! boundaries and return values before running. Keep intentional panics in scratch.
//!
//! TRANSFER: invent a new edge case, predict it, then add a check. A lesson is
//! complete only when its tests, predictions, lab, and explanation are done.

/// A. Insert text at a byte boundary, returning false without mutation for an
/// invalid boundary or index beyond len; otherwise true.
fn insert_checked(text: &mut String, index: usize, addition: &str) -> bool {
    todo!("Task 1: implement the documented contract")
}
/// B. Retain only the first limit Unicode scalars. Do not allocate a replacement.
fn truncate_scalars(text: &mut String, limit: usize) {
    todo!("Task 2: implement the documented contract")
}
/// C. Build "[LEFT|RIGHT]" using push/push_str, preserving all input text.
fn envelope(left: &str, right: &str) -> String {
    todo!("Task 3: implement the documented contract")
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a_insertion_checks_utf8_boundaries() {
        let mut s = String::from("aé");
        assert!(!insert_checked(&mut s, 2, "!"));
        assert_eq!(s, "aé");
        assert!(!insert_checked(&mut s, 4, "!"));
        assert!(insert_checked(&mut s, 3, "🦀"));
        assert_eq!(s, "aé🦀");
    }
    #[test]
    fn b_truncation_counts_scalars_without_copying() {
        let mut s = String::from("aé🦀b");
        let ptr = s.as_ptr();
        truncate_scalars(&mut s, 2);
        assert_eq!(s, "aé");
        assert_eq!(s.as_ptr(), ptr);
        truncate_scalars(&mut s, 9);
        assert_eq!(s, "aé");
        truncate_scalars(&mut s, 0);
        assert_eq!(s, "");
    }
    #[test]
    fn c_appending_builds_exact_text() {
        assert_eq!(envelope("é", "🦀"), "[é|🦀]");
        assert_eq!(envelope("", ""), "[|]");
    }
}
