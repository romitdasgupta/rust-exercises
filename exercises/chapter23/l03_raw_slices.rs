//! LESSON 03 — Raw slice reconstruction
//! Run: cargo test --test chapter23 l03_
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider: Can two returned mutable slices overlap even if their elements are not accessed immediately? What properties do zero-length raw slices still require?
//! IMPLEMENT the tasks below. Preserve the tests and add an edge case.
//! EXPLAIN: Prove bounds, provenance, alignment, initialization, lifetime, and nonoverlap separately.
//! COMPILER LAB (examples/ch23_scratch.rs): Try implementing the split with two ordinary overlapping index expressions and compare borrow errors. Do not bypass them without a nonoverlap proof.

/// A. Safe checked split, implemented with raw pointers and from_raw_parts_mut.
/// For mid > len return None. Otherwise return exactly the two disjoint regions.
/// Input's unique borrow lasts for both outputs; no allocation and no cloning.
fn split_checked<T>(values: &mut [T], mid: usize) -> Option<(&mut [T], &mut [T])> {
    todo!("A: implement the contract")
}
/// B. Swap two elements via raw pointer operations after checking BOTH indices.
/// Equal indices are valid. Return false with no mutation for any invalid index.
fn swap_checked<T>(values: &mut [T], a: usize, b: usize) -> bool {
    todo!("B: implement the contract")
}

#[cfg(test)]
mod tests {
    use super::*;
    use practice::{alloc::count, guess};
    #[test]
    fn a_disjoint_regions_mutate_original() {
        let mut a = [1, 2, 3, 4];
        let (l, r) = split_checked(&mut a, 2).unwrap();
        l[0] = 9;
        r[0] = 8;
        assert_eq!(a, [9, 2, 8, 4]);
        assert!(split_checked(&mut a, 5).is_none());
        assert_eq!(
            split_checked::<u8>(&mut [], 0).unwrap(),
            (&mut [][..], &mut [][..])
        );
    }
    #[test]
    fn a_endpoints_and_exact_regions() {
        let mut a = [1, 2, 3];
        let base = a.as_ptr();
        let (l, r) = split_checked(&mut a, 0).unwrap();
        assert_eq!((l.len(), r.len()), (0, 3));
        assert_eq!(r.as_ptr(), base);
        let (l, r) = split_checked(&mut a, 3).unwrap();
        assert_eq!((l.len(), r.len()), (3, 0));
        assert_eq!(l.as_ptr(), base);
        let (l, r) = split_checked(&mut a, 1).unwrap();
        assert_eq!((l, r), (&mut [1][..], &mut [2, 3][..]));
        assert!(split_checked(&mut a, usize::MAX).is_none());
    }
    #[test]
    fn a_split_does_not_copy() {
        let mut labels = vec![String::from("a"), String::from("b")];
        let first = labels[0].as_ptr();
        let (parts, allocations) =
            count(|| split_checked(&mut labels, 1).map(|(l, r)| (l.len(), r.len())));
        assert_eq!((parts, allocations), (Some((1, 1)), 0));
        assert_eq!(labels[0].as_ptr(), first);
    }
    #[test]
    fn b_raw_swap_preserves_noncopy_ownership() {
        let mut a = [String::from("a"), String::from("b")];
        assert!(swap_checked(&mut a, 0, 1));
        assert_eq!(a, ["b", "a"]);
        assert!(swap_checked(&mut a, 1, 1));
        assert!(!swap_checked(&mut a, 0, 2));
        assert_eq!(a, ["b", "a"]);
    }
    #[test]
    fn b_either_index_may_be_invalid() {
        let mut a = [String::from("x"), String::from("y")];
        let px = a[0].as_ptr();
        assert!(!swap_checked(&mut a, 2, 0), "first index out of range");
        assert!(!swap_checked(&mut a, usize::MAX, usize::MAX));
        assert_eq!(a, ["x", "y"]);
        let (swapped, allocations) = count(|| swap_checked(&mut a, 1, 0));
        assert_eq!((swapped, allocations), (true, 0));
        assert_eq!(a[1].as_ptr(), px, "swap moves the Strings, not their text");
        let mut empty: [u8; 0] = [];
        assert!(!swap_checked(&mut empty, 0, 0));
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    #[test]
    fn p_predictions() {
        // P1. An empty Vec has not allocated. Is its pointer's address zero,
        // and what is that address modulo the alignment of u64?
        let v: Vec<u64> = Vec::new();
        let p1: (bool, usize) = guess();
        assert_eq!(
            p1,
            (
                v.as_ptr().addr() == 0,
                v.as_ptr().addr() % align_of::<u64>()
            ),
            "P1: the pointer of an empty Vec"
        );

        // P2. How many BYTES separate the two halves of this split?
        let mut words = [0_u16; 5];
        let (left, right) = words.split_at_mut(2);
        let p2: usize = guess();
        assert_eq!(
            p2,
            right.as_ptr().addr() - left.as_ptr().addr(),
            "P2: split offset"
        );

        // P3. ptr::swap with the SAME pointer twice. What does the array hold?
        let mut pair = [1, 2];
        let p = pair.as_mut_ptr();
        // SAFETY: p is valid for reads and writes; ptr::swap allows overlap.
        unsafe { std::ptr::swap(p, p) };
        let p3: [i32; 2] = guess();
        assert_eq!(p3, pair, "P3: swapping a place with itself");
    }
}
