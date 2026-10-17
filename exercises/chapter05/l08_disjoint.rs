//! LESSON 08 — Disjoint mutable borrows
//! Run: cargo test --test chapter05 l08_
//!
//! PREDICT before running: Will borrowing &mut values[0] and &mut values[1] separately compile? How can split_at_mut prove disjointness?
//! IMPLEMENT the tasks below. Preserve the tests and add an edge case.
//! EXPLAIN afterward: Explain which facts are encoded by split_at_mut and why it does not require unsafe in caller code.
//! COMPILER / TRANSFER LAB: Try the two indexing borrows, then split_at_mut. Contrast borrowing two distinct struct fields, which the compiler can already distinguish.
//! Use examples/ch05_scratch.rs; repair intentional failures before committing.

/// A. Add delta to all items before mid; subtract it from all items after mid.
/// Reject mid > len without mutation. Inputs and delta are within -100..100.
fn adjust_halves(values: &mut [i32], mid: usize, delta: i32) -> bool {
    todo!("Implement this task")
}
/// B. Swap distinct valid positions using split_at_mut and mem::swap.
/// Equal valid indices are a successful no-op; invalid ones return false.
fn swap_positions(values: &mut [String], a: usize, b: usize) -> bool {
    todo!("Implement this task")
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a_halves_are_nonoverlapping() {
        let mut a = [1, 2, 3];
        assert!(adjust_halves(&mut a, 1, 2));
        assert_eq!(a, [3, 0, 1]);
        assert!(!adjust_halves(&mut a, 4, 9));
        assert_eq!(a, [3, 0, 1]);
        assert!(adjust_halves(&mut [], 0, 1));
    }
    #[test]
    fn b_swap_handles_both_index_orders() {
        let mut a = ["a".into(), "b".into(), "c".into()];
        assert!(swap_positions(&mut a, 2, 0));
        assert_eq!(a, ["c", "b", "a"]);
        assert!(swap_positions(&mut a, 1, 1));
        assert!(!swap_positions(&mut a, 1, 3));
        assert!(!swap_positions(&mut [], 0, 0));
    }
}
