//! LESSON 03 — Moves and control flow
//! Run: cargo test --test chapter04 l03_
//!
//! PREDICT before running: Can a value moved in only one branch be used after the conditional? What if every branch reinitializes it?
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
}
