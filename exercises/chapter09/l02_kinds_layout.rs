//! LESSON 02 — Tuple structs, unit structs and layout
//! Run: cargo test --test chapter09 l02_
//!
//! PREDICT before running: Are two tuple structs with the same fields interchangeable? Does default repr guarantee field order or a fixed size?
//! IMPLEMENT the tasks below. Preserve the tests and add an edge case.
//! EXPLAIN afterward: Explain nominal types, zero-sized marker values, alignment and padding, and why Rust layout should not be assumed from declaration order.
//! COMPILER / TRANSFER LAB: Define tuple structs UserId(u64) and OrderId(u64), then pass the wrong one to a function. Inspect size_of/align_of on field permutations and a unit struct; record observations separately from guarantees. Add repr(C) in scratch and explain why it is a boundary contract, not an optimization switch. Do not test fixed offsets of repr(Rust).
//! Use examples/ch09_scratch.rs; repair intentional failures before committing.

#[derive(Debug, PartialEq)]
struct Meters(u32);
#[derive(Debug, PartialEq)]
struct Centimeters(u64);
#[derive(Debug, PartialEq)]
struct Validated;
/// A. Convert Meters to a distinct Centimeters type; widen before multiplication.
fn centimeters(distance: Meters) -> Centimeters {
    todo!("Implement this task")
}
/// B. Return the unit marker only if the input is nonempty after trimming.
fn validate(text: &str) -> Option<Validated> {
    todo!("Implement this task")
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a_newtype_keeps_units_explicit() {
        assert_eq!(centimeters(Meters(3)), Centimeters(300));
        assert_eq!(centimeters(Meters(u32::MAX)), Centimeters(429496729500));
    }
    #[test]
    fn b_unit_marker_represents_a_proof_step() {
        assert_eq!(validate(" x "), Some(Validated));
        assert_eq!(validate("\n"), None);
    }
}
