//! LESSON 09 — Capstone: a task scheduler and audit
//! Run: cargo test --test chapter16 l09_
//!
//! PREDICT before running: Choose a container for pending priorities, uniqueness, and
//! completion order before coding. Predict duplicate tasks and equal-priority ties.
//!
//! IMPLEMENT the documented tasks, then keep the tests and add an edge case.
//! EXPLAIN afterward: What complexity do heap insertion/pop and hash lookup suggest? Which
//! output order must be made explicit?
//!
//! COMPILER / MANUAL LAB (use examples/ch16_scratch.rs):
//! Write a design note for an indexed collection, graph, persistent map, and small-vector
//! workload. Identify when a standard collection suffices and what measurements would
//! justify an external collection crate.
//!
//! TRANSFER: invent a new edge case, predict it, then add a check. A lesson is
//! complete only when its tests, predictions, lab, and explanation are done.

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
/// A. For duplicate names keep only the highest priority.
fn merge(tasks: &[(&str, u8)]) -> HashMap<String, u8> {
    todo!("Task 1: implement the documented contract")
}
/// B. Execute at most limit unique names: highest priority first, then ascending
/// lexical name for ties. Return owned names; zero limit returns empty.
fn schedule(tasks: &[(&str, u8)], limit: usize) -> Vec<String> {
    todo!("Task 2: implement the documented contract")
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a_merging_keeps_highest_priority_and_all_names() {
        assert_eq!(
            merge(&[("a", 2), ("a", 7), ("b", 0), ("a", 3)]),
            HashMap::from([("a".into(), 7), ("b".into(), 0)])
        );
        assert_eq!(
            merge(&[("x", 255), ("x", 0), ("é", 1)]),
            HashMap::from([("x".into(), 255), ("é".into(), 1)])
        );
        assert!(merge(&[]).is_empty());
    }
    #[test]
    fn b_scheduler_has_deterministic_ties_and_limits() {
        let tasks = [("z", 3), ("a", 3), ("low", 1), ("z", 8), ("b", 3)];
        assert_eq!(schedule(&tasks, 9), ["z", "a", "b", "low"]);
        assert_eq!(schedule(&tasks, 2), ["z", "a"]);
        assert!(schedule(&tasks, 0).is_empty());
        assert!(schedule(&[], 5).is_empty());
    }
    #[test]
    fn b_duplicates_run_once_at_their_highest_priority() {
        let tasks = [("b", 1), ("a", 1), ("b", 5), ("c", 5), ("a", 0)];
        assert_eq!(schedule(&tasks, 3), ["b", "c", "a"]);
        assert_eq!(schedule(&tasks, 1), ["b"]);
        let ties = [("B", 2), ("a", 2), ("A", 2)];
        assert_eq!(schedule(&ties, 3), ["A", "B", "a"], "byte-wise name order");
    }
}
