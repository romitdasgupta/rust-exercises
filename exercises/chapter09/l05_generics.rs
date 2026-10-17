//! LESSON 05 — Generic structs and impl blocks
//! Run: cargo test --test chapter09 l05_
//!
//! PREDICT before running: Does a container need T: Clone just to own or move T? Where should a bound go if only one method requires it?
//! IMPLEMENT the tasks below. Preserve the tests and add an edge case.
//! EXPLAIN afterward: Explain unconstrained storage and the difference between generic type parameters and method-specific bounds.
//! COMPILER / TRANSFER LAB: Add a borrowed peek method, then a copied peek requiring T: Copy in a separate impl. Instantiate the same container with String and u32; explain why imposing Copy on the whole struct would unnecessarily reject String.
//! Use examples/ch09_scratch.rs; repair intentional failures before committing.

#[derive(Debug)]
struct Slot<T> {
    value: Option<T>,
}
impl<T> Slot<T> {
    /// A. Remove the owned value, leaving None, for arbitrary T without cloning.
    fn take(&mut self) -> Option<T> {
        todo!("Implement this task")
    }
    /// B. Install a new owned value; return the previous owned value if any.
    fn replace(&mut self, value: T) -> Option<T> {
        todo!("Implement this task")
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a_take_does_not_require_copy() {
        let text = String::from("job");
        let p = text.as_ptr();
        let mut slot = Slot { value: Some(text) };
        let out = slot.take().unwrap();
        assert_eq!(out.as_ptr(), p);
        assert_eq!(slot.take(), None);
    }
    #[test]
    fn b_replace_works_across_types() {
        let mut s = Slot { value: None };
        assert_eq!(s.replace(3), None);
        assert_eq!(s.replace(8), Some(3));
        let mut t = Slot {
            value: Some(String::from("a")),
        };
        assert_eq!(t.replace("b".into()), Some("a".into()));
        assert_eq!(t.take(), Some("b".into()));
    }
}
