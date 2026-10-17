//! LESSON 06 — Hash maps, tree maps, and entries
//! Run: cargo test --test chapter16 l06_
//!
//! PREDICT before executing: Does HashMap iteration order belong in a test assertion? Does
//! entry(key).or_insert_with(f) invoke f for occupied entries?
//!
//! IMPLEMENT the documented tasks, then keep the tests and add an edge case.
//! EXPLAIN afterward: When does a BTreeMap range query justify ordered keys? How can
//! borrowed lookup avoid temporary String allocation?
//!
//! COMPILER / MANUAL LAB (use examples/ch16_scratch.rs):
//! Write an Entry match that increments an occupied count and inserts one for a vacant
//! entry. Query a String-keyed map with &str. Mutate values through values_mut without
//! changing keys.
//!
//! TRANSFER: invent a new edge case, predict it, then add a check. A lesson is
//! complete only when its tests, predictions, lab, and explanation are done.

use std::collections::{BTreeMap, HashMap};
/// A. Count exact words using Entry. Do not normalize or discard empty strings.
fn frequencies(words: &[&str]) -> HashMap<String, usize> {
    todo!("Task 1: implement the documented contract")
}
/// B. Return entries sorted by key, without relying on hash iteration order.
fn ordered(map: &HashMap<String, usize>) -> Vec<(String, usize)> {
    todo!("Task 2: implement the documented contract")
}
/// C. Return owned copies of tree-map entries with start<=key<end. Require start<=end.
fn in_range(map: &BTreeMap<i32, String>, start: i32, end: i32) -> Vec<(i32, String)> {
    todo!("Task 3: implement the documented contract")
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a_entry_counts_duplicates_and_empty_keys() {
        let map = frequencies(&["b", "a", "b", ""]);
        assert_eq!(map.len(), 3);
        assert_eq!(map.get("b"), Some(&2));
        assert_eq!(map.get(""), Some(&1));
    }
    #[test]
    fn b_output_order_is_explicit() {
        assert_eq!(
            ordered(&HashMap::from([("z".into(), 1), ("a".into(), 2)])),
            [("a".into(), 2), ("z".into(), 1)]
        );
    }
    #[test]
    fn c_tree_range_is_ordered_and_half_open() {
        let map = BTreeMap::from([(3, "c".into()), (1, "a".into()), (2, "b".into())]);
        assert_eq!(in_range(&map, 1, 3), [(1, "a".into()), (2, "b".into())]);
        assert!(in_range(&map, 2, 2).is_empty());
    }
}
