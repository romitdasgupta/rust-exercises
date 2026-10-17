//! LESSON 02 — Borrowed parallel work
//! Run: cargo test --test chapter19 l02_
//!
//! PREDICT before running:
//! 1. Does Arc clone copy the allocation contents?
//! 2. Can scoped threads mutate disjoint parts of a slice?
//!
//! IMPLEMENT the tasks; keep the tests.
//! EXPLAIN: Explain the relationship between scope lifetime, disjoint borrows, and joining.
//! COMPILER LAB (examples/ch19_scratch.rs): Try capturing one mutable slice twice in scoped closures; repair with split_at_mut. Compare Arc<Vec<_>> for unscoped workers.

use std::thread;
/// A. Sum a borrowed i32 slice in two scoped workers. Widen each element to
/// i64 before adding; split at len/2. Empty inputs are valid.
fn scoped_sum(values: &[i32]) -> i64 {
    todo!("A: implement the contract")
}
/// B. Add delta to every element in two scoped workers without cloning data
/// or using a mutex. Inputs guarantee every sum fits i32.
fn scoped_add(values: &mut [i32], delta: i32) {
    todo!("B: implement the contract")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn scoped_workers_borrow_input() {
        assert_eq!(scoped_sum(&[i32::MAX, i32::MAX, -1]), 4294967293);
        assert_eq!(scoped_sum(&[]), 0);
        let mut a = [1, 2, 3, 4, 5];
        scoped_add(&mut a, -2);
        assert_eq!(a, [-1, 0, 1, 2, 3]);
        scoped_add(&mut [], 7);
    }
}
