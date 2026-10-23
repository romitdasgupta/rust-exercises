//! LESSON 05 — Copy and Clone for captures
//! Run: cargo test --test chapter14 l05_
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider: If a closure captures only an i32 by move,
//! can the closure be Copy? If it owns a Vec, can cloning create independent state?
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
    use practice::guess;
    #[test]
    fn a_scalar_capture_makes_copy_possible() {
        assert_eq!(copied_calls(7, 3), (10, 10));
        assert_eq!(copied_calls(-4, 0), (-4, -4));
        assert_eq!(copied_calls(0, -9), (-9, -9));
    }
    #[test]
    fn b_cloned_owned_captures_are_independent() {
        assert_eq!(cloned_states(vec![9]), (vec![9, 1], vec![9, 2]));
        assert_eq!(cloned_states(vec![]), (vec![1], vec![2]));
        assert_eq!(cloned_states(vec![5, 6]), (vec![5, 6, 1], vec![5, 6, 2]));
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    /// A wrong prediction is useful: read the failure, then explain it in NOTES.md.
    #[test]
    fn p_predictions() {
        // P1. This FnMut closure owns a copied i32, so the closure itself is Copy.
        // (f's second result, g's first result, hits afterwards)?
        let mut hits = 0;
        let mut f = move || {
            hits += 1;
            hits
        };
        let mut g = f;
        f();
        let second = f();
        let p1: (i32, i32, i32) = guess();
        assert_eq!(
            p1,
            (second, g(), hits),
            "P1: copying a closure copies its state"
        );

        // P2. Cloning a closure that owns a Vec: does the clone share the Vec's buffer?
        let v = Vec::from([1, 2, 3]);
        let owner = move || v.as_ptr();
        let twin = owner.clone();
        let p2: bool = guess(); // true if both closures see the same buffer
        assert_eq!(p2, owner() == twin(), "P2: cloning clones owned captures");

        // P3. A closure that borrows a String is Copy. Do the copies see one buffer?
        let s = String::from("shared");
        let reader = || s.as_ptr();
        let copy = reader;
        let p3: bool = guess(); // true if both closures see the same buffer
        assert_eq!(
            p3,
            reader() == copy(),
            "P3: copying a borrow copies the pointer"
        );
    }
}
