//! LESSON 02 — Tuple structs, unit structs and layout
//! Run: cargo test --test chapter09 l02_
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider: Are two tuple structs with the same fields interchangeable? Does default repr guarantee field order or a fixed size?
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
    use practice::{alloc::count, guess};
    #[test]
    fn a_newtype_keeps_units_explicit() {
        assert_eq!(centimeters(Meters(3)), Centimeters(300));
        assert_eq!(centimeters(Meters(u32::MAX)), Centimeters(429496729500));
        assert_eq!(centimeters(Meters(0)), Centimeters(0));
    }
    #[test]
    fn b_unit_marker_represents_a_proof_step() {
        assert_eq!(validate(" x "), Some(Validated));
        assert_eq!(validate("\n"), None);
        assert_eq!(validate(""), None);
        assert_eq!(validate("\u{3000}\t"), None, "U+3000 is Unicode whitespace");
        assert_eq!(validate("\u{3000}港"), Some(Validated));
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    #[test]
    fn p_predictions() {
        // P1. Sizes in bytes of the unit struct and of the newtype over u32.
        let p1: (usize, usize) = guess();
        assert_eq!(
            p1,
            (size_of::<Validated>(), size_of::<Meters>()),
            "P1: unit structs and newtypes"
        );

        // P2. How many heap allocations does a Vec of 1000 unit values make?
        let (markers, allocations) = count(|| (0..1000).map(|_| Validated).collect::<Vec<_>>());
        let p2: usize = guess();
        assert_eq!(p2, allocations, "P2: zero-sized elements");
        assert_eq!(markers.len(), 1000);

        // P3. repr(C) keeps declaration order. Size and alignment of this struct?
        #[repr(C)]
        struct Header {
            tag: u8,
            length: u32,
            flags: u8,
        }
        let p3: (usize, usize) = guess();
        assert_eq!(
            p3,
            (size_of::<Header>(), align_of::<Header>()),
            "P3: padding under repr(C)"
        );
    }
}
