//! LESSON 04 — Moving from indexed collections
//! Run: cargo test --test chapter04 l04_
//!
//! PREDICT before running: Why can you copy vec[0] for integers but not move vec[0] for Strings? How does remove differ from swap_remove?
//! IMPLEMENT the tasks below. Preserve the tests and add an edge case.
//! EXPLAIN afterward: Explain why indexed storage must remain initialized and compare order and complexity of extraction operations.
//! COMPILER / TRANSFER LAB: Try `let value = vec![String::from("x")][0];`. Repair using pop, remove, and swap_remove in separate experiments. Draw how each changes indices.
//! Use examples/ch04_scratch.rs; repair intentional failures before committing.

/// A. Remove a valid index while preserving order. Return None and change
/// nothing for an invalid index. Do not clone the removed String.
fn ordered_take(values: &mut Vec<String>, index: usize) -> Option<String> {
    todo!("Implement this task")
}
/// B. Remove a valid index by filling its slot with the last element.
/// Return None without mutation for an invalid index. Use swap_remove.
fn unordered_take(values: &mut Vec<String>, index: usize) -> Option<String> {
    todo!("Implement this task")
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a_order_is_preserved() {
        let mut v = vec!["a".into(), "b".into(), "c".into()];
        assert_eq!(ordered_take(&mut v, 1), Some("b".into()));
        assert_eq!(v, ["a", "c"]);
        assert_eq!(ordered_take(&mut v, 2), None);
        assert_eq!(v, ["a", "c"]);
        assert_eq!(ordered_take(&mut vec![], 0), None);
    }
    #[test]
    fn b_last_value_fills_the_hole() {
        let mut v = vec!["a".into(), "b".into(), "c".into(), "d".into()];
        assert_eq!(unordered_take(&mut v, 1), Some("b".into()));
        assert_eq!(v, ["a", "d", "c"]);
        assert_eq!(unordered_take(&mut v, 9), None);
        assert_eq!(unordered_take(&mut vec![], 0), None);
    }
}
