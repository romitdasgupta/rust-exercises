//! LESSON 02 — Constructing and editing owned text
//! Run: cargo test --test chapter17 l02_
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider: Which String operations take byte offsets? Is capacity the
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
    use practice::{alloc::count, guess};
    #[test]
    fn a_insertion_checks_utf8_boundaries() {
        let mut s = String::from("aé");
        assert!(!insert_checked(&mut s, 2, "!"));
        assert_eq!(s, "aé");
        assert!(!insert_checked(&mut s, 4, "!"));
        assert!(insert_checked(&mut s, 3, "🦀"));
        assert_eq!(s, "aé🦀");
        assert!(insert_checked(&mut s, 0, "¡"));
        assert_eq!(s, "¡aé🦀");
        assert!(!insert_checked(&mut s, usize::MAX, "x"));
        assert_eq!(s, "¡aé🦀");
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
        let mut crab = String::from("🦀🦀🦀");
        truncate_scalars(&mut crab, 1);
        assert_eq!(crab, "🦀");
    }
    #[test]
    fn b_truncation_does_not_allocate() {
        let mut s = "é".repeat(50);
        let (_, allocations) = count(|| truncate_scalars(&mut s, 10));
        assert_eq!(s.chars().count(), 10);
        assert_eq!(allocations, 0, "truncate in place");
    }
    #[test]
    fn c_appending_builds_exact_text() {
        assert_eq!(envelope("é", "🦀"), "[é|🦀]");
        assert_eq!(envelope("", ""), "[|]");
        assert_eq!(envelope("a|b", "]"), "[a|b|]]");
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    #[test]
    fn p_predictions() {
        // P1. pop removes one whole scalar. What comes off, and how many bytes remain?
        let mut s = String::from("aé🦀");
        let popped = s.pop();
        let p1: (Option<char>, usize) = guess();
        assert_eq!(p1, (popped, s.len()), "P1: pop");

        // P2. remove takes a byte index. What does remove(1) return on "aéb"?
        let mut s = String::from("aéb");
        let removed = s.remove(1);
        let p2: (char, &str) = guess();
        assert_eq!(p2, (removed, s.as_str()), "P2: remove");

        // P3. drain(..2) on "hello": the drained text, and what remains.
        let mut s = String::from("hello");
        let drained: String = s.drain(..2).collect();
        let p3: (&str, &str) = guess();
        assert_eq!(p3, (drained.as_str(), s.as_str()), "P3: drain");

        // P4. Heap allocations made by building "hello world" in a
        // String::with_capacity(11).
        let (built, allocations) = count(|| {
            let mut s = String::with_capacity(11);
            s.push_str("hello");
            s.push(' ');
            s.push_str("world");
            s
        });
        let p4: usize = guess();
        assert_eq!(p4, allocations, "P4: pushes within capacity");
        assert_eq!(built, "hello world");
    }
}
