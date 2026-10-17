//! LESSON 09 — Collections, extension, and partition
//! Run: cargo test --test chapter15 l09_
//!
//! PREDICT before executing: How does collect know its destination type? Does collecting
//! Result values retain successful values after the first Err?
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
    #[test]
    fn a_collect_transposes_results() {
        assert_eq!(parse_all(&[" 1 ", "-2"]).unwrap(), [1, -2]);
        assert!(parse_all(&["1", "bad", "2"]).is_err());
        assert_eq!(parse_all(&[]).unwrap(), Vec::<i32>::new());
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
    }
}
