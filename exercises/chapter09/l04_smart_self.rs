//! LESSON 04 — Smart-pointer self receivers
//! Run: cargo test --test chapter09 l04_
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider: Does self: Rc<Self> consume one pointer owner or all owners? Can self: Box<Self> move a field out?
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
    #[allow(clippy::boxed_local)] // the Box receiver is the point of this task
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
    use practice::{alloc::count, guess};
    #[test]
    fn a_box_receiver_transfers_field() {
        let text = String::from("draft");
        let p = text.as_ptr();
        let out = Box::new(Document { text }).into_text();
        assert_eq!(out, "draft");
        assert_eq!(out.as_ptr(), p);
        let doc = Box::new(Document {
            text: "final".into(),
        });
        let (out, allocations) = count(|| doc.into_text());
        assert_eq!(out, "final");
        assert_eq!(allocations, 0, "into_text should move the String out");
    }
    #[test]
    fn b_rc_receiver_preserves_pointer() {
        let d = Rc::new(Document { text: "x".into() });
        let other = Rc::clone(&d);
        let (out, n) = other.counted();
        assert_eq!(n, 2);
        assert!(Rc::ptr_eq(&d, &out));
        assert_eq!(
            Rc::strong_count(&d),
            2,
            "counted must not keep extra owners"
        );
        drop(out);
        let (_, n) = d.counted();
        assert_eq!(n, 1);
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
        let (_, n) = Arc::new(Document {
            text: String::new(),
        })
        .measured();
        assert_eq!(n, 0);
    }

    struct Probe;
    impl Probe {
        fn owners(self: Rc<Self>) -> usize {
            Rc::strong_count(&self)
        }
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    #[test]
    fn p_predictions() {
        // P1. Strong count inside the method, and in the caller after it returns.
        let d = Rc::new(Probe);
        let inside = Rc::clone(&d).owners();
        let p1: (usize, usize) = guess();
        assert_eq!(
            p1,
            (inside, Rc::strong_count(&d)),
            "P1: an Rc receiver is one owner"
        );

        // P2. Moving a Box: does the Document on the heap move too?
        let b = Box::new(Document { text: "x".into() });
        let before: *const Document = &*b;
        let moved = b;
        let p2: bool = guess(); // true if the Document is still at the same address
        assert_eq!(
            p2,
            std::ptr::eq(before, &*moved),
            "P2: moving a Box moves the pointer"
        );

        // P3. How many machine words (usize) are Box<Document>, Rc<Document>,
        // and Arc<Document> on the stack?
        let p3: (usize, usize, usize) = guess();
        let word = size_of::<usize>();
        assert_eq!(
            (p3.0 * word, p3.1 * word, p3.2 * word),
            (
                size_of::<Box<Document>>(),
                size_of::<Rc<Document>>(),
                size_of::<Arc<Document>>()
            ),
            "P3: smart pointer sizes"
        );
    }
}
