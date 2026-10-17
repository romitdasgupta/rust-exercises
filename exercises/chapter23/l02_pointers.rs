//! LESSON 02 — Raw pointer reads with contracts
//! Run: cargo test --test chapter23 l02_
//!
//! PREDICT before running:
//! 1. Can creating a raw pointer be safe while dereferencing it is unsafe?
//! 2. Is a non-null pointer alone enough to permit reading?
//!
//! IMPLEMENT the tasks; keep the tests.
//! EXPLAIN: List every caller obligation and show how the safe slice wrapper discharges them.
//! COMPILER LAB (examples/ch23_scratch.rs): Create raw pointers from valid references and print addresses without dereferencing arbitrary addresses. Explain why an address value is not an allocation proof.

/// A. Read and copy one u32.
/// # Safety
/// p must be non-null, aligned for u32, and point to an initialized u32 in a
/// live allocation readable for this operation. There must be no concurrent
/// mutation or exclusive reference conflicting with this read. Do not free it.
unsafe fn read_u32(p: *const u32) -> u32 {
    todo!("A: implement the contract")
}
/// B. Safe boundary: return None for an invalid index. Otherwise derive a raw
/// pointer from this slice and call read_u32; do not form a pointer out of range.
fn at(values: &[u32], index: usize) -> Option<u32> {
    todo!("B: implement the contract")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn safe_boundary_checks_bounds() {
        assert_eq!(at(&[4, 9], 1), Some(9));
        assert_eq!(at(&[4, 9], 2), None);
        assert_eq!(at(&[], usize::MAX), None);
    }
    #[test]
    fn raw_read_does_not_take_ownership() {
        let value = 37_u32;
        let observed = unsafe { read_u32(&value) };
        assert_eq!(observed, 37);
        assert_eq!(value, 37);
    }
}
