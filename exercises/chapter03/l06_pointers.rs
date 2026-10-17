//! LESSON 06 — Pointer types encode different policies
//! Run: cargo test --test chapter03 l06_
//!
//! PREDICT: Which types borrow and which own an allocation: `&T`, `&mut T`,
//! `Box<T>`, `Rc<T>`, `Arc<T>`? When does each owned allocation get freed?
//! Does cloning an Rc clone its string contents? What does Arc add to Rc?
//! Arc's count is atomic; it does not by itself synchronize mutation of its contents.
//!
//! EXPLAIN: Why are nested Rust tuples values stored together rather than
//! automatically separate heap objects? Why use `Option<&T>` for absence?
//! How do raw pointers differ in nullability and dereferencing requirements?
//! Recognizing these policies is enough here; full borrowing comes later.
//!
//! COMPILER LAB: try mutating through `&i32`, then through `&mut i32`.
//! Separately, try reading through a `*const i32` in safe code. Repair that
//! experiment by using a reference; no exercise calls for an unsafe block.
//! Compare `size_of::<&i32>()` with `size_of::<Option<&i32>>()`, and then with
//! `size_of::<&[i32]>()`. Explain absence without null references and slice metadata.

use std::rc::Rc;
use std::sync::Arc;

/// A. Add one to the borrowed integer and return its new value.
/// Inputs are below i32::MAX.
fn increment(value: &mut i32) -> i32 {
    todo!("A: mutate the referent, not the reference variable")
}

/// B. Allocate one pair on the heap and return its owning box.
fn boxed_point(x: i32, y: i32) -> Box<(i32, i32)> {
    todo!("B: put the pair in a Box")
}

/// C. Return TWO Rc pointers to the SAME owned String allocation.
fn share_label(label: &str) -> (Rc<String>, Rc<String>) {
    todo!("C: allocate once, clone the reference-counted pointer")
}

/// D. Return TWO Arc pointers to the SAME u32 allocation.
/// The tests supply the thread code; your task is just construction and sharing.
fn share_count(value: u32) -> (Arc<u32>, Arc<u32>) {
    todo!("D: choose the reference count suitable for sharing across threads")
}

/// E. Borrow the supplied value only when visible is true; otherwise use None.
/// Do not copy the integer into a new allocation.
fn visible_value(value: &i32, visible: bool) -> Option<&i32> {
    todo!("E: represent an optional non-owning pointer")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_mutable_borrow_changes_the_original() {
        let mut value = 41;
        assert_eq!(increment(&mut value), 42);
        assert_eq!(value, 42);
        let mut negative = -1;
        assert_eq!(increment(&mut negative), 0);
    }

    #[test]
    fn b_box_owns_the_pair() {
        let point = boxed_point(3, -8);
        assert_eq!(*point, (3, -8));
    }

    #[test]
    fn c_rc_pointers_share_one_allocation() {
        let (first, second) = share_label("sensor north");
        assert_eq!(first.as_str(), "sensor north");
        assert!(Rc::ptr_eq(&first, &second));
        assert_eq!(Rc::strong_count(&first), 2);
        drop(first);
        assert_eq!(Rc::strong_count(&second), 1);
        assert_eq!(second.as_str(), "sensor north");
    }

    #[test]
    fn d_arc_keeps_the_shared_value_alive_across_threads() {
        let (first, second) = share_count(42);
        assert!(Arc::ptr_eq(&first, &second));
        assert_eq!(Arc::strong_count(&first), 2);
        let worker = std::thread::spawn(move || *second);
        assert_eq!(worker.join().expect("worker panicked"), 42);
        assert_eq!(*first, 42);
        assert_eq!(Arc::strong_count(&first), 1);
    }

    #[test]
    fn e_optional_reference_preserves_identity() {
        let value = 123;
        let borrowed = visible_value(&value, true).expect("visible value missing");
        assert!(std::ptr::eq(borrowed, &value));
        assert_eq!(visible_value(&value, false), None);
    }
}
