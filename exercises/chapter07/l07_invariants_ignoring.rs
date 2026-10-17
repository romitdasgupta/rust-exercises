//! LESSON 07 — Impossible errors and deliberate recovery
//! Run: cargo test --test chapter07 l07_
//!
//! PREDICT before running: When is expect justified? Does assigning a Result to _ mean success? Why does unused Result warn?
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
    }
}
