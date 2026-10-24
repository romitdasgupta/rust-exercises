//! LESSON 07 — Guards, alternatives and @ bindings
//! Run: cargo test --test chapter10 l07_
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider: Does a guard make a pattern exhaustive? Must every side of an or-pattern bind the same names and types?
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
    use practice::guess;
    use std::cell::Cell;
    #[test]
    fn a_binding_keeps_the_range_checked_value() {
        assert_eq!(reserved(9), None);
        assert_eq!(reserved(10), Some(10));
        assert_eq!(reserved(20), Some(20));
        assert_eq!(reserved(21), None);
        assert_eq!(reserved(15), Some(15));
        assert_eq!(reserved(0), None);
        assert_eq!(reserved(u8::MAX), None);
    }
    #[test]
    fn b_guard_applies_to_both_alternatives() {
        assert!(accepted(('A', 2)));
        assert!(accepted(('B', 4)));
        assert!(!accepted(('A', -2)));
        assert!(!accepted(('B', 3)));
        assert!(!accepted(('C', 2)));
        assert!(!accepted(('A', 0)));
        assert!(!accepted(('B', 0)));
        assert!(!accepted(('B', -4)));
        assert!(!accepted(('b', 2)), "letters are case-sensitive");
        assert!(!accepted(('A', i32::MAX)));
        assert!(!accepted(('B', i32::MIN)));
        assert!(accepted(('B', i32::MAX - 1)));
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    #[test]
    fn p_predictions() {
        // P1. Does the guard apply to `4` only, or to `1 | 4`?
        let x = 1;
        let p1: &str = guess();
        let actual = match x {
            1 | 4 if x > 3 => "first arm",
            _ => "fallback",
        };
        assert_eq!(p1, actual, "P1: guard precedence");

        // P2. Both alternatives match (1, 1). How many times does the guard run?
        let runs = Cell::new(0);
        match (1, 1) {
            (1, _) | (_, 1)
                if {
                    runs.set(runs.get() + 1);
                    false
                } => {}
            _ => {}
        }
        let p2: i32 = guess();
        assert_eq!(p2, runs.get(), "P2: guards and or-patterns");
    }
}
