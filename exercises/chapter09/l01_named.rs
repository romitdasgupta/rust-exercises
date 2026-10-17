//! LESSON 01 — Named structs and update syntax
//! Run: cargo test --test chapter09 l01_
//!
//! PREDICT before running: Does struct update syntax clone a String field? Can you use unmoved fields of the original afterward?
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
}
