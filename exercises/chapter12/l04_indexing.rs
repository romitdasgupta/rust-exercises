//! LESSON 04 — Index and IndexMut
//! Run: cargo test --test chapter12 l04_
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider: Why does Index::index return a reference? Is
//! an indexing panic an appropriate replacement for a checked get API?
//!
//! IMPLEMENT the documented tasks, then keep the tests and add an edge case.
//! EXPLAIN afterward: Which trait enables reads and which enables writes? Why is returning
//! a reference to a temporary incorrect?
//!
//! COMPILER / MANUAL LAB (use examples/ch12_scratch.rs):
//! Attempt to implement Index with Output=str and return an allocated String. Then
//! implement Index<Range<usize>> for a wrapper over Vec<i32> and observe the unsized
//! Output=[i32].
//!
//! TRANSFER: invent a new edge case, predict it, then add a check. A lesson is
//! complete only when its tests, predictions, lab, and explanation are done.

use std::ops::{Index, IndexMut};
#[derive(Debug)]
struct Grid([i32; 6]);
impl Index<(usize, usize)> for Grid {
    type Output = i32;
    /// A. Row-major 2x3 read. Panic "grid coordinate out of bounds" for invalid coordinates.
    fn index(&self, (row, col): (usize, usize)) -> &i32 {
        todo!("Task 1: implement the documented contract")
    }
}
impl IndexMut<(usize, usize)> for Grid {
    /// B. Same coordinate contract, permitting in-place mutation.
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut i32 {
        todo!("Task 2: implement the documented contract")
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use practice::guess;
    use std::collections::VecDeque;
    #[test]
    fn a_index_maps_rows_and_columns() {
        let g = Grid([0, 1, 2, 3, 4, 5]);
        assert_eq!(g[(0, 2)], 2);
        assert_eq!(g[(1, 0)], 3);
        assert_eq!(g[(1, 2)], 5);
    }
    #[test]
    fn b_mutable_index_changes_one_element() {
        let mut g = Grid([0; 6]);
        g[(1, 1)] = 9;
        assert_eq!(g.0, [0, 0, 0, 0, 9, 0]);
    }
    #[test]
    #[should_panic(expected = "grid coordinate out of bounds")]
    fn a_rejects_invalid_column() {
        let _ = Grid([0; 6])[(0, 3)];
    }
    #[test]
    #[should_panic(expected = "grid coordinate out of bounds")]
    fn b_rejects_invalid_row() {
        Grid([0; 6])[(usize::MAX, 0)] = 1;
    }

    #[test]
    #[should_panic(expected = "grid coordinate out of bounds")]
    fn b_rejects_invalid_column_even_when_the_flat_index_fits() {
        // (0, 3) would be element 3 in a flat array: the row-major formula alone
        // does not catch it.
        Grid([0; 6])[(0, 3)] = 1;
    }

    #[test]
    fn b_writes_reach_the_same_element_reads_use() {
        let mut g = Grid([0; 6]);
        for row in 0..2 {
            for col in 0..3 {
                g[(row, col)] = (row * 10 + col) as i32;
            }
        }
        assert_eq!(g.0, [0, 1, 2, 10, 11, 12]);
        assert_eq!(g[(1, 1)], 11);
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    #[test]
    fn p_predictions() {
        // P1. Index with ranges: a slice of a slice, and a byte range of a str.
        let p1: (i32, &str) = guess();
        assert_eq!(
            p1,
            ([10, 20, 30][1..][0], &"héllo"[1..3]),
            "P1: Index<Range<usize>>"
        );

        // P2. VecDeque implements Index too. After push_front, what is at 0 and 2?
        let mut d = VecDeque::from([1, 2]);
        d.push_front(0);
        let p2: (i32, i32) = guess();
        assert_eq!(p2, (d[0], d[2]), "P2: logical, not physical, positions");

        // P3. `v[i]` means `*v.index(i)`. Does `&v[1]` point into the array itself?
        let v = [String::from("a"), String::from("b")];
        let p3: bool = guess();
        assert_eq!(
            p3,
            std::ptr::eq(&v[1], v.as_ptr().wrapping_add(1)),
            "P3: Index returns a reference"
        );
    }
}
