//! LESSON 06 — Copy and explicit cloning
//! Run: cargo test --test chapter04 l06_
//!
//! PREDICT before running: Does assigning [u8; 4] move it? Does assigning [String; 2]? Is Clone implicit?
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
}
