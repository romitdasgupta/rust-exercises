//! LESSON 04 — for, while, break and continue
//! Run: cargo test --test chapter06 l04_
//!
//! PREDICT before running: What does continue skip? When does a while condition run? Does a range include its endpoint?
//! IMPLEMENT the tasks below. Preserve the tests and add an edge case.
//! EXPLAIN afterward: Explain why the sentinel must be checked before filtering negative inputs, and how a loop changes program state.
//! COMPILER / TRANSFER LAB: Rewrite a for loop as while over an index. Deliberately place continue before incrementing that index, reason about nontermination without running it, then repair it. Compare 0..n and 0..=n.
//! Use examples/ch06_scratch.rs; repair intentional failures before committing.

/// A. Scan until the FIRST zero. Skip negatives; sum positives as i64.
/// Use for, break, and continue explicitly. Empty input sums to zero.
fn before_sentinel(values: &[i16]) -> i64 {
    todo!("Implement this task")
}
/// B. Count how many integer halvings bring n to zero. Use while and /=.
fn halvings(mut n: u64) -> u32 {
    todo!("Implement this task")
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a_stop_and_skip_are_distinct() {
        assert_eq!(before_sentinel(&[3, -2, 4, 0, 9]), 7);
        assert_eq!(before_sentinel(&[-1, 0, 7]), 0);
        assert_eq!(before_sentinel(&[]), 0);
    }
    #[test]
    fn b_while_has_explicit_progress() {
        assert_eq!(halvings(0), 0);
        assert_eq!(halvings(1), 1);
        assert_eq!(halvings(8), 4);
        assert_eq!(halvings(u64::MAX), 64);
    }
}
