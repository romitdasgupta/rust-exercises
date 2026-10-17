//! LESSON 01 — Borrowed, mutable, owned, and draining sources
//! Run: cargo test --test chapter15 l01_
//!
//! PREDICT before executing: What are the Item types of slice.iter(), iter_mut(), and
//! Vec::into_iter()? When are elements removed by drain?
//!
//! IMPLEMENT the documented tasks, then keep the tests and add an edge case.
//! EXPLAIN afterward: Why can reading and editing use different iterator types over the
//! same container? Who owns elements returned by drain?
//!
//! COMPILER / MANUAL LAB (use examples/ch15_scratch.rs):
//! Call into_iter on a Vec<String>, then try to use the vector. Compare .iter(). For drain,
//! create a drain over 1..3, consume one item, then drop it; inspect the remaining vector.
//!
//! TRANSFER: invent a new edge case, predict it, then add a check. A lesson is
//! complete only when its tests, predictions, lab, and explanation are done.

/// A. Sum borrowed i32 values into i64 without consuming the source.
fn borrowed_sum(values: &[i32]) -> i64 {
    todo!("Task 1: implement the documented contract")
}
/// B. Negate each value through iter_mut. Inputs exclude i32::MIN.
fn negate(values: &mut [i32]) {
    todo!("Task 2: implement the documented contract")
}
/// C. Drain valid start..end from an owned vector; return (removed,remaining).
fn drain_range(mut values: Vec<String>, start: usize, end: usize) -> (Vec<String>, Vec<String>) {
    todo!("Task 3: implement the documented contract")
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a_borrowed_source_widens_before_sum() {
        let v = vec![i32::MAX, i32::MAX, -1];
        assert_eq!(borrowed_sum(&v), 4_294_967_293);
        assert_eq!(v.len(), 3);
        assert_eq!(borrowed_sum(&[]), 0);
    }
    #[test]
    fn b_mutable_source_edits_elements() {
        let mut v = [-2, 0, 7];
        negate(&mut v);
        assert_eq!(v, [2, 0, -7]);
    }
    #[test]
    fn c_drain_transfers_only_selected_elements() {
        assert_eq!(
            drain_range(vec!["a".into(), "b".into(), "c".into(), "d".into()], 1, 3),
            (vec!["b".into(), "c".into()], vec!["a".into(), "d".into()])
        );
        assert_eq!(drain_range(vec![], 0, 0), (vec![], vec![]));
    }
}
