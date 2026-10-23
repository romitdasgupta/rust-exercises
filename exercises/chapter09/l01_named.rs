//! LESSON 01 — Named structs and update syntax
//! Run: cargo test --test chapter09 l01_
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider: Does struct update syntax clone a String field? Can you use unmoved fields of the original afterward?
//! IMPLEMENT the tasks below. Preserve the tests and add an edge case.
//! EXPLAIN afterward: Explain construction by field name, field shorthand, update syntax moves, and partial initialization.
//! COMPILER / TRANSFER LAB: Construct a two-String struct, make a second value with one replacement field and ..first, then inspect which original fields remain usable. Compare destructuring with field access.
//! Use examples/ch09_scratch.rs; repair intentional failures before committing.

#[derive(Debug, PartialEq)]
struct Parcel {
    label: String,
    zone: String,
    grams: u32,
}
/// A. Construct Parcel using field shorthand where possible.
fn parcel(label: String, zone: String, grams: u32) -> Parcel {
    todo!("Implement this task")
}
/// B. Consume a parcel and replace only its zone, using struct update syntax.
/// Do not clone its label. Keep grams unchanged.
fn reroute(old: Parcel, zone: String) -> Parcel {
    todo!("Implement this task")
}
#[cfg(test)]
mod tests {
    use super::*;
    use practice::{alloc::count, guess};
    use std::cell::RefCell;
    #[test]
    fn a_fields_have_named_meanings() {
        assert_eq!(
            parcel("x".into(), "west".into(), 30),
            Parcel {
                label: "x".into(),
                zone: "west".into(),
                grams: 30
            }
        );
    }
    #[test]
    fn b_update_moves_unspecified_fields() {
        let label = String::from("fragile");
        let p = label.as_ptr();
        let new = reroute(parcel(label, "west".into(), 99), "east".into());
        assert_eq!(new.label.as_ptr(), p);
        assert_eq!(new.zone, "east");
        assert_eq!(new.grams, 99);
    }
    #[test]
    fn b_update_syntax_does_not_allocate() {
        let old = parcel("fragile".into(), "west".into(), 99);
        let zone = String::from("east");
        let (new, allocations) = count(|| reroute(old, zone));
        assert_eq!(new.label, "fragile");
        assert_eq!(
            allocations, 0,
            "reroute should move the label, not clone it"
        );
    }

    struct Noisy<'a>(&'static str, &'a RefCell<Vec<&'static str>>);
    impl Drop for Noisy<'_> {
        fn drop(&mut self) {
            self.1.borrow_mut().push(self.0);
        }
    }
    struct Pair<'a> {
        first: Noisy<'a>,
        second: Noisy<'a>,
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    /// A wrong prediction is useful: read the failure, then explain it in NOTES.md.
    #[test]
    fn p_predictions() {
        // P1. The fields are written in the opposite order to their declaration.
        // In what order are they dropped?
        let log = RefCell::new(Vec::new());
        {
            let _pair = Pair {
                second: Noisy("second", &log),
                first: Noisy("first", &log),
            };
        }
        let p1: Vec<&str> = guess();
        assert_eq!(p1, *log.borrow(), "P1: field drop order");

        // P2. `..old` moves old.second into `new`; old.first stays in `old`.
        // What does the drop log contain when the block ends?
        let log = RefCell::new(Vec::new());
        {
            let old = Pair {
                first: Noisy("old first", &log),
                second: Noisy("old second", &log),
            };
            let new = Pair {
                first: Noisy("new first", &log),
                ..old
            };
        }
        let p2: Vec<&str> = guess();
        assert_eq!(p2, *log.borrow(), "P2: update syntax moves fields");
    }
}
