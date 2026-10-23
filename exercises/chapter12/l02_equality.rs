//! LESSON 02 — Equivalence and NaN
//! Run: cargo test --test chapter12 l02_
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider: Is equality reflexive for f64? Should a
//! metadata field affect the identity of a record?
//!
//! IMPLEMENT the documented tasks, then keep the tests and add an edge case.
//! EXPLAIN afterward: List reflexivity, symmetry, and transitivity. Why must Eq not be
//! added to a wrapper whose equality follows NaN behavior?
//!
//! COMPILER / MANUAL LAB (use examples/ch12_scratch.rs):
//! Attempt deriving Eq for a struct holding f64. Then create a case-insensitive ASCII name
//! newtype; explain why a future Hash implementation must use the same normalized identity.
//!
//! TRANSFER: invent a new edge case, predict it, then add a check. A lesson is
//! complete only when its tests, predictions, lab, and explanation are done.

#[derive(Debug)]
struct Record {
    id: u64,
    note: String,
}
impl PartialEq for Record {
    /// A. Records are equal exactly when ids match; ignore note.
    fn eq(&self, other: &Self) -> bool {
        todo!("Task 1: implement the documented contract")
    }
}
impl Eq for Record {}
#[derive(Debug)]
struct Reading(f64);
impl PartialEq for Reading {
    /// B. Preserve f64 partial equality, including NaN != NaN.
    fn eq(&self, other: &Self) -> bool {
        todo!("Task 2: implement the documented contract")
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use practice::guess;
    #[test]
    fn a_record_identity_ignores_metadata() {
        let a = Record {
            id: 1,
            note: "old".into(),
        };
        let b = Record {
            id: 1,
            note: "new".into(),
        };
        assert_eq!(a, b);
        assert_ne!(
            a,
            Record {
                id: 2,
                note: "old".into()
            }
        );
        assert_eq!(a, a);
    }
    #[test]
    fn b_float_wrapper_keeps_nan_and_signed_zero_semantics() {
        assert_ne!(Reading(f64::NAN), Reading(f64::NAN));
        assert_eq!(Reading(0.0), Reading(-0.0));
        assert_eq!(Reading(4.0), Reading(4.0));
        assert_ne!(Reading(4.0), Reading(5.0));
    }

    #[derive(PartialEq)]
    struct Point {
        x: f64,
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    #[test]
    fn p_predictions() {
        // P1. NaN equality, directly and inside a collection.
        let (nan, also_nan) = (f64::NAN, f64::NAN);
        let p1: (bool, bool) = guess();
        assert_eq!(
            p1,
            (nan == also_nan, [1.0, nan].contains(&also_nan)),
            "P1: NaN is not equal to itself"
        );

        // P2. Derived PartialEq compares fields with ==. Are the zeros equal? Are their
        // bit patterns equal?
        let p2: (bool, bool) = guess();
        assert_eq!(
            p2,
            (
                Point { x: 0.0 } == Point { x: -0.0 },
                0.0_f64.to_bits() == (-0.0_f64).to_bits()
            ),
            "P2: == is not bitwise"
        );

        // P3. f64::max and f64::min with one NaN operand.
        let p3: (f64, f64) = guess();
        assert_eq!(
            p3,
            (f64::NAN.max(1.0), f64::NAN.min(1.0)),
            "P3: max/min ignore NaN"
        );
    }
}
