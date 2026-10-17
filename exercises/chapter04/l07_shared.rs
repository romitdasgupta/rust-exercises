//! LESSON 07 — Rc and Arc
//! Run: cargo test --test chapter04 l07_
//!
//! PREDICT before running: Does Rc::clone duplicate the String allocation? Can two Rc owners provide &mut String? Which pointer is appropriate across threads?
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
}
