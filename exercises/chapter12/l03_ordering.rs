//! LESSON 03 — Partial and total comparisons
//! Run: cargo test --test chapter12 l03_
//!
//! PREDICT before executing: If cmp returns Equal, what must equality report? What does
//! partial_cmp return for NaN?
//!
//! IMPLEMENT the documented tasks, then keep the tests and add an edge case.
//! EXPLAIN afterward: Why is lexicographic tie-breaking part of the public contract? Why
//! must PartialOrd delegate to Ord for a totally ordered type?
//!
//! COMPILER / MANUAL LAB (use examples/ch12_scratch.rs):
//! Create a float wrapper using f64::total_cmp and bitwise equality in scratch. Test -0.0,
//! +0.0, and two NaN bit patterns; explain how that order differs from native float
//! equality.
//!
//! TRANSFER: invent a new edge case, predict it, then add a check. A lesson is
//! complete only when its tests, predictions, lab, and explanation are done.

use std::cmp::Ordering;
#[derive(Debug, PartialEq, Eq)]
struct Job {
    priority: u8,
    name: String,
}
impl Ord for Job {
    /// A. Ascending priority, then ascending name; equality uses both fields.
    fn cmp(&self, other: &Self) -> Ordering {
        todo!("Task 1: implement the documented contract")
    }
}
impl PartialOrd for Job {
    /// B. This total order is always comparable and agrees with Ord.
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        todo!("Task 2: implement the documented contract")
    }
}
/// C. Compare floats without inventing an ordering for NaN.
fn compare_readings(a: f64, b: f64) -> Option<Ordering> {
    todo!("Task 3: implement the documented contract")
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a_total_order_respects_primary_and_secondary_keys() {
        let mut jobs = vec![
            Job {
                priority: 2,
                name: "a".into(),
            },
            Job {
                priority: 1,
                name: "z".into(),
            },
            Job {
                priority: 1,
                name: "a".into(),
            },
        ];
        jobs.sort();
        assert_eq!(
            jobs.iter()
                .map(|j| (j.priority, j.name.as_str()))
                .collect::<Vec<_>>(),
            [(1, "a"), (1, "z"), (2, "a")]
        );
    }
    #[test]
    fn b_partial_order_agrees_with_equality_and_total_order() {
        let a = Job {
            priority: 1,
            name: "a".into(),
        };
        let b = Job {
            priority: 2,
            name: "a".into(),
        };
        assert_eq!(a.partial_cmp(&b), Some(a.cmp(&b)));
        assert_eq!(a.partial_cmp(&a), Some(Ordering::Equal));
    }
    #[test]
    fn c_float_order_is_partial() {
        assert_eq!(compare_readings(f64::NAN, 1.0), None);
        assert_eq!(compare_readings(-0.0, 0.0), Some(Ordering::Equal));
        assert_eq!(compare_readings(-1.0, 2.0), Some(Ordering::Less));
    }
}
