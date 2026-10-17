//! LESSON 05 — Layout, unsafe traits, unions
//! Run: cargo test --test chapter23 l05_
//!
//! PREDICT before running:
//! 1. Can a Copy type contain uninitialized padding?
//! 2. Does reading another union field validate the new bit pattern?
//!
//! IMPLEMENT the tasks; keep the tests.
//! EXPLAIN: State the unsafe trait contract and explain why bool would violate an all-bit-patterns-valid promise.
//! COMPILER LAB (examples/ch23_scratch.rs): Inspect size_of, align_of, and offset_of for repr(C) and default-layout structs. Do not infer a general ABI from one observed layout.

use std::mem::{align_of, size_of};
/// # Safety
/// Implementors must be Copy, have no uninitialized padding, and every bit
/// pattern of their size must be a valid initialized value. This exercise only
/// implements u32; do not implement for bool, references, or arbitrary structs.
unsafe trait Plain: Copy {}
// SAFETY: u32 has no padding and every 32-bit pattern is valid.
unsafe impl Plain for u32 {}
/// A. Borrow initialized bytes without allocation. The returned lifetime must
/// remain tied to value, and you must not create a mutable alias.
fn bytes<T: Plain>(value: &T) -> &[u8] {
    todo!("A: implement the contract")
}
#[repr(C)]
struct Header {
    tag: u8,
    count: u32,
}
/// B. Report actual size, alignment, and count offset for this target.
fn header_layout() -> (usize, usize, usize) {
    todo!("B: implement the contract")
}
union Word {
    integer: u32,
    float: f32,
}
/// C. View an f32 bit pattern as u32 through a union. All u32 bit patterns are
/// valid; do not generalize this operation to types with restricted patterns.
fn float_bits(value: f32) -> u32 {
    todo!("C: implement the contract")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn byte_view_matches_native_order() {
        let n = 0x12345678_u32;
        assert_eq!(bytes(&n), n.to_ne_bytes());
        assert_eq!(bytes(&n).as_ptr(), (&n as *const u32).cast::<u8>());
    }
    #[test]
    fn layout_is_measured_not_guessed() {
        let (size, align, offset) = header_layout();
        assert_eq!(size, size_of::<Header>());
        assert_eq!(align, align_of::<Header>());
        assert_eq!(offset, std::mem::offset_of!(Header, count));
        assert_eq!(offset % align_of::<u32>(), 0);
        assert!(size >= offset + size_of::<u32>());
    }
    #[test]
    fn union_view_preserves_bits() {
        for v in [
            0.0_f32,
            -0.0,
            1.5,
            f32::INFINITY,
            f32::from_bits(0x7fc00001),
        ] {
            assert_eq!(float_bits(v), v.to_bits());
        }
    }
}
