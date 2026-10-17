//! LESSON 04 — Smart-pointer self receivers
//! Run: cargo test --test chapter09 l04_
//!
//! PREDICT before running: Does self: Rc<Self> consume one pointer owner or all owners? Can self: Box<Self> move a field out?
//! IMPLEMENT the tasks below. Preserve the tests and add an edge case.
//! EXPLAIN afterward: Explain why a method may need ownership of a smart pointer rather than a borrow of the pointee.
//! COMPILER / TRANSFER LAB: Try moving a String out of &self, then out of self: Box<Self>. Define an Arc receiver, clone the Arc at a call site, and track counts before and after. Explain how autoderef makes receiver calls convenient without changing ownership.
//! Use examples/ch09_scratch.rs; repair intentional failures before committing.

use std::{rc::Rc, sync::Arc};
struct Document {
    text: String,
}
impl Document {
    /// A. Consume unique heap ownership and return the owned String, no clone.
    fn into_text(self: Box<Self>) -> String {
        todo!("Implement this task")
    }
    /// B. Return the same Rc owner together with the number of strong owners.
    fn counted(self: Rc<Self>) -> (Rc<Self>, usize) {
        todo!("Implement this task")
    }
    /// C. Return the same Arc owner with the text byte length.
    fn measured(self: Arc<Self>) -> (Arc<Self>, usize) {
        todo!("Implement this task")
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a_box_receiver_transfers_field() {
        let text = String::from("draft");
        let p = text.as_ptr();
        let out = Box::new(Document { text }).into_text();
        assert_eq!(out, "draft");
        assert_eq!(out.as_ptr(), p);
    }
    #[test]
    fn b_rc_receiver_preserves_pointer() {
        let d = Rc::new(Document { text: "x".into() });
        let other = Rc::clone(&d);
        let (out, n) = other.counted();
        assert_eq!(n, 2);
        assert!(Rc::ptr_eq(&d, &out));
    }
    #[test]
    fn c_arc_receiver_preserves_pointer() {
        let d = Arc::new(Document {
            text: "🦀".into()
        });
        let (out, n) = Arc::clone(&d).measured();
        assert_eq!(n, 4);
        assert!(Arc::ptr_eq(&d, &out));
        assert_eq!(Arc::strong_count(&d), 2);
    }
}
