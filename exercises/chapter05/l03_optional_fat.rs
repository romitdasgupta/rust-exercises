//! LESSON 03 — Optional references and fat pointers
//! Run: cargo test --test chapter05 l03_
//!
//! PREDICT before running: What represents absence if references cannot be null? Does &[u8] contain just an address?
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
}
