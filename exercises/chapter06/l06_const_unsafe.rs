//! LESSON 06 — const and unsafe expression blocks
//! Run: cargo test --test chapter06 l06_
//!
//! PREDICT before running: When is an inline const block evaluated? Does putting safe code inside unsafe make it faster?
//! IMPLEMENT the tasks below. Preserve the tests and add an edge case.
//! EXPLAIN afterward: Explain which operations require unsafe and what facts a caller must prove. Describe why these tasks need no unchecked operations.
//! COMPILER / TRANSFER LAB: Use `const { 6 * 7 }` in an array length, then try referring to a runtime local inside the const block. Separately obtain a raw pointer from a live i32 and read it in one small unsafe block; state validity, alignment, initialization, and aliasing facts. Never dereference a null or dangling pointer, even as an experiment.
//! Use examples/ch06_scratch.rs; repair intentional failures before committing.

/// A. Produce an all-zero 12-byte frame. Express its length using an inline
/// const block whose calculation is 3 fields * 4 bytes, rather than literal 12.
fn zero_frame() -> [u8; 12] {
    todo!("Implement this task")
}
/// B. Return a copied byte at index if it exists. Do not use unsafe: safe
/// checked indexing is the right tool when no special invariant is provided.
fn checked_byte(bytes: &[u8], index: usize) -> Option<u8> {
    todo!("Implement this task")
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a_const_expression_sets_array_length() {
        assert_eq!(zero_frame(), [0; 12]);
    }
    #[test]
    fn b_safe_lookup_handles_boundaries() {
        assert_eq!(checked_byte(&[4, 8], 1), Some(8));
        assert_eq!(checked_byte(&[4, 8], 2), None);
        assert_eq!(checked_byte(&[], 0), None);
    }
}
