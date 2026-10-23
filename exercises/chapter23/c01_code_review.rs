//! CODE REVIEW 01 — A teammate's unsafe pull request
//! Run: cargo test --test chapter23 c01_
//! Then: cargo clippy --test chapter23
//!
//! This is the kind of code you will be asked to review at work. It compiles,
//! and most behavior tests pass. Your job:
//! 1. Before running anything, read the code and write a review in NOTES.md:
//!    list every bug, every unnecessary allocation, every unsafe block that
//!    safe code could replace, and every missing or wrong safety argument
//!    (including what happens if something panics halfway).
//! 2. Run the tests. Compare what they found with your review.
//! 3. Fix it. Prefer safe std APIs; keep `unsafe` only where it buys something
//!    and justify it with a SAFETY comment. You MAY make `copy_prefix` a safe
//!    fn; the tests compile either way.
//! 4. Run clippy and resolve its warnings (and rustc's) for this file.

/// Move the last `n` labels (all of them if fewer exist) out of `labels`
/// into a new Vec, keeping their order. Do not clone any String.
fn take_last(labels: &mut Vec<String>, n: usize) -> Vec<String> {
    let n = n.min(labels.len());
    let len = labels.len();
    let mut out = Vec::new();
    unsafe {
        for i in 0..n {
            out.push(std::ptr::read(labels.as_ptr().add(len - 1 - i)));
        }
        labels.set_len(len - n);
    }
    out
}

/// A row-major grid with `cols` cells per row.
struct Grid {
    cells: Vec<i32>,
    cols: usize,
}

impl Grid {
    /// Return the cell at (row, col), or None outside the grid.
    fn get(&self, row: usize, col: usize) -> Option<i32> {
        let i = row * self.cols + col;
        if i < self.cells.len() {
            Some(unsafe { *self.cells.get_unchecked(i) })
        } else {
            None
        }
    }
}

/// Copy as many bytes as fit from `src` to the start of `dst`; return how
/// many were copied.
unsafe fn copy_prefix(dst: &mut [u8], src: &[u8]) -> usize {
    let n = dst.len().min(src.len());
    std::ptr::copy_nonoverlapping(src.as_ptr(), dst.as_mut_ptr(), n);
    n
}

#[cfg(test)]
mod tests {
    use super::*;
    use practice::alloc::count;

    fn labels(n: usize) -> Vec<String> {
        (0..n).map(|i| format!("label{i}")).collect()
    }

    #[test]
    fn a_take_last_keeps_order() {
        let mut v = labels(4);
        assert_eq!(take_last(&mut v, 2), ["label2", "label3"]);
        assert_eq!(v, ["label0", "label1"]);
    }

    #[test]
    fn a_take_last_handles_short_and_empty_requests() {
        let mut v = labels(2);
        assert!(take_last(&mut v, 0).is_empty());
        assert_eq!(take_last(&mut v, 9), ["label0", "label1"]);
        assert!(v.is_empty());
        assert!(take_last(&mut v, 1).is_empty());
    }

    #[test]
    fn a_take_last_moves_without_regrowing() {
        let mut v = labels(50);
        let tenth = v[10].as_ptr();
        let (out, allocations) = count(|| take_last(&mut v, 40));
        assert_eq!(out.len(), 40);
        assert!(out.iter().any(|s| s.as_ptr() == tenth), "move, don't clone");
        assert!(
            allocations <= 1,
            "one Vec for 40 moved labels; got {allocations} allocations"
        );
    }

    fn grid() -> Grid {
        // 2 rows x 3 columns:
        // 1 2 3
        // 4 5 6
        Grid {
            cells: vec![1, 2, 3, 4, 5, 6],
            cols: 3,
        }
    }

    #[test]
    fn b_grid_reads_cells() {
        let g = grid();
        assert_eq!(g.get(0, 0), Some(1));
        assert_eq!(g.get(1, 2), Some(6));
        assert_eq!(g.get(2, 0), None);
    }

    #[test]
    fn b_grid_rejects_columns_past_the_edge() {
        let g = grid();
        assert_eq!(g.get(0, 3), None, "(0, 3) is outside, not cell (1, 0)");
        assert_eq!(g.get(0, 5), None);
    }

    #[test]
    fn b_grid_rejects_huge_coordinates() {
        let g = grid();
        assert_eq!(g.get(usize::MAX, 0), None);
        assert_eq!(g.get(0, usize::MAX), None);
        assert_eq!(g.get(usize::MAX / 2, 2), None);
    }

    #[test]
    #[allow(unused_unsafe)]
    fn c_copy_prefix_copies_the_overlap() {
        let mut dst = [0_u8; 4];
        assert_eq!(unsafe { copy_prefix(&mut dst, b"ab") }, 2);
        assert_eq!(dst, *b"ab\0\0");
        assert_eq!(unsafe { copy_prefix(&mut dst, b"wxyz!") }, 4);
        assert_eq!(dst, *b"wxyz");
        assert_eq!(unsafe { copy_prefix(&mut [], b"x") }, 0);
    }
}
