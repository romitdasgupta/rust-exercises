//! LESSON 05 — Generic structs and impl blocks
//! Run: cargo test --test chapter09 l05_
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider: Does a container need T: Clone just to own or move T? Where should a bound go if only one method requires it?
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
    use practice::{alloc::count, guess};
    use std::cell::RefCell;
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
        assert_eq!(t.replace("c".into()), None);
    }
    #[test]
    fn a_b_slot_moves_without_cloning() {
        let mut slot = Slot {
            value: Some(String::from("old")),
        };
        let new = String::from("new");
        let p = new.as_ptr();
        let (old, allocations) = count(|| slot.replace(new));
        assert_eq!(old.as_deref(), Some("old"));
        assert_eq!(allocations, 0, "replace should move both values");
        let (taken, allocations) = count(|| slot.take());
        assert_eq!(taken.map(|s| s.as_ptr()), Some(p));
        assert_eq!(allocations, 0, "take should move the value out");
    }

    struct Noisy<'a>(&'static str, &'a RefCell<Vec<&'static str>>);
    impl Drop for Noisy<'_> {
        fn drop(&mut self) {
            self.1.borrow_mut().push(self.0);
        }
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    #[test]
    fn p_predictions() {
        // P1. Is a Slot<Box<String>> the same size as a bare Box<String>?
        let p1: bool = guess();
        assert_eq!(
            p1,
            size_of::<Slot<Box<String>>>() == size_of::<Box<String>>(),
            "P1: Option<Box<T>> uses the null niche"
        );

        // P2. `let _ = ...` versus `let _kept = ...`: when is each old value dropped?
        let log = RefCell::new(Vec::new());
        {
            let mut slot = Slot {
                value: Some(Noisy("a", &log)),
            };
            let _ = slot.value.replace(Noisy("b", &log));
            log.borrow_mut().push("after replace");
            let _kept = slot.value.take();
            log.borrow_mut().push("after take");
        }
        let p2: Vec<&str> = guess();
        assert_eq!(p2, *log.borrow(), "P2: `_` does not bind");

        // P3. get_or_insert on a full slot, then replace.
        let mut s = Slot { value: Some(5) };
        let first = *s.value.get_or_insert(7);
        let p3: (i32, Option<i32>, Option<i32>) = guess();
        assert_eq!(
            p3,
            (first, s.value.replace(9), s.value),
            "P3: Option's in-place methods"
        );
    }
}
