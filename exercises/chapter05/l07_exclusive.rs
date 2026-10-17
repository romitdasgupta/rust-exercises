//! LESSON 07 — Sharing versus mutation
//! Run: cargo test --test chapter05 l07_
//!
//! PREDICT before running: Can a vector grow while a reference to one of its elements is still used? Does a borrow always last to the closing brace?
//! IMPLEMENT the tasks below. Preserve the tests and add an edge case.
//! EXPLAIN afterward: Explain last use, reallocation invalidation, exclusive access, and why runtime capacity is not enough to permit an overlapping borrow.
//! COMPILER / TRANSFER LAB: Borrow v[0], push another element, then print the borrow. Move the print before the push. Try simultaneous &mut and & access to one integer. Reborrow an &mut temporarily and show when the original becomes usable again.
//! Use examples/ch05_scratch.rs; repair intentional failures before committing.

/// A. Return the old byte length, then append the suffix in place.
fn measure_then_append(text: &mut String, suffix: &str) -> usize {
    todo!("Implement this task")
}
/// B. If nonempty, append a CLONE of the first String and return true.
/// Empty input stays empty and returns false. End the borrow before push.
fn duplicate_first(values: &mut Vec<String>) -> bool {
    todo!("Implement this task")
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a_read_ends_before_write() {
        let mut s = "é".to_owned();
        assert_eq!(measure_then_append(&mut s, "!"), 2);
        assert_eq!(s, "é!");
        assert_eq!(measure_then_append(&mut s, ""), 3);
    }
    #[test]
    fn b_growth_keeps_existing_values_valid() {
        let mut v = vec!["a".into(), "b".into()];
        assert!(duplicate_first(&mut v));
        assert_eq!(v, ["a", "b", "a"]);
        assert_ne!(v[0].as_ptr(), v[2].as_ptr());
        let mut empty = vec![];
        assert!(!duplicate_first(&mut empty));
        assert!(empty.is_empty());
    }
}
