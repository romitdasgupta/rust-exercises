//! LESSON 07 — Guards, alternatives and @ bindings
//! Run: cargo test --test chapter10 l07_
//!
//! PREDICT before running: Does a guard make a pattern exhaustive? Must every side of an or-pattern bind the same names and types?
//! IMPLEMENT the tasks below. Preserve the tests and add an edge case.
//! EXPLAIN afterward: Explain the order of pattern matching and guard evaluation, and why @ can preserve a matched subvalue for later use.
//! COMPILER / TRANSFER LAB: Write an or-pattern with a binding on only one side and inspect the diagnostic. Add a guard to an otherwise exhaustive pattern and remove the fallback to see why guards do not establish exhaustiveness. Compare guard precedence over A | B.
//! Use examples/ch10_scratch.rs; repair intentional failures before committing.

/// A. Use @ and a range pattern: values 10..=20 yield Some(the value),
/// all others None. Do not write an if or call contains.
fn reserved(code: u8) -> Option<u8> {
    todo!("Implement this task")
}
/// B. Accept ('A', n) OR ('B', n) only when n is positive and even.
/// Use one or-pattern with a guard; every other tuple returns false.
fn accepted(packet: (char, i32)) -> bool {
    todo!("Implement this task")
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a_binding_keeps_the_range_checked_value() {
        assert_eq!(reserved(9), None);
        assert_eq!(reserved(10), Some(10));
        assert_eq!(reserved(20), Some(20));
        assert_eq!(reserved(21), None);
    }
    #[test]
    fn b_guard_applies_to_both_alternatives() {
        assert!(accepted(('A', 2)));
        assert!(accepted(('B', 4)));
        assert!(!accepted(('A', -2)));
        assert!(!accepted(('B', 3)));
        assert!(!accepted(('C', 2)));
        assert!(!accepted(('A', 0)));
    }
}
