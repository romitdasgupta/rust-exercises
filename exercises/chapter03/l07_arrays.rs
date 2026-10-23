//! LESSON 07 — Fixed-size, inline sequences
//! Run: cargo test --test chapter03 l07_
//!
//! PREDICT: first answer `p_predictions` at the bottom (replace each `guess()`
//! with your answer). Then consider: Are `[u8; 8]` and `[u8; 9]` the same type?
//! What is `[0_u8; 8]`? How many bytes do `[[i16; 3]; 2]` occupy? Does an array
//! inherently require a separate heap allocation? Is a local array guaranteed
//! to live on the stack?
//!
//! EXPLAIN: Why must an array length be known at compile time? How can a nested
//! array live inside another value? Compare indexing with `get` on invalid input.
//! Avoid assuming a particular physical location after compiler optimization.
//!
//! COMPILER LAB: try `[0_u8; n]` where n is a function parameter. Try assigning
//! `[0_u8; 9]` to `[u8; 8]`. Separately, write a function that uses a parameter
//! as an array index and call it with an out-of-range index. Which is a type
//! error, and which is a runtime panic? What if the bad index is a constant?

/// A. Initialize eight zero bytes, then set the first to 0x7e and last to 0x7f.
fn framed_buffer() -> [u8; 8] {
    todo!("A: use repeat-value initialization and indexed mutation")
}

/// B. Return two identical rows using repeat-value array syntax.
fn repeated_rows(row: [i16; 3]) -> [[i16; 3]; 2] {
    todo!("B: an array can contain arrays inline")
}

/// C. Return an optional borrow of a byte; invalid indices must return None.
fn lookup(buffer: &[u8; 8], index: usize) -> Option<&u8> {
    todo!("C: choose checked access rather than a panicking index")
}

#[cfg(test)]
mod tests {
    use super::*;
    use practice::{alloc::count, guess};

    #[test]
    fn a_all_bytes_are_initialized() {
        let buffer = framed_buffer();
        assert_eq!(buffer, [0x7e, 0, 0, 0, 0, 0, 0, 0x7f]);
        assert_eq!(std::mem::size_of_val(&buffer), 8);
    }

    #[test]
    fn b_rows_are_nested_values() {
        let grid = repeated_rows([2, -3, 5]);
        assert_eq!(grid, [[2, -3, 5], [2, -3, 5]]);
        assert_eq!(std::mem::size_of_val(&grid), 12);
    }

    #[test]
    fn c_lookup_handles_boundaries_without_panicking() {
        let buffer = [10, 20, 30, 40, 50, 60, 70, 80];
        assert_eq!(lookup(&buffer, 0), Some(&10));
        assert_eq!(lookup(&buffer, 7), Some(&80));
        assert_eq!(lookup(&buffer, 8), None);
        assert_eq!(lookup(&buffer, usize::MAX), None);
        assert!(std::ptr::eq(lookup(&buffer, 3).unwrap(), &buffer[3]));
    }

    #[test]
    fn a_b_arrays_do_not_touch_the_heap() {
        let (_, allocations) = count(framed_buffer);
        assert_eq!(allocations, 0, "an array needs no heap allocation");
        let (_, allocations) = count(|| repeated_rows([1, 2, 3]));
        assert_eq!(allocations, 0, "nested arrays are stored inline");
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    #[test]
    fn p_predictions() {
        // P1. Sizes in bytes of a nested array and an empty array.
        let p1: (usize, usize) = guess();
        assert_eq!(
            p1,
            (size_of::<[[u32; 4]; 3]>(), size_of::<[u8; 0]>()),
            "P1: array sizes"
        );

        // P2. Which length is the outer one?
        let grid = [[0_u8; 3]; 2];
        let p2: (usize, usize) = guess();
        assert_eq!(p2, (grid.len(), grid[0].len()), "P2: nested lengths");

        // P3. Checked access at the last index and one past it.
        let a = [10, 20, 30];
        let p3: (Option<&i32>, Option<&i32>) = guess();
        assert_eq!(p3, (a.get(2), a.get(3)), "P3: get");
    }
}
