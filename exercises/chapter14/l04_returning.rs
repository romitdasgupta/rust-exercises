//! LESSON 04 — Returning borrowed and erased closures
//! Run: cargo test --test chapter14 l04_
//!
//! PREDICT before executing: Can a returned closure borrow a local String created inside
//! its factory? Why can a boxed callable return different closure types from branches?
//!
//! IMPLEMENT the documented tasks, then keep the tests and add an edge case.
//! EXPLAIN afterward: Compare the allocation and dispatch choices of impl Fn and Box<dyn
//! Fn>. What is the captured-data lifetime of each API?
//!
//! COMPILER / MANUAL LAB (use examples/ch14_scratch.rs):
//! Remove move from a returned closure that refers to a local copy of a reference; inspect
//! the error. Return a closure borrowing a parameter with + 'a, then attempt to outlive the
//! referenced String.
//!
//! TRANSFER: invent a new edge case, predict it, then add a check. A lesson is
//! complete only when its tests, predictions, lab, and explanation are done.

/// A. Borrow prefix for as long as the returned callable is used.
fn prefix_test<'a>(prefix: &'a str) -> impl Fn(&str) -> bool + 'a {
    move |_text: &str| -> bool { todo!("Task 1: implement the documented contract") }
}
/// B. If double, multiply by 2; otherwise add 1. Inputs are in -1000..=1000.
/// Return different closure types behind one boxed interface.
fn choose(double: bool) -> Box<dyn Fn(i32) -> i32> {
    todo!("Task 2: implement the documented contract")
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a_returned_closure_keeps_a_borrow() {
        let p = String::from("β");
        let test = prefix_test(&p);
        assert!(test("βeta"));
        assert!(!test("beta"));
    }
    #[test]
    fn b_erased_closures_support_multiple_branches() {
        assert_eq!(choose(true)(6), 12);
        assert_eq!(choose(false)(6), 7);
        assert_eq!(choose(true)(-4), -8);
    }
}
