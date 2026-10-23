//! LESSON 06 — Copy and explicit cloning
//! Run: cargo test --test chapter04 l06_
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider: Does assigning [u8; 4] move it? Does assigning [String; 2]? Is Clone implicit?
//! IMPLEMENT the tasks below. Preserve the tests and add an edge case.
//! EXPLAIN afterward: Explain value semantics, Copy opt-in, and independent deep data after cloning a String.
//! COMPILER / TRANSFER LAB: Derive Copy and Clone on a two-integer struct, then add String and inspect the diagnostic. Try copying a tuple containing one String. Explain why a type with Drop cannot be Copy.
//! Use examples/ch04_scratch.rs; repair intentional failures before committing.

/// A. Copy the fixed-size integer array, overwrite the copy's first byte, and
/// return (original, changed). Do not use clone.
fn copied_header(header: [u8; 4], first: u8) -> ([u8; 4], [u8; 4]) {
    todo!("Implement this task")
}
/// B. Return an independent owned copy with a suffix; input remains unchanged.
/// Explicitly clone the input String before modifying it.
fn cloned_label(input: &String, suffix: &str) -> String {
    todo!("Implement this task")
}
#[cfg(test)]
mod tests {
    use super::*;
    use practice::{alloc::count, guess};
    use std::hint::black_box;
    #[test]
    fn a_copy_leaves_source_usable() {
        assert_eq!(copied_header([1, 2, 3, 4], 9), ([1, 2, 3, 4], [9, 2, 3, 4]));
    }
    #[test]
    fn b_clone_owns_independent_text() {
        let s = String::from("港");
        let out = cloned_label(&s, "!");
        assert_eq!(s, "港");
        assert_eq!(out, "港!");
        assert_ne!(s.as_ptr(), out.as_ptr());
        assert_eq!(cloned_label(&String::new(), ""), "");
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    #[test]
    fn p_predictions() {
        // P1. Copying an array of integers, then changing the copy.
        let a = [1u8, 2, 3];
        let mut b = a;
        b[0] = 9;
        let p1: u8 = guess();
        assert_eq!(p1, a[0], "P1: a[0] after changing the copy");
        assert_eq!(b, [9, 2, 3]);

        // P2. Heap allocations made by cloning a Vec holding two non-empty Strings.
        let v = vec!["ab".to_owned(), "cd".to_owned()];
        let (copy, allocations) = count(|| black_box(&v).clone());
        let p2: usize = guess();
        assert_eq!(p2, allocations, "P2: Clone is deep for Vec<String>");
        assert_eq!(copy, v);

        // P3. Heap allocations made by copying a [u64; 1000] array.
        let big = [7u64; 1000];
        let (copy, allocations) = count(|| black_box(big));
        let p3: usize = guess();
        assert_eq!(p3, allocations, "P3: Copy never touches the heap");
        assert_eq!(copy[999], 7);
    }
}
