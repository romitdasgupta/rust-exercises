//! LESSON 03 — Partial and total comparisons
//! Run: cargo test --test chapter12 l03_
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider: If cmp returns Equal, what must equality
//! report? What does partial_cmp return for NaN?
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
    use practice::guess;
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

    #[test]
    fn a_priority_wins_over_name() {
        let low = Job {
            priority: 1,
            name: "zzz".into(),
        };
        let high = Job {
            priority: 9,
            name: "a".into(),
        };
        assert_eq!(low.cmp(&high), Ordering::Less);
        assert_eq!(high.cmp(&low), Ordering::Greater);
        assert!(low < high, "the < operator goes through PartialOrd");
    }

    #[test]
    fn c_float_order_handles_nan_on_either_side() {
        assert_eq!(compare_readings(1.0, f64::NAN), None);
        assert_eq!(compare_readings(f64::NAN, f64::NAN), None);
        assert_eq!(compare_readings(2.0, -1.0), Some(Ordering::Greater));
        assert_eq!(
            compare_readings(f64::NEG_INFINITY, f64::MIN),
            Some(Ordering::Less)
        );
    }

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
    enum Level {
        High,
        Low,
        Medium,
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    #[test]
    fn p_predictions() {
        // P1. Partial vs total comparison of the two zeros.
        let p1: (Option<Ordering>, Ordering) = guess();
        assert_eq!(
            p1,
            ((-0.0_f64).partial_cmp(&0.0), (-0.0_f64).total_cmp(&0.0)),
            "P1: partial_cmp vs total_cmp"
        );

        // P2. Derived Ord on an enum, after sorting.
        let mut levels = [Level::Low, Level::High, Level::Medium];
        levels.sort();
        let p2: [Level; 3] = guess();
        assert_eq!(p2, levels, "P2: derived enum order");

        // P3. str comparison.
        let p3: (bool, bool, bool) = guess();
        assert_eq!(
            p3,
            ("Zebra" < "apple", "10" < "9", "apple" < "Apple"),
            "P3: strings compare bytes"
        );

        // P4. Ties: which element do max_by_key and min_by_key return?
        let pairs = [(1, 'a'), (1, 'b')];
        let p4: (char, char) = guess();
        assert_eq!(
            p4,
            (
                pairs.iter().max_by_key(|p| p.0).unwrap().1,
                pairs.iter().min_by_key(|p| p.0).unwrap().1
            ),
            "P4: tie-breaking in max and min"
        );
    }
}
