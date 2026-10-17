//! LESSON 04 — Constants and statics
//! Run: cargo test --test chapter08 l04_
//!
//! PREDICT before running: Does every use of a const refer to one stable address? What distinguishes a static item lifetime from the lifetime of a local binding?
//! IMPLEMENT the tasks below. Preserve the tests and add an edge case.
//! EXPLAIN afterward: Explain shared static storage and why const is a value definition, without assuming identity for constant references.
//! COMPILER / TRANSFER LAB: Define a const and a static array in scratch. Borrow each repeatedly, inspect values and static pointer identity, and explain why const addresses are not an API guarantee. Try a non-const function call in a const initializer; repair with const fn. Discuss why mutable static requires special care and prefer atomics or locks later.
//! Use examples/ch08_scratch.rs; repair intentional failures before committing.

const HEADER_BYTES: usize = 4;
static MAGIC: [u8; 4] = *b"RUST";
/// A. Return bytes for n fixed-size headers as Option, rejecting usize overflow.
/// Use HEADER_BYTES rather than repeating a literal.
fn header_storage(n: usize) -> Option<usize> {
    todo!("Implement this task")
}
/// B. Return a static borrow of the single MAGIC array, not an equal copy.
fn magic() -> &'static [u8; 4] {
    todo!("Implement this task")
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a_const_drives_checked_size() {
        assert_eq!(header_storage(0), Some(0));
        assert_eq!(header_storage(3), Some(12));
        assert_eq!(header_storage(usize::MAX), None);
    }
    #[test]
    fn b_static_has_stable_identity() {
        assert_eq!(magic(), b"RUST");
        assert!(std::ptr::eq(magic(), &MAGIC));
        assert!(std::ptr::eq(magic(), magic()));
    }
}
