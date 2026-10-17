//! LESSON 04 — Queues and ring buffers
//! Run: cargo test --test chapter16 l04_
//!
//! PREDICT before executing: Is VecDeque guaranteed to store every element in one
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
    #[test]
    fn a_history_evicts_from_front_including_oversized_input() {
        let mut q = VecDeque::from([1, 2, 3, 4]);
        remember(&mut q, 5, 3);
        assert_eq!(q, VecDeque::from([3, 4, 5]));
        remember(&mut q, 9, 0);
        assert!(q.is_empty());
    }
    #[test]
    fn b_rotation_preserves_fifo_sequence() {
        let mut q = VecDeque::from(["a".into(), "b".into()]);
        assert_eq!(round_robin(&mut q), Some("a".into()));
        assert_eq!(q, VecDeque::from(["b".into(), "a".into()]));
        assert_eq!(round_robin(&mut VecDeque::new()), None);
    }
}
