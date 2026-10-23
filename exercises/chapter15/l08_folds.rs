//! LESSON 08 — Folds, short circuits, and side effects
//! Run: cargo test --test chapter15 l08_
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider: Is fold order observable for subtraction or
//! string construction? Does try_fold consume the first item that fails?
//!
//! IMPLEMENT the documented tasks, then keep the tests and add an edge case.
//! EXPLAIN afterward: Why should a fallible consumer stop immediately? How do rfold and
//! try_rfold change processing direction?
//!
//! COMPILER / MANUAL LAB (use examples/ch15_scratch.rs):
//! Fold [1,2,3] with subtraction from zero in both directions and then build nested
//! parenthesized text to expose order. Use try_rfold with an error near the right edge and
//! inspect which values ran.
//!
//! TRANSFER: invent a new edge case, predict it, then add a check. A lesson is
//! complete only when its tests, predictions, lab, and explanation are done.

/// A. Build exact left-associated nesting from seed "x": [1,2] -> "((x+1)+2)".
fn nesting(values: &[i32]) -> String {
    todo!("Task 1: implement the documented contract")
}
/// B. Checked sum in u8 using try_fold; empty sum is Some(0).
fn checked_sum(values: &[u8]) -> Option<u8> {
    todo!("Task 2: implement the documented contract")
}
/// C. Use try_for_each to record values until and including first negative.
/// Return (seen,Ok(())) if none, otherwise (seen,Err("negative")).
fn audit(values: &[i32]) -> (Vec<i32>, Result<(), &'static str>) {
    todo!("Task 3: implement the documented contract")
}
#[cfg(test)]
mod tests {
    use super::*;
    use practice::guess;
    #[test]
    fn a_fold_carries_the_previous_result() {
        assert_eq!(nesting(&[1, -2]), "((x+1)+-2)");
        assert_eq!(nesting(&[]), "x");
        assert_eq!(nesting(&[7]), "(x+7)");
    }
    #[test]
    fn b_try_fold_checks_without_wrapping() {
        assert_eq!(checked_sum(&[]), Some(0));
        assert_eq!(checked_sum(&[250, 5]), Some(255));
        assert_eq!(checked_sum(&[250, 6, 0]), None);
    }
    #[test]
    fn c_try_for_each_stops_immediately() {
        assert_eq!(audit(&[1, -2, 3]), (vec![1, -2], Err("negative")));
        assert_eq!(audit(&[0, 2]), (vec![0, 2], Ok(())));
        assert_eq!(audit(&[-1, -2]), (vec![-1], Err("negative")));
        assert_eq!(audit(&[]), (vec![], Ok(())));
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    /// A wrong prediction is useful: read the failure, then explain it in NOTES.md.
    #[test]
    fn p_predictions() {
        // P1. Building digits with fold and with rfold: (fold, rfold).
        let digits = [1, 2, 3];
        let left = digits.iter().fold(0, |acc, d| acc * 10 + d);
        let right = digits.iter().rfold(0, |acc, d| acc * 10 + d);
        let p1: (i32, i32) = guess();
        assert_eq!(p1, (left, right), "P1: fold direction");

        // P2. Checked u8 sum of [100, 100, 100, 1]: (result, closure calls).
        let mut calls = 0;
        let sum = [100u8, 100, 100, 1].iter().try_fold(0u8, |acc, &x| {
            calls += 1;
            acc.checked_add(x)
        });
        let p2: (Option<u8>, i32) = guess();
        assert_eq!(p2, (sum, calls), "P2: try_fold stops at the first None");

        // P3. After that failure, how many items are left in the iterator?
        let mut it = [1u8, 255, 3, 4].iter();
        let sum = it.try_fold(0u8, |acc, &x| acc.checked_add(x));
        let p3: (Option<u8>, usize) = guess();
        assert_eq!(
            p3,
            (sum, it.len()),
            "P3: the failing item is consumed, the rest remain"
        );
    }
}

/// D. try_fold an iterator with checked u8 sum. Return (result,unconsumed tail).
/// The item that causes overflow is consumed; later items must remain available.
fn sum_and_remainder(values: &[u8]) -> (Option<u8>, Vec<u8>) {
    todo!("Task 4: implement the documented contract")
}
#[cfg(test)]
mod remainder_tests {
    use super::*;
    #[test]
    fn d_try_fold_preserves_the_tail_after_failure() {
        assert_eq!(sum_and_remainder(&[250, 6, 7, 8]), (None, vec![7, 8]));
        assert_eq!(sum_and_remainder(&[1, 2]), (Some(3), vec![]));
        assert_eq!(sum_and_remainder(&[]), (Some(0), vec![]));
    }
}
