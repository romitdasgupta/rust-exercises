//! LESSON 05 — Layout, unsafe traits, unions
//! Run: cargo test --test chapter23 l05_
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider: Can a Copy type contain uninitialized padding? Does reading another union field validate the new bit pattern?
//! IMPLEMENT the tasks below. Preserve the tests and add an edge case.
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
    use practice::{alloc::count, guess};
    #[test]
    fn a_byte_view_matches_native_order() {
        let n = 0x12345678_u32;
        assert_eq!(bytes(&n), n.to_ne_bytes());
        assert_eq!(bytes(&n).as_ptr(), (&n as *const u32).cast::<u8>());
        assert_eq!(bytes(&0_u32), [0; 4]);
    }
    #[test]
    fn a_byte_view_borrows_without_allocating() {
        let n = u32::MAX;
        let (len, allocations) = count(|| bytes(&n).len());
        assert_eq!((len, allocations), (4, 0));
    }
    #[test]
    fn b_layout_is_measured_not_guessed() {
        let (size, align, offset) = header_layout();
        assert_eq!(size, size_of::<Header>());
        assert_eq!(align, align_of::<Header>());
        assert_eq!(offset, std::mem::offset_of!(Header, count));
        assert_eq!(offset % align_of::<u32>(), 0);
        assert!(size >= offset + size_of::<u32>());
    }
    #[test]
    fn c_union_view_preserves_bits() {
        for v in [
            0.0_f32,
            -0.0,
            1.5,
            f32::INFINITY,
            f32::from_bits(0x7fc00001),
        ] {
            assert_eq!(float_bits(v), v.to_bits());
        }
        assert_eq!(float_bits(1.0), 0x3f80_0000);
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    #[test]
    fn p_predictions() {
        #[repr(C)]
        struct Packet {
            a: u8,
            b: u16,
            c: u8,
        }
        // P1. repr(C): size, alignment, and offset of `c`.
        let p1: (usize, usize, usize) = guess();
        assert_eq!(
            p1,
            (
                size_of::<Packet>(),
                align_of::<Packet>(),
                std::mem::offset_of!(Packet, c)
            ),
            "P1: repr(C) padding"
        );

        // P2. -0.0 and 0.0: equal as floats? Same bits?
        let p2: (bool, bool) = guess();
        assert_eq!(
            p2,
            (-0.0_f32 == 0.0, (-0.0_f32).to_bits() == 0.0_f32.to_bits()),
            "P2: float bits"
        );

        // P3. Sizes of Option<bool> and Option<u8>.
        let p3: (usize, usize) = guess();
        assert_eq!(
            p3,
            (size_of::<Option<bool>>(), size_of::<Option<u8>>()),
            "P3: Option<bool> vs Option<u8>"
        );
    }
}
