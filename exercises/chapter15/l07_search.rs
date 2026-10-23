//! LESSON 07 — Searching and iterator positions
//! Run: cargo test --test chapter15 l07_
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider: Does nth(2) count from the beginning or from
//! the current iterator state? What does rposition require beyond Iterator?
//!
//! IMPLEMENT the documented tasks, then keep the tests and add an edge case.
//! EXPLAIN afterward: Explain consuming search versus indexing. Why must a reverse position
//! be reported in original index coordinates?
//!
//! COMPILER / MANUAL LAB (use examples/ch15_scratch.rs):
//! On 0..10 call nth(2), next(), nth_back(1), last(); predict each value and remaining len
//! before every call. Test find versus find_map with parse failures.
//!
//! TRANSFER: invent a new edge case, predict it, then add a check. A lesson is
//! complete only when its tests, predictions, lab, and explanation are done.

/// A. Return first and last original indices of target.
fn positions(values: &[i32], target: i32) -> (Option<usize>, Option<usize>) {
    todo!("Task 1: implement the documented contract")
}
/// B. Find the first parseable i32 after trimming; ignore invalid tokens.
fn first_number(tokens: &[&str]) -> Option<i32> {
    todo!("Task 2: implement the documented contract")
}
/// C. Fresh double-ended iterator: return third-from-front, then second-from-back,
/// then all remaining values in forward order. Missing selections return None.
fn selected(values: &[i32]) -> (Option<i32>, Option<i32>, Vec<i32>) {
    todo!("Task 3: implement the documented contract")
}
/// D. Find the last negative value using rfind.
fn last_negative(values: &[i32]) -> Option<i32> {
    todo!("Task 4: implement the documented contract")
}
#[cfg(test)]
mod tests {
    use super::*;
    use practice::guess;
    #[test]
    fn a_positions_are_original_indices() {
        assert_eq!(positions(&[2, 7, 2, 2], 2), (Some(0), Some(3)));
        assert_eq!(positions(&[], 2), (None, None));
        assert_eq!(positions(&[5], 5), (Some(0), Some(0)));
    }
    #[test]
    fn b_find_map_skips_errors_and_stops_on_zero() {
        assert_eq!(first_number(&["x", " 0 ", "12"]), Some(0));
        assert_eq!(first_number(&["x", ""]), None);
        assert_eq!(first_number(&["1.5", "-7", "8"]), Some(-7));
    }
    #[test]
    fn c_nth_consumes_skipped_values_from_each_end() {
        assert_eq!(
            selected(&[0, 1, 2, 3, 4, 5, 6]),
            (Some(2), Some(5), vec![3, 4])
        );
        assert_eq!(selected(&[1]), (None, None, vec![]));
        assert_eq!(selected(&[0, 1, 2, 3]), (Some(2), None, vec![]));
        assert_eq!(selected(&[0, 1, 2, 3, 4]), (Some(2), Some(3), vec![]));
    }
    #[test]
    fn d_reverse_search_returns_a_value() {
        assert_eq!(last_negative(&[-2, 1, -5, 0]), Some(-5));
        assert_eq!(last_negative(&[0, 1]), None);
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    /// A wrong prediction is useful: read the failure, then explain it in NOTES.md.
    #[test]
    fn p_predictions() {
        // P1. Two nth(2) calls on 0..10, then the remaining length.
        let mut it = 0..10;
        let calls = (it.nth(2), it.nth(2), it.len());
        let p1: (Option<i32>, Option<i32>, usize) = guess();
        assert_eq!(p1, calls, "P1: nth counts from the current position");

        // P2. (position of 8 after one next(), rposition of 8 on a fresh iterator)
        let values = [7, 8, 9, 8];
        let mut it = values.iter();
        it.next();
        let after_next = it.position(|&x| x == 8);
        let from_back = values.iter().rposition(|&x| x == 8);
        let p2: (Option<usize>, Option<usize>) = guess();
        assert_eq!(
            p2,
            (after_next, from_back),
            "P2: which index each method reports"
        );

        // P3. find on an endless range, then next().
        let mut naturals = 1..;
        let first = naturals.find(|n| n % 5 == 0);
        let p3: (Option<i32>, Option<i32>) = guess();
        assert_eq!(
            p3,
            (first, naturals.next()),
            "P3: find consumes the match too"
        );
    }
}
