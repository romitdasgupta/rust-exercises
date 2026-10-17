//! LESSON 04 — Index and IndexMut
//! Run: cargo test --test chapter12 l04_
//!
//! PREDICT before executing: Why does Index::index return a reference? Is an indexing panic
//! an appropriate replacement for a checked get API?
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
}
