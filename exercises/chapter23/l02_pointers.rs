//! LESSON 02 — Raw pointer reads with contracts
//! Run: cargo test --test chapter23 l02_
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider: Can creating a raw pointer be safe while dereferencing it is unsafe? Is a non-null pointer alone enough to permit reading?
//! IMPLEMENT the tasks below. Preserve the tests and add an edge case.
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
    use practice::guess;
    #[test]
    fn a_raw_read_does_not_take_ownership() {
        let value = 37_u32;
        let observed = unsafe { read_u32(&value) };
        assert_eq!(observed, 37);
        assert_eq!(value, 37);
        let max = u32::MAX;
        assert_eq!(unsafe { read_u32(&max) }, u32::MAX);
    }
    #[test]
    fn b_safe_boundary_checks_bounds() {
        assert_eq!(at(&[4, 9], 1), Some(9));
        assert_eq!(at(&[4, 9], 0), Some(4));
        assert_eq!(at(&[4, 9], 2), None);
        assert_eq!(at(&[4, 9], usize::MAX), None);
        assert_eq!(at(&[], 0), None);
        assert_eq!(at(&[], usize::MAX), None);
    }
    #[test]
    fn b_reads_from_a_subslice() {
        let values = [1, 2, 3, 4, 5];
        assert_eq!(at(&values[2..], 1), Some(4));
        assert_eq!(
            at(&values[2..4], 2),
            None,
            "bounds come from the slice, not the array"
        );
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    #[test]
    fn p_predictions() {
        // P1. Sizes on a 64-bit target: Option<&u32>, Option<*const u32>, *const [u32].
        let p1: (usize, usize, usize) = guess();
        assert_eq!(
            p1,
            (
                size_of::<Option<&u32>>(),
                size_of::<Option<*const u32>>(),
                size_of::<*const [u32]>()
            ),
            "P1: niches and fat pointers"
        );

        // P2. `add` counts elements. How many BYTES apart are these pointers?
        let values = [1_u32, 2, 3];
        let start = values.as_ptr();
        let p2: usize = guess();
        assert_eq!(
            p2,
            start.wrapping_add(2).addr() - start.addr(),
            "P2: pointer arithmetic"
        );

        // P3. This pointer outlived its referent (creating it is safe; it is
        // never dereferenced). Is it null?
        let dangling = {
            let local = 5_u32;
            &local as *const u32
        };
        let p3: bool = guess();
        assert_eq!(p3, dangling.is_null(), "P3: dangling pointers");
    }
}
