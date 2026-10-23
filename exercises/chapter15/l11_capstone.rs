//! LESSON 11 — Capstone: a lazy measurement report
//! Run: cargo test --test chapter15 l11_
//!
//! PREDICT before running: Predict how many rows are parsed when limit=0, and when the
//! first matching row arrives late. Which adapters skip work?
//!
//! IMPLEMENT the documented tasks, then keep the tests and add an edge case.
//! EXPLAIN afterward: Why must enumerate precede trimming/filtering to preserve original
//! source numbering? Show where each consumer starts execution.
//!
//! COMPILER / MANUAL LAB (use examples/ch15_scratch.rs):
//! Instrument parsing with inspect or a Cell and prove take stops processing later rows.
//! Then replace collect with a checked try_fold and explain the changed error policy.
//!
//! TRANSFER: invent a new edge case, predict it, then add a check. A lesson is
//! complete only when its tests, predictions, lab, and explanation are done.

#[derive(Debug, PartialEq)]
struct Report {
    rows: Vec<(usize, i32)>,
    sum: i64,
    mean: Option<f64>,
}
/// A. Ignore blank/comment lines (trimmed text starts '#') and invalid i32.
/// Keep values >= minimum, retain original 1-based line numbers, stop after limit.
/// Build one lazy pipeline: no intermediate collections, no work past the limit.
fn selected_rows(lines: &[&str], minimum: i32, limit: usize) -> Vec<(usize, i32)> {
    todo!("Task 1: implement the documented contract")
}
/// B. Assemble a report from selected rows; preserve order. Empty mean is None.
fn report(lines: &[&str], minimum: i32, limit: usize) -> Report {
    todo!("Task 2: implement the documented contract")
}
#[cfg(test)]
mod tests {
    use super::*;
    use practice::alloc::count;
    #[test]
    fn a_pipeline_keeps_source_line_numbers_and_limit() {
        let lines = [" # comment", "3", "oops", " 9 ", "-2", "9", ""];
        assert_eq!(selected_rows(&lines, 3, 2), [(2, 3), (4, 9)]);
        assert_eq!(selected_rows(&lines, 3, 9), [(2, 3), (4, 9), (6, 9)]);
        assert!(selected_rows(&lines, 0, 0).is_empty());
        assert_eq!(selected_rows(&["#5", "  ", "+7"], 7, 9), [(3, 7)]);
    }
    #[test]
    fn a_pipeline_is_lazy_and_collects_once() {
        let lines: Vec<String> = (0..1000).map(|i| i.to_string()).collect();
        let lines: Vec<&str> = lines.iter().map(String::as_str).collect();
        let (rows, allocations) = count(|| selected_rows(&lines, 10, 3));
        assert_eq!(rows, [(11, 10), (12, 11), (13, 12)]);
        assert!(
            allocations <= 1,
            "only the output vector should allocate; got {allocations}"
        );
    }
    #[test]
    fn b_report_combines_owned_collection_and_consumers() {
        assert_eq!(
            report(&["3", "bad", "5"], 0, 10),
            Report {
                rows: vec![(1, 3), (3, 5)],
                sum: 8,
                mean: Some(4.0)
            }
        );
        assert_eq!(
            report(&[], 0, 10),
            Report {
                rows: vec![],
                sum: 0,
                mean: None
            }
        );
        let r = report(&["2147483647", "2147483647"], 0, 10);
        assert_eq!(r.sum, 4_294_967_294);
        assert_eq!(r.mean, Some(2_147_483_647.0));
        assert_eq!(report(&["1", "2", "#9"], 0, 10).mean, Some(1.5));
        assert_eq!(report(&["1", "2", "3"], 0, 2).sum, 3);
    }
}
