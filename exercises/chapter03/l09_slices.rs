//! LESSON 09 — Borrow a region instead of owning a collection
//! Run: cargo test --test chapter03 l09_
//!
//! PREDICT: Can the same `&[i32]` parameter accept an array, a Vec, and part of
//! either? What two pieces of information describe a slice reference? Does
//! making a subslice allocate or copy its elements?
//!
//! EXPLAIN: Distinguish `[i32]`, `[i32; 4]`, and `&[i32]`. Why prefer a slice
//! parameter over `&Vec<i32>`? Why can both arrays and vectors call `sort`?
//!
//! COMPILER LAB: pass an array and a vector to the same slice-taking function.
//! Change its parameter to `&Vec<i32>` and try an array and `&v[1..]` again.
//! Restore the general signature and explain the difference.

/// A. Borrow all elements except the first and last. For lengths 0, 1, or 2,
/// return an empty borrowed slice. No allocation, copying, or leaking memory.
fn interior(values: &[i32]) -> &[i32] {
    todo!("A: handle short inputs before forming the range")
}

/// B. Sum a borrowed sequence using a loop. Empty input sums to zero.
/// The caller guarantees the sum fits in i32; overflow policy is not this task.
fn sum(values: &[i32]) -> i32 {
    todo!("B: one API for arrays, vectors, and subslices")
}

/// C. Sort only start..end in place, leaving elements outside the range alone.
/// The caller guarantees 0 <= start <= end <= values.len().
fn sort_region(values: &mut [i32], start: usize, end: usize) {
    todo!("C: use a mutable subslice and its sort method")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_interior_is_a_view_of_existing_storage() {
        let values = [10, 20, 30, 40, 50];
        let middle = interior(&values);
        assert_eq!(middle, [20, 30, 40]);
        assert_eq!(middle.as_ptr(), values[1..].as_ptr());
        assert!(interior(&[]).is_empty());
        assert!(interior(&[7]).is_empty());
        assert!(interior(&[7, 8]).is_empty());
        assert_eq!(interior(&[1, 2, 3]), [2]);
    }

    #[test]
    fn b_one_function_accepts_different_storage() {
        let array = [4, -1, 7, 2];
        let vector = vec![4, -1, 7, 2];
        assert_eq!(sum(&array), 12);
        assert_eq!(sum(&vector), 12);
        assert_eq!(sum(&vector[1..3]), 6);
        assert_eq!(sum(&[]), 0);
    }

    #[test]
    fn c_mutable_subslice_edits_only_its_region() {
        let mut values = [90, 4, 2, 3, 1, 80];
        sort_region(&mut values, 1, 5);
        assert_eq!(values, [90, 1, 2, 3, 4, 80]);
        sort_region(&mut values, 2, 2);
        assert_eq!(values, [90, 1, 2, 3, 4, 80]);
        let mut vector = vec![2, -1, 2, 0];
        sort_region(&mut vector, 0, 4);
        assert_eq!(vector, [-1, 0, 2, 2]);
        sort_region(&mut [], 0, 0);
    }
}
