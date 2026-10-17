//! LESSON 01 — Panics and invariants
//! Run: cargo test --test chapter07 l01_
//!
//! PREDICT before running: Do local destructors run on unwinding? Do they run after panic=abort? Can catch_unwind catch abort?
//! IMPLEMENT the tasks below. Preserve the tests and add an edge case.
//! EXPLAIN afterward: Explain why panic is appropriate for the stated invariant, and why ordinary missing input should use Result instead.
//! COMPILER / TRANSFER LAB: In a disposable package, observe a Drop logger before a panic, then rerun with panic=abort in the profile and compare. In separate program runs install set_hook/take_hook; log payload/location and restore the hook. Try assert!, assert_eq!, debug_assert!, unreachable!, todo!, and unimplemented!; compare release behavior. Never change hooks or abort settings inside parallel chapter tests.
//! Use examples/ch07_scratch.rs; repair intentional failures before committing.

/// A. Require a strictly positive capacity, panicking with EXACT message
/// "capacity must be positive" when zero. Return the input otherwise.
fn require_capacity(capacity: usize) -> usize {
    todo!("Implement this task")
}
/// B. Expected invalid input should return Err("empty label"). Trim text and
/// return the borrowed label on success; never panic for an empty label.
fn label(text: &str) -> Result<&str, &'static str> {
    todo!("Implement this task")
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a_positive_capacity_is_accepted() {
        assert_eq!(require_capacity(1), 1);
        assert_eq!(require_capacity(99), 99);
    }
    #[test]
    #[should_panic(expected = "capacity must be positive")]
    fn a_zero_violates_the_invariant() {
        require_capacity(0);
    }
    #[test]
    fn b_empty_label_is_a_recoverable_error() {
        assert_eq!(label(" 港 "), Ok("港"));
        assert_eq!(label(" \n"), Err("empty label"));
    }
}
