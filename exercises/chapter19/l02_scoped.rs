//! LESSON 02 — Borrowed parallel work
//! Run: cargo test --test chapter19 l02_
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider:
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
    use practice::{alloc::count, guess};
    use std::sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    };
    #[test]
    fn a_scoped_workers_borrow_input() {
        assert_eq!(scoped_sum(&[i32::MAX, i32::MAX, -1]), 4294967293);
        assert_eq!(scoped_sum(&[i32::MIN, i32::MIN]), -4294967296);
        assert_eq!(scoped_sum(&[]), 0);
        assert_eq!(scoped_sum(&[-5]), -5);
    }
    #[test]
    fn b_scoped_workers_mutate_disjoint_halves() {
        let mut a = [1, 2, 3, 4, 5];
        scoped_add(&mut a, -2);
        assert_eq!(a, [-1, 0, 1, 2, 3]);
        let mut one = [7];
        scoped_add(&mut one, 1);
        assert_eq!(one, [8]);
        scoped_add(&mut [], 7);
    }
    #[test]
    fn b_scoped_add_edits_in_place() {
        let mut v: Vec<i32> = (0..1000).collect();
        let p = v.as_ptr();
        scoped_add(&mut v, 1);
        assert_eq!(v.as_ptr(), p);
        assert!(v.iter().copied().eq(1..1001));
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    #[test]
    fn p_predictions() {
        // P1. Allocations made by Arc::clone, and do both handles point at one Vec?
        let a = Arc::new(vec![1u8; 1024]);
        let (b, allocations) = count(|| Arc::clone(&a));
        let p1: (usize, bool) = guess();
        assert_eq!(p1, (allocations, Arc::ptr_eq(&a, &b)), "P1: cloning an Arc");

        // P2. Lengths of the two halves from split_at_mut(len / 2).
        let mut v = [10, 20, 30, 40, 50];
        let mid = v.len() / 2;
        let (left, right) = v.split_at_mut(mid);
        let p2: (usize, usize) = guess();
        assert_eq!(p2, (left.len(), right.len()), "P2: where the split lands");

        // P3. Nobody calls join here. What does the counter read after the scope?
        let hits = AtomicUsize::new(0);
        thread::scope(|s| {
            for _ in 0..3 {
                s.spawn(|| hits.fetch_add(1, Ordering::Relaxed));
            }
        });
        let p3: usize = guess();
        assert_eq!(p3, hits.into_inner(), "P3: a scope joins its threads");
    }
}
