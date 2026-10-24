//! LESSON 07 — Rc and Arc
//! Run: cargo test --test chapter04 l07_
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider: Does Rc::clone duplicate the String allocation? Can two Rc owners provide &mut String? Which pointer is appropriate across threads?
//! IMPLEMENT the tasks below. Preserve the tests and add an edge case.
//! EXPLAIN afterward: Explain strong counts, destruction of the last owner, thread safety costs, and why reference cycles can leak.
//! COMPILER / TRANSFER LAB: Try sending Rc<String> to thread::spawn; then use Arc. Compare clone on an Rc with clone on its dereferenced String. Sketch a two-node strong cycle and how Weak would break it (implementation optional).
//! Use examples/ch04_scratch.rs; repair intentional failures before committing.

use std::rc::Rc;
use std::sync::Arc;
/// A. Return two owners of one allocation, containing input text.
fn shared_text(text: &str) -> (Rc<String>, Rc<String>) {
    todo!("Implement this task")
}
/// B. Spawn one thread that returns the byte length of shared text; join it.
/// Clone the Arc, not its String. Return (length, owner_count_after_join).
fn thread_length(text: Arc<String>) -> (usize, usize) {
    todo!("Implement this task")
}
#[cfg(test)]
mod tests {
    use super::*;
    use practice::{alloc::count, guess};
    #[test]
    fn a_pointers_share_one_allocation() {
        let (a, b) = shared_text("harbor");
        assert!(Rc::ptr_eq(&a, &b));
        assert_eq!(Rc::strong_count(&a), 2);
        drop(b);
        assert_eq!(Rc::strong_count(&a), 1);
        assert_eq!(a.as_str(), "harbor");
    }
    #[test]
    fn b_arc_crosses_a_thread_boundary() {
        let text = Arc::new("🦀".into());
        assert_eq!(thread_length(Arc::clone(&text)), (4, 2));
        assert_eq!(Arc::strong_count(&text), 1);
        assert_eq!(thread_length(Arc::new(String::new())), (0, 1));
    }

    #[test]
    fn a_shared_text_makes_one_rc_allocation() {
        let (_, allocations) = count(|| shared_text("harbor"));
        assert!(
            allocations <= 2,
            "one String buffer and one Rc allocation; got {allocations}"
        );
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    #[test]
    fn p_predictions() {
        let a = Rc::new(String::from("config"));
        // P1. Allocations made by Rc::clone.
        let (b, allocations) = count(|| Rc::clone(&a));
        let p1: usize = guess();
        assert_eq!(p1, allocations, "P1: cloning an Rc");

        // P2. Strong count after the clone, and after dropping it.
        let p2: (usize, usize) = guess();
        let after_clone = Rc::strong_count(&a);
        drop(b);
        assert_eq!(p2, (after_clone, Rc::strong_count(&a)), "P2: strong counts");

        // P3. `(*a).clone()` instead: allocations made, and the count afterwards.
        let (text, allocations) = count(|| (*a).clone());
        let p3: (usize, usize) = guess();
        assert_eq!(
            p3,
            (allocations, Rc::strong_count(&a)),
            "P3: cloning the value"
        );
        assert_eq!(text, "config");
    }
}
