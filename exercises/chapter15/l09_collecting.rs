//! LESSON 09 — Collections, extension, and partition
//! Run: cargo test --test chapter15 l09_
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider: How does collect know its destination type?
//! Does collecting Result values retain successful values after the first Err?
//!
//! IMPLEMENT the documented tasks, then keep the tests and add an edge case.
//! EXPLAIN afterward: Explain FromIterator versus Extend. Why does partition require two
//! output collections of compatible type?
//!
//! COMPILER / MANUAL LAB (use examples/ch15_scratch.rs):
//! Collect the same numbers into Vec, HashSet, BTreeSet, String (after mapping to char),
//! and Result<Vec<_>,_>. Record ordering and duplicate differences. Implement Extend<i32>
//! for a wrapper as a transfer task.
//!
//! TRANSFER: invent a new edge case, predict it, then add a check. A lesson is
//! complete only when its tests, predictions, lab, and explanation are done.

/// A. Trim and parse every token as i32 using collect into Result.
/// Preserve order and return the first parse error; empty input succeeds.
fn parse_all(tokens: &[&str]) -> Result<Vec<i32>, std::num::ParseIntError> {
    todo!("Task 1: implement the documented contract")
}
/// B. Return (even,odd), each preserving input order.
fn partition_parity(values: &[i32]) -> (Vec<i32>, Vec<i32>) {
    todo!("Task 2: implement the documented contract")
}
/// C. Extend an existing vector with any IntoIterator<Item=i32>.
fn append_all<I: IntoIterator<Item = i32>>(out: &mut Vec<i32>, items: I) {
    todo!("Task 3: implement the documented contract")
}
#[cfg(test)]
mod tests {
    use super::*;
    use practice::{alloc::count, guess};
    use std::cell::Cell;
    use std::collections::BTreeSet;
    use std::num::IntErrorKind;
    #[test]
    fn a_collect_transposes_results() {
        assert_eq!(parse_all(&[" 1 ", "-2"]).unwrap(), [1, -2]);
        assert!(parse_all(&["1", "bad", "2"]).is_err());
        assert_eq!(parse_all(&[]).unwrap(), Vec::<i32>::new());
        let first_error = parse_all(&["1", "  ", "x"]).unwrap_err();
        assert_eq!(
            first_error.kind(),
            &IntErrorKind::Empty,
            "report the first error"
        );
    }
    #[test]
    fn b_partition_keeps_stable_order() {
        assert_eq!(
            partition_parity(&[3, 2, -4, 1, 0]),
            (vec![2, -4, 0], vec![3, 1])
        );
    }
    #[test]
    fn c_extend_keeps_existing_elements() {
        let mut out = vec![9];
        append_all(&mut out, 0..3);
        append_all(&mut out, [4, 5]);
        assert_eq!(out, [9, 0, 1, 2, 4, 5]);
        append_all(&mut out, std::iter::empty());
        assert_eq!(out.len(), 6);
    }
    #[test]
    fn c_extend_reserves_from_the_size_hint() {
        let mut out = vec![9];
        let (_, allocations) = count(|| append_all(&mut out, 0..1000));
        assert_eq!(out.len(), 1001);
        assert!(
            allocations <= 1,
            "an exact-size source needs one reservation; got {allocations}"
        );
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    /// A wrong prediction is useful: read the failure, then explain it in NOTES.md.
    #[test]
    fn p_predictions() {
        // P1. Collecting into Result: (the result, how many tokens were parsed).
        let parsed = Cell::new(0);
        let result: Result<Vec<i32>, &str> = ["1", "x", "y"]
            .iter()
            .map(|s| {
                parsed.set(parsed.get() + 1);
                s.parse::<i32>().map_err(|_| *s)
            })
            .collect();
        let p1: (Result<Vec<i32>, &str>, usize) = guess();
        assert_eq!(
            p1,
            (result, parsed.get()),
            "P1: collect stops at the first Err"
        );

        // P2. Collect [3, 1, 3, 2] into a BTreeSet, then list it.
        let set: BTreeSet<i32> = [3, 1, 3, 2].into_iter().collect();
        let p2: Vec<i32> = guess();
        assert_eq!(
            p2,
            set.into_iter().collect::<Vec<_>>(),
            "P2: the destination decides"
        );

        // P3. Summing Options: (with a None, without one).
        let p3: (Option<i32>, Option<i32>) = guess();
        assert_eq!(
            p3,
            (
                [Some(1), None, Some(3)].into_iter().sum(),
                [Some(1), Some(3)].into_iter().sum()
            ),
            "P3: Sum for Option short-circuits"
        );
    }
}
