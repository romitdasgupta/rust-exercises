//! LESSON 03 — Raw slice reconstruction
//! Run: cargo test --test chapter23 l03_
//!
//! PREDICT before running:
//! 1. Can two returned mutable slices overlap even if their elements are not accessed immediately?
//! 2. What properties do zero-length raw slices still require?
//!
//! IMPLEMENT the tasks; keep the tests.
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
    #[test]
    fn disjoint_regions_mutate_original() {
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
    fn raw_swap_preserves_noncopy_ownership() {
        let mut a = [String::from("a"), String::from("b")];
        assert!(swap_checked(&mut a, 0, 1));
        assert_eq!(a, ["b", "a"]);
        assert!(swap_checked(&mut a, 1, 1));
        assert!(!swap_checked(&mut a, 0, 2));
        assert_eq!(a, ["b", "a"]);
    }
}
