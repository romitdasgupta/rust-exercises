//! LESSON 04 — Queues and ring buffers
//! Run: cargo test --test chapter16 l04_
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider: Is VecDeque guaranteed to store every element in one
//! contiguous slice? Which end should a FIFO queue push and pop?
//!
//! IMPLEMENT the documented tasks, then keep the tests and add an edge case.
//! EXPLAIN afterward: Why can VecDeque support efficient front removal where Vec shifts
//! elements? When is make_contiguous useful?
//!
//! COMPILER / MANUAL LAB (use examples/ch16_scratch.rs):
//! Push/pop repeatedly to wrap the ring, inspect as_slices, then call make_contiguous and
//! sort. Do not depend on a particular split position.
//!
//! TRANSFER: invent a new edge case, predict it, then add a check. A lesson is
//! complete only when its tests, predictions, lab, and explanation are done.

use std::collections::VecDeque;
/// A. Add newest; retain at most limit latest values. A zero limit clears history.
fn remember(queue: &mut VecDeque<i32>, value: i32, limit: usize) {
    todo!("Task 1: implement the documented contract")
}
/// B. Pop the front and push it back; return that value, None for empty.
fn round_robin(queue: &mut VecDeque<String>) -> Option<String> {
    todo!("Task 2: implement the documented contract")
}
#[cfg(test)]
mod tests {
    use super::*;
    use practice::guess;
    #[test]
    fn a_history_evicts_from_front_including_oversized_input() {
        let mut q = VecDeque::from([1, 2, 3, 4]);
        remember(&mut q, 5, 3);
        assert_eq!(q, VecDeque::from([3, 4, 5]));
        remember(&mut q, 9, 0);
        assert!(q.is_empty());
        remember(&mut q, 1, 5);
        remember(&mut q, 2, 5);
        assert_eq!(q, VecDeque::from([1, 2]));
    }
    #[test]
    fn b_rotation_preserves_fifo_sequence() {
        let mut q = VecDeque::from(["a".into(), "b".into()]);
        assert_eq!(round_robin(&mut q), Some("a".into()));
        assert_eq!(q, VecDeque::from(["b".into(), "a".into()]));
        assert_eq!(round_robin(&mut VecDeque::new()), None);
        let mut one = VecDeque::from(["solo".to_string()]);
        assert_eq!(round_robin(&mut one), Some("solo".into()));
        assert_eq!(one, VecDeque::from(["solo".to_string()]));
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    #[test]
    fn p_predictions() {
        // P1. Contents after push_front(0) and pop_back().
        let mut q = VecDeque::from([1, 2, 3]);
        q.push_front(0);
        q.pop_back();
        let p1: Vec<i32> = guess();
        assert_eq!(p1, Vec::from(q), "P1: both ends");

        // P2. rotate_left(1): which element moves, and where?
        let mut q = VecDeque::from([1, 2, 3, 4]);
        q.rotate_left(1);
        let p2: Vec<i32> = guess();
        assert_eq!(p2, Vec::from(q), "P2: rotation");

        // P3. After wrapping around the ring, make_contiguous returns one slice.
        // What does it contain?
        let mut q = VecDeque::with_capacity(4);
        q.extend([1, 2, 3]);
        q.pop_front();
        q.extend([4, 5]);
        let p3: Vec<i32> = guess();
        assert_eq!(p3, q.make_contiguous().to_vec(), "P3: logical order");

        // P4. VecDeque::from(Vec) is documented not to reallocate. Does the
        // front slice start at the Vec's old buffer?
        let v = vec![7, 8, 9];
        let before = v.as_ptr();
        let q = VecDeque::from(v);
        let p4: bool = guess();
        assert_eq!(
            p4,
            q.as_slices().0.as_ptr() == before,
            "P4: Vec -> VecDeque"
        );
    }
}
