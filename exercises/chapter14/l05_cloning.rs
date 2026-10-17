//! LESSON 05 — Copy and Clone for captures
//! Run: cargo test --test chapter14 l05_
//!
//! PREDICT before executing: If a closure captures only an i32 by move, can the closure be
//! Copy? If it owns a Vec, can cloning create independent state?
//!
//! IMPLEMENT the documented tasks, then keep the tests and add an edge case.
//! EXPLAIN afterward: How do the capabilities of captured fields determine a closure’s
//! Clone and Copy implementations?
//!
//! COMPILER / MANUAL LAB (use examples/ch14_scratch.rs):
//! Write helpers requiring F:Copy and F:Clone. Pass closures capturing an i32, String,
//! &String, and &mut String. Predict each result before checking. Explain why copying a
//! shared reference does not clone its referent.
//!
//! TRANSFER: invent a new edge case, predict it, then add a check. A lesson is
//! complete only when its tests, predictions, lab, and explanation are done.

/// A. Copy a closure capturing offset by value; invoke both copies with input.
/// Inputs are bounded so addition fits.
fn copied_calls(offset: i32, input: i32) -> (i32, i32) {
    todo!("Task 1: implement the documented contract")
}
/// B. Clone a stateful closure owning seed. First copy appends 1, second appends 2.
/// Each call returns its own owned vector snapshot. Neither output shares edits.
fn cloned_states(seed: Vec<i32>) -> (Vec<i32>, Vec<i32>) {
    todo!("Task 2: implement the documented contract")
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a_scalar_capture_makes_copy_possible() {
        assert_eq!(copied_calls(7, 3), (10, 10));
        assert_eq!(copied_calls(-4, 0), (-4, -4));
    }
    #[test]
    fn b_cloned_owned_captures_are_independent() {
        assert_eq!(cloned_states(vec![9]), (vec![9, 1], vec![9, 2]));
        assert_eq!(cloned_states(vec![]), (vec![1], vec![2]));
    }
}
