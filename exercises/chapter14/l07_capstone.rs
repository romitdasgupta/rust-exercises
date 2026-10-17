//! LESSON 07 — Capstone: configurable text processing
//! Run: cargo test --test chapter14 l07_
//!
//! PREDICT before executing: Decide whether each callback needs Fn, FnMut, or FnOnce.
//! Predict the audit state for duplicate and rejected rows.
//!
//! IMPLEMENT the documented tasks, then keep the tests and add an edge case.
//! EXPLAIN afterward: Explain how the bounds express policy without hard-coding the policy.
//! Which callback must not be invoked for rejected input?
//!
//! COMPILER / MANUAL LAB (use examples/ch14_scratch.rs):
//! Build a stateful predicate that accepts alternating rows, then use an owning formatter.
//! Replace a generic callback with a boxed callable and compare the API tradeoff.
//!
//! TRANSFER: invent a new edge case, predict it, then add a check. A lesson is
//! complete only when its tests, predictions, lab, and explanation are done.

/// A. Apply configurable policies in input order:
/// call keep once per input; on acceptance call format once, then audit once;
/// audit receives the formatted text. Return all formatted accepted rows.
fn process<K, F, A>(rows: &[&str], mut keep: K, mut format: F, mut audit: A) -> Vec<String>
where
    K: FnMut(&str) -> bool,
    F: FnMut(&str) -> String,
    A: FnMut(&str),
{
    todo!("Task 1: implement the documented contract")
}
/// B. Invoke finish exactly once with the number of processed rows.
/// The callback may consume captures, so require only FnOnce.
fn finish_report<F: FnOnce(usize) -> String>(count: usize, finish: F) -> String {
    todo!("Task 2: implement the documented contract")
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a_policies_run_in_documented_order() {
        let mut seen = vec![];
        let mut calls = 0;
        let mut formats = 0;
        let out = process(
            &["a", "", "b", "a"],
            |s| {
                calls += 1;
                !s.is_empty()
            },
            |s| {
                formats += 1;
                s.to_uppercase()
            },
            |s| seen.push(s.to_owned()),
        );
        assert_eq!(out, ["A", "B", "A"]);
        assert_eq!(seen, out);
        assert_eq!(calls, 4);
        assert_eq!(formats, 3);
    }
    #[test]
    fn a_stateful_predicate_and_empty_input_are_supported() {
        let mut n = 0;
        assert_eq!(
            process(
                &["a", "b", "c"],
                |_| {
                    n += 1;
                    n % 2 == 1
                },
                str::to_owned,
                |_| {}
            ),
            ["a", "c"]
        );
        assert!(
            process(
                &[],
                |_| panic!("no input"),
                str::to_owned,
                |_| panic!("no output")
            )
            .is_empty()
        );
    }
    #[test]
    fn b_finish_can_consume_owned_data() {
        let prefix = String::from("count");
        assert_eq!(
            finish_report(3, move |n| {
                let owned = prefix;
                format!("{owned}={n}")
            }),
            "count=3"
        );
    }
}
