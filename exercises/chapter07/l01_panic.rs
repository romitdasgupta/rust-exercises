//! LESSON 01 — Panics and invariants
//! Run: cargo test --test chapter07 l01_
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider: Do local destructors run on unwinding? Do they run after panic=abort? Can catch_unwind catch abort?
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
    use practice::guess;
    use std::cell::RefCell;
    use std::panic::{AssertUnwindSafe, catch_unwind};
    #[test]
    fn a_positive_capacity_is_accepted() {
        assert_eq!(require_capacity(1), 1);
        assert_eq!(require_capacity(99), 99);
        assert_eq!(require_capacity(usize::MAX), usize::MAX);
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
        assert_eq!(label(""), Err("empty label"));
        assert_eq!(label("\u{3000}"), Err("empty label"));
    }
    #[test]
    fn b_label_borrows_from_the_input() {
        let text = String::from("  route  ");
        let out = label(&text).unwrap();
        assert_eq!(out.as_ptr() as usize, text.as_ptr() as usize + 2);
    }

    struct Noisy<'a>(&'static str, &'a RefCell<Vec<&'static str>>);
    impl Drop for Noisy<'_> {
        fn drop(&mut self) {
            self.1.borrow_mut().push(self.0);
        }
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    /// The panics below are caught; their messages in the test output are expected.
    #[test]
    fn p_predictions() {
        // P1. Do destructors run while a panic unwinds? In what order?
        let log = RefCell::new(Vec::new());
        let result = catch_unwind(AssertUnwindSafe(|| {
            let _first = Noisy("first", &log);
            let _second = Noisy("second", &log);
            panic!("boom");
        }));
        assert!(result.is_err());
        let p1: Vec<&str> = guess();
        assert_eq!(p1, *log.borrow(), "P1: unwinding drops locals");

        // P2. Is each panic payload a String? (A literal message vs a formatted one.)
        let n = 7;
        let literal = catch_unwind(|| panic!("boom")).unwrap_err();
        let formatted = catch_unwind(|| panic!("code {n}")).unwrap_err();
        let p2: (bool, bool) = guess();
        assert_eq!(
            p2,
            (literal.is::<String>(), formatted.is::<String>()),
            "P2: panic payload types"
        );
    }
}
