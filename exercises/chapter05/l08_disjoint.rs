//! LESSON 08 — Disjoint mutable borrows
//! Run: cargo test --test chapter05 l08_
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider: Will borrowing &mut values[0] and &mut values[1] separately compile? How can split_at_mut prove disjointness?
//! IMPLEMENT the tasks below. Preserve the tests and add an edge case.
//! EXPLAIN afterward: Explain which facts are encoded by split_at_mut and why it does not require unsafe in caller code.
//! COMPILER / TRANSFER LAB: Try the two indexing borrows, then split_at_mut. Contrast borrowing two distinct struct fields, which the compiler can already distinguish.
//! Use examples/ch05_scratch.rs; repair intentional failures before committing.

/// A. Add delta to every item before index mid (`values[..mid]`) and subtract
/// it from every item from mid on (`values[mid..]`). Return true. If mid > len,
/// return false without mutation. Inputs and delta are within -100..100.
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
    use practice::{alloc::count, guess};
    #[test]
    fn a_halves_are_nonoverlapping() {
        let mut a = [1, 2, 3];
        assert!(adjust_halves(&mut a, 1, 2));
        assert_eq!(a, [3, 0, 1]);
        assert!(!adjust_halves(&mut a, 4, 9));
        assert_eq!(a, [3, 0, 1]);
        assert!(adjust_halves(&mut a, 3, 1));
        assert_eq!(a, [4, 1, 2]);
        assert!(adjust_halves(&mut a, 0, 1));
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

    #[test]
    fn b_swap_moves_strings_without_cloning() {
        let mut values: Vec<String> = vec!["a".into(), "b".into(), "c".into()];
        let (first, last) = (values[0].as_ptr(), values[2].as_ptr());
        let (swapped, allocations) = count(|| swap_positions(&mut values, 0, 2));
        assert!(swapped);
        assert_eq!(values, ["c", "b", "a"]);
        assert_eq!((values[0].as_ptr(), values[2].as_ptr()), (last, first));
        assert_eq!(allocations, 0, "swap the Strings; don't clone them");
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    #[test]
    fn p_predictions() {
        // P1. Which half does the element at `mid` land in?
        let mut a = [1, 2, 3, 4, 5];
        let (left, right) = a.split_at_mut(2);
        let p1: (usize, i32) = guess();
        assert_eq!(p1, (left.len(), right[0]), "P1: split_at_mut(2)");

        // P2. One write per half, each reading the other half.
        left[0] += right[0];
        right[2] = left[1];
        let p2: [i32; 5] = guess();
        assert_eq!(p2, a, "P2: disjoint halves");

        // P3. Two mutable borrows of different tuple fields.
        let mut pair = (1, 2);
        let x = &mut pair.0;
        let y = &mut pair.1;
        *x += *y;
        *y += *x;
        let p3: (i32, i32) = guess();
        assert_eq!(p3, pair, "P3: fields are disjoint places");

        // P4. chunks_mut yields non-overlapping mutable slices.
        let mut data = [1, 2, 3, 4, 5];
        for chunk in data.chunks_mut(2) {
            chunk[0] *= 10;
        }
        let p4: [i32; 5] = guess();
        assert_eq!(p4, data, "P4: chunks_mut");
    }
}
