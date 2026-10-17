//! LESSON 01 — C representations
//! Run: cargo test --test chapter24 l01_
//!
//! PREDICT before running:
//! 1. Does repr(C) make a Rust String field C-compatible?
//! 2. Is C long guaranteed the same size as Rust i64 on every target?
//!
//! IMPLEMENT the tasks; keep the tests.
//! EXPLAIN: Explain primitive aliases, field layout, padding, and why a raw struct byte dump is not a portable wire format.
//! COMPILER LAB (examples/ch24_scratch.rs): Compare c_int/c_long/c_char sizes to Rust primitives on this platform, then inspect the same C struct with a tiny C sizeof/offsetof program in the lab.

use std::ffi::{c_char, c_int, c_uint};
#[repr(C)]
#[derive(Debug, PartialEq)]
struct Reading {
    kind: c_char,
    count: c_uint,
    value: c_int,
}
/// A. Construct a C-layout record using C aliases. No casts or assumptions about
/// signed char are required: accept exactly the field types in the signature.
fn reading(kind: c_char, count: c_uint, value: c_int) -> Reading {
    todo!("A: implement the contract")
}
/// B. Return size, alignment, and the value field's byte offset for this target.
fn layout() -> (usize, usize, usize) {
    todo!("B: implement the contract")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn field_types_and_values_preserved() {
        assert_eq!(
            reading(1, 7, -9),
            Reading {
                kind: 1,
                count: 7,
                value: -9
            }
        );
    }
    #[test]
    fn layout_accounts_for_alignment() {
        let (size, align, offset) = layout();
        assert_eq!(size, std::mem::size_of::<Reading>());
        assert_eq!(align, std::mem::align_of::<Reading>());
        assert_eq!(offset, std::mem::offset_of!(Reading, value));
        assert_eq!(offset % std::mem::align_of::<c_int>(), 0);
        assert!(size >= offset + std::mem::size_of::<c_int>());
    }
}
