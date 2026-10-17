//! LESSON 04 — Stopping, peeking, and resuming
//! Run: cargo test --test chapter15 l04_
//!
//! PREDICT before executing: Does take_while preserve the first rejected item? Does
//! Peekable::peek consume it? Is every iterator permanently exhausted after None?
//!
//! IMPLEMENT the documented tasks, then keep the tests and add an edge case.
//! EXPLAIN afterward: Why does by_ref permit continued use of the original iterator? What
//! guarantee does fuse add?
//!
//! COMPILER / MANUAL LAB (use examples/ch15_scratch.rs):
//! Use take_while on [1,2,0,3] by_ref and inspect the remaining items. Write an iterator
//! yielding Some(1), None, Some(2), then compare it with fuse().
//!
//! TRANSFER: invent a new edge case, predict it, then add a check. A lesson is
//! complete only when its tests, predictions, lab, and explanation are done.

/// A. Skip leading zeros, then keep consecutive nonzero values until the next zero.
fn active_region(values: &[i32]) -> Vec<i32> {
    todo!("Task 1: implement the documented contract")
}
/// B. Consume consecutive leading ASCII digits using peekable, leaving the first
/// nondigit in the returned remainder. Empty prefix is valid. Work with chars.
fn digit_prefix(text: &str) -> (String, String) {
    todo!("Task 2: implement the documented contract")
}
/// C. Consume first two items with by_ref().take(2), then collect the remainder.
fn split_two<I: Iterator<Item = i32>>(mut input: I) -> (Vec<i32>, Vec<i32>) {
    todo!("Task 3: implement the documented contract")
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a_skip_and_take_while_have_distinct_boundaries() {
        assert_eq!(active_region(&[0, 0, 3, 4, 0, 5]), [3, 4]);
        assert!(active_region(&[0, 0]).is_empty());
        assert_eq!(active_region(&[2, 3]), [2, 3]);
    }
    #[test]
    fn b_peek_keeps_the_delimiter() {
        assert_eq!(digit_prefix("12🦀3"), ("12".into(), "🦀3".into()));
        assert_eq!(digit_prefix("x9"), ("".into(), "x9".into()));
        assert_eq!(digit_prefix(""), ("".into(), "".into()));
    }
    #[test]
    fn c_by_ref_borrows_instead_of_consuming_the_iterator() {
        assert_eq!(split_two(1..6), (vec![1, 2], vec![3, 4, 5]));
        assert_eq!(split_two(std::iter::once(9)), (vec![9], vec![]));
    }
    #[test]
    fn c_fuse_makes_a_resuming_source_stay_exhausted() {
        let mut n = 0;
        let source = std::iter::from_fn(move || {
            n += 1;
            match n {
                1 => Some(1),
                2 => None,
                3 => Some(2),
                _ => None,
            }
        });
        let mut fused = source.fuse();
        assert_eq!(split_two(fused.by_ref()), (vec![1], vec![]));
        assert_eq!(fused.next(), None);
    }
}
