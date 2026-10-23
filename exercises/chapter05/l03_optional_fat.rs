//! LESSON 03 — Optional references and fat pointers
//! Run: cargo test --test chapter05 l03_
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider: What represents absence if references cannot be null? Does &[u8] contain just an address?
//! IMPLEMENT the tasks below. Preserve the tests and add an edge case.
//! EXPLAIN afterward: Explain borrowed temporaries, Option<&T>, slice length metadata, and why a borrow does not own the referent.
//! COMPILER / TRANSFER LAB: Inspect size_of::<&u8>(), size_of::<&[u8]>(), and size_of::<&str>() without assuming a byte width. Borrow `&format!(...)` in a let initializer, then try returning that borrow from a function. Explain temporary lifetime extension.
//! Use examples/ch05_scratch.rs; repair intentional failures before committing.

/// A. Borrow the indexed string, or return None. Do not allocate.
fn lookup(values: &[String], index: usize) -> Option<&String> {
    todo!("Implement this task")
}
/// B. Borrow at most n leading bytes. An excessive n is clamped to the length.
fn prefix(bytes: &[u8], n: usize) -> &[u8] {
    todo!("Implement this task")
}
#[cfg(test)]
mod tests {
    use super::*;
    use practice::{alloc::count, guess};
    use std::cell::RefCell;
    #[test]
    fn a_optional_borrow_is_not_null() {
        let v = vec!["a".into(), "b".into()];
        assert!(std::ptr::eq(lookup(&v, 1).unwrap(), &v[1]));
        assert_eq!(lookup(&v, 2), None);
        assert_eq!(lookup(&[], 0), None);
    }
    #[test]
    fn b_slice_keeps_address_and_length() {
        let a = [1, 2, 3];
        assert_eq!(prefix(&a, 2), [1, 2]);
        assert_eq!(prefix(&a, 99), a);
        assert_eq!(prefix(&a, 0), []);
        assert_eq!(prefix(&a, 2).as_ptr(), a.as_ptr());
        assert_eq!(prefix(&[], 9), []);
    }

    #[test]
    fn a_b_lookups_do_not_allocate() {
        let v: Vec<String> = vec!["a".into(), "b".into()];
        let (found, allocations) = count(|| lookup(&v, 0));
        assert_eq!(found.map(String::as_str), Some("a"));
        assert_eq!(allocations, 0, "lookup should borrow, not clone");
        let bytes = [1, 2, 3];
        let (head, allocations) = count(|| prefix(&bytes, 2));
        assert_eq!(head, [1, 2]);
        assert_eq!(allocations, 0, "prefix should borrow a subslice");
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
        // P1. Size in machine words. Does None need extra space?
        let p1: (usize, usize) = guess();
        assert_eq!(
            p1,
            (
                size_of::<Option<&[u8]>>() / size_of::<usize>(),
                size_of::<Option<usize>>() / size_of::<usize>()
            ),
            "P1: Option<&T> reuses the never-null pointer"
        );

        // P2. A borrowed temporary lives to the end of the block; a value
        // bound to `_` is dropped at once. Order of the log?
        let log = RefCell::new(Vec::new());
        {
            let _kept = &Noisy("kept", &log);
            let _ = Noisy("dropped", &log);
            log.borrow_mut().push("end");
        }
        let p2: Vec<&str> = guess();
        assert_eq!(p2, *log.borrow(), "P2: temporary lifetime extension");

        // P3. `max` and `min` return references. Which element does each
        // point at when there are ties? Give their indices.
        let v = [3, 9, 1, 9, 1];
        let index_of = |r: &i32| v.iter().position(|x| std::ptr::eq(x, r)).unwrap();
        let p3: (usize, usize) = guess();
        assert_eq!(
            p3,
            (
                index_of(v.iter().max().unwrap()),
                index_of(v.iter().min().unwrap())
            ),
            "P3: ties in max and min"
        );
    }
}
