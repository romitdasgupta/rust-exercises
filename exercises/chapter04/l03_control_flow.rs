//! LESSON 03 — Moves and control flow
//! Run: cargo test --test chapter04 l03_
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider: Can a value moved in only one branch be used after the conditional? What if every branch reinitializes it?
//! IMPLEMENT the tasks below. Preserve the tests and add an edge case.
//! EXPLAIN afterward: Explain how definite initialization applies to every path and every loop iteration.
//! COMPILER / TRANSFER LAB: Inside a loop, move a String into a temporary without replacing it; observe the next-iteration error. Repair by assigning the returned owner on every iteration. Compare an if branch that reinitializes with one that does not.
//! Use examples/ch04_scratch.rs; repair intentional failures before committing.

/// A. Consume both alternatives and return exactly one without cloning.
fn choose(first: String, second: String, use_first: bool) -> String {
    todo!("Implement this task")
}
/// B. Pass ownership through a helper once per suffix; append suffixes in order.
/// The helper must take and return String. Use a loop and reassign `value`.
fn pipeline(mut value: String, suffixes: &[&str]) -> String {
    todo!("Implement this task")
}
#[cfg(test)]
mod tests {
    use super::*;
    use practice::{alloc::count, guess};
    use std::cell::RefCell;
    #[test]
    fn a_branch_returns_one_owner() {
        assert_eq!(choose("left".into(), "right".into(), true), "left");
        assert_eq!(choose("left".into(), "right".into(), false), "right");
    }
    #[test]
    fn b_loop_reinitializes_owner() {
        assert_eq!(pipeline("go".into(), &["-", "🦀", "!"]), "go-🦀!");
        assert_eq!(pipeline("keep".into(), &[]), "keep");
    }

    struct Noisy<'a>(&'static str, &'a RefCell<Vec<&'static str>>);
    impl Drop for Noisy<'_> {
        fn drop(&mut self) {
            self.1.borrow_mut().push(self.0);
        }
    }

    #[test]
    fn a_choose_does_not_clone() {
        let (left, right) = ("left".to_owned(), "right".to_owned());
        let (_, allocations) = count(|| choose(left, right, false));
        assert_eq!(allocations, 0, "choose should move one input out");
    }

    fn pick(log: &RefCell<Vec<&'static str>>, take_a: bool) {
        let a = Noisy("a", log);
        let b = Noisy("b", log);
        let kept = if take_a { a } else { b };
        log.borrow_mut().push("end");
    }

    /// P. Predict BEFORE running. Rust tracks at run time which owners were
    /// moved on the path taken ("drop flags").
    #[test]
    fn p_predictions() {
        // P1. Drop log when `a` is moved into `kept`.
        let log = RefCell::new(Vec::new());
        pick(&log, true);
        let p1: Vec<&str> = guess();
        assert_eq!(p1, *log.borrow(), "P1: conditional move of a");

        // P2. Drop log when `b` is moved into `kept` instead.
        let log = RefCell::new(Vec::new());
        pick(&log, false);
        let p2: Vec<&str> = guess();
        assert_eq!(p2, *log.borrow(), "P2: conditional move of b");
    }
}
