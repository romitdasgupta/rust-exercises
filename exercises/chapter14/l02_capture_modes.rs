//! LESSON 02 — Mutation, move, and consuming captures
//! Run: cargo test --test chapter14 l02_
//!
//! PREDICT before executing: Does move || text.len() consume text on every call? What
//! changes when the body returns text itself?
//!
//! IMPLEMENT the documented tasks, then keep the tests and add an edge case.
//! EXPLAIN afterward: Which closures implement Fn, FnMut, and FnOnce? Explain why every Fn
//! closure also supports the weaker call traits.
//!
//! COMPILER / MANUAL LAB (use examples/ch14_scratch.rs):
//! Write move || owned_string and call it twice; record the moved-closure error. Change the
//! body to owned_string.len() and call twice. Capture a mutable counter without move and
//! inspect access while the closure is still used later.
//!
//! TRANSFER: invent a new edge case, predict it, then add a check. A lesson is
//! complete only when its tests, predictions, lab, and explanation are done.

/// A. A closure mutably captures total. Return each running total, starting at initial.
/// Inputs are small enough to fit i64.
fn running_totals(initial: i64, values: &[i32]) -> Vec<i64> {
    todo!("Task 1: implement the documented contract")
}
/// B. Return an owning, reusable closure whose result is prefix + argument.
fn owned_prefix(prefix: String) -> impl Fn(&str) -> String {
    move |_suffix: &str| -> String { todo!("Task 2: implement the documented contract") }
}
/// C. Move an owned String into an FnOnce closure and consume it exactly once.
fn consume_once(text: String) -> String {
    todo!("Task 3: implement the documented contract")
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a_mutable_capture_tracks_state() {
        assert_eq!(running_totals(10, &[2, -5, 0]), [12, 7, 7]);
        assert!(running_totals(0, &[]).is_empty());
    }
    #[test]
    fn b_move_capture_can_implement_fn() {
        let f = owned_prefix("api/".into());
        assert_eq!(f("a"), "api/a");
        assert_eq!(f("b"), "api/b");
    }
    #[test]
    fn c_consuming_capture_transfers_the_allocation() {
        let text = String::from("owned");
        let ptr = text.as_ptr();
        let out = consume_once(text);
        assert_eq!(out, "owned");
        assert_eq!(out.as_ptr(), ptr);
    }
}
