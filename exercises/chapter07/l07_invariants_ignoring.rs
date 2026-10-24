//! LESSON 07 — Impossible errors and deliberate recovery
//! Run: cargo test --test chapter07 l07_
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider: When is expect justified? Does assigning a Result to _ mean success? Why does unused Result warn?
//! IMPLEMENT the tasks below. Preserve the tests and add an edge case.
//! EXPLAIN afterward: Explain the documented proof behind expect and the information intentionally discarded by a recovery policy.
//! COMPILER / TRANSFER LAB: Call a fallible function as a bare statement, observe must_use, then use let _ = and explain the changed diagnostic but unchanged failure. Compare ignoring an error with logging it and with propagating it.
//! Use examples/ch07_scratch.rs; repair intentional failures before committing.

/// A. Parse the fixed literal "64" as u16 and use expect with a message that
/// documents why failure would be a programming error. Do not accept input.
fn built_in_limit() -> u16 {
    todo!("Implement this task")
}
/// B. Parse every trimmed i16 input; return (successful values, error count).
/// This API deliberately recovers from bad rows instead of stopping early.
fn recover_rows(rows: &[&str]) -> (Vec<i16>, usize) {
    todo!("Implement this task")
}
#[cfg(test)]
mod tests {
    use super::*;
    use practice::guess;
    use std::cell::RefCell;
    #[test]
    fn a_literal_has_an_explainable_invariant() {
        assert_eq!(built_in_limit(), 64);
    }
    #[test]
    fn b_recovery_reports_information_loss() {
        assert_eq!(
            recover_rows(&["1", "bad", " -2 ", "40000"]),
            (vec![1, -2], 2)
        );
        assert_eq!(recover_rows(&[]), (vec![], 0));
        assert_eq!(recover_rows(&["bad"]), (vec![], 1));
        assert_eq!(
            recover_rows(&["3", "", "1", "-32768"]),
            (vec![3, 1, -32768], 1)
        );
    }

    struct Noisy<'a>(&'static str, &'a RefCell<Vec<&'static str>>);
    impl Drop for Noisy<'_> {
        fn drop(&mut self) {
            self.1.borrow_mut().push(self.0);
        }
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    #[test]
    fn p_predictions() {
        // P1. `let _ =` does not bind; `let _b =` does. When does each value drop?
        let log = RefCell::new(Vec::new());
        {
            let _ = Noisy("a", &log);
            log.borrow_mut().push("mid");
            let _b = Noisy("b", &log);
            log.borrow_mut().push("end");
        }
        let p1: Vec<&str> = guess();
        assert_eq!(p1, *log.borrow(), "P1: _ discards immediately");

        // P2. ok() silently discards every error, including overflow.
        let rows = ["5", "300", "-2", "x"];
        let sum: i8 = rows.iter().filter_map(|r| r.parse::<i8>().ok()).sum();
        let p2: i8 = guess();
        assert_eq!(p2, sum, "P2: filter_map with ok()");
    }
}
