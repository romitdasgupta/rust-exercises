//! LESSON 02 — Equivalence and NaN
//! Run: cargo test --test chapter12 l02_
//!
//! PREDICT before executing: Is equality reflexive for f64? Should a metadata field affect
//! the identity of a record?
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
}
