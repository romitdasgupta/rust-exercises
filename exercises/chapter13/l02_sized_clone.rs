//! LESSON 02 — Sized, Clone, and Copy
//! Run: cargo test --test chapter13 l02_
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider: Is str Sized? Is &str Sized? Does cloning
//! Rc<Vec<_>> clone its elements?
//!
//! IMPLEMENT the documented tasks, then keep the tests and add an edge case.
//! EXPLAIN afterward: Why is Copy a promise about implicit duplication rather than a
//! customization hook? Which fields prevent a struct from implementing Copy?
//!
//! COMPILER / MANUAL LAB (use examples/ch13_scratch.rs):
//! Remove ?Sized from borrowed_size and try str/[u8]. Attempt Copy for a struct holding
//! String and for a type implementing Drop. Compare Rc::clone strong counts with cloning a
//! Vec<String>.
//!
//! TRANSFER: invent a new edge case, predict it, then add a check. A lesson is
//! complete only when its tests, predictions, lab, and explanation are done.

use std::mem::size_of_val;
/// A. Size of the referent in bytes, supporting str and [T].
fn borrowed_size<T: ?Sized>(value: &T) -> usize {
    todo!("Task 1: implement the documented contract")
}
/// B. Clone an owned vector, append the suffix to the first cloned String if any.
/// Return the clone, leaving the source untouched.
fn cloned_edit(source: &[String], suffix: &str) -> Vec<String> {
    todo!("Task 2: implement the documented contract")
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Coordinates {
    x: i16,
    y: i16,
}
/// C. Copy point to another binding, then change the copy's x to 99.
/// Return (original,copy).
fn copied_edit(point: Coordinates) -> (Coordinates, Coordinates) {
    todo!("Task 3: implement the documented contract")
}
#[cfg(test)]
mod tests {
    use super::*;
    use practice::{alloc::count, guess};
    use std::hint::black_box;
    #[test]
    fn a_unsized_referents_are_accepted() {
        assert_eq!(borrowed_size("é"), 2);
        assert_eq!(borrowed_size(&[1_u32, 2][..]), 8);
        assert_eq!(borrowed_size(""), 0);
    }
    #[test]
    fn b_clone_owns_independent_elements() {
        let source = vec!["a".into(), "b".into()];
        assert_eq!(cloned_edit(&source, "!"), ["a!", "b"]);
        assert_eq!(source, ["a", "b"]);
        assert!(cloned_edit(&[], "!").is_empty());
    }
    #[test]
    fn c_copy_keeps_original_available() {
        let p = Coordinates { x: 2, y: 3 };
        assert_eq!(copied_edit(p), (p, Coordinates { x: 99, y: 3 }));
    }

    #[test]
    fn a_sized_referents_work_too() {
        assert_eq!(borrowed_size(&5_u64), 8);
        assert_eq!(borrowed_size(&[0_u16; 3]), 6);
    }

    #[test]
    fn b_only_the_first_element_changes() {
        let source: Vec<String> = vec!["x".into(), "y".into(), "z".into()];
        assert_eq!(cloned_edit(&source, "🦀"), ["x🦀", "y", "z"]);
        assert_eq!(cloned_edit(&source, ""), source);
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    #[test]
    fn p_predictions() {
        // P1. Sizes, in machine words, of a reference to an unsized and a sized referent.
        let p1: (usize, usize) = guess();
        assert_eq!(
            (p1.0 * size_of::<usize>(), p1.1 * size_of::<usize>()),
            (size_of::<&str>(), size_of::<&[u8; 4]>()),
            "P1: fat and thin references"
        );

        // P2. clone_from into a String that already has room. Heap allocations?
        let source = String::from("short");
        let mut target = String::with_capacity(32);
        let before = target.as_ptr();
        let (_, allocations) = count(|| target.clone_from(black_box(&source)));
        let p2: (usize, bool) = guess();
        assert_eq!(
            p2,
            (allocations, target.as_ptr() == before),
            "P2: clone_from can reuse the target's buffer"
        );
        assert_eq!(target, "short");
    }
}
