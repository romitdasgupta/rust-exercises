//! LESSON 04 — Moving from indexed collections
//! Run: cargo test --test chapter04 l04_
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider: Why can you copy vec[0] for integers but not move vec[0] for Strings? How does remove differ from swap_remove?
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
    use practice::{alloc::count, guess};
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

    #[test]
    fn a_b_extraction_does_not_clone() {
        let mut v: Vec<String> = vec!["a".into(), "b".into(), "c".into()];
        let (_, allocations) = count(|| ordered_take(&mut v, 0));
        assert_eq!(allocations, 0, "ordered_take should move the element out");
        let (_, allocations) = count(|| unordered_take(&mut v, 0));
        assert_eq!(allocations, 0, "unordered_take should move the element out");
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    #[test]
    fn p_predictions() {
        // P1. Contents after swap_remove(0).
        let mut v = vec!["a", "b", "c", "d"];
        v.swap_remove(0);
        let p1: Vec<&str> = guess();
        assert_eq!(p1, v, "P1: swap_remove");

        // P2. Contents after remove(0).
        let mut v = vec!["a", "b", "c", "d"];
        v.remove(0);
        let p2: Vec<&str> = guess();
        assert_eq!(p2, v, "P2: remove");

        // P3. swap_remove on the LAST index: what is returned, and what is left?
        let mut v = vec!["a", "b", "c"];
        let last = v.swap_remove(2);
        let p3: (&str, Vec<&str>) = guess();
        assert_eq!(p3, (last, v), "P3: swap_remove of the last element");
    }
}
