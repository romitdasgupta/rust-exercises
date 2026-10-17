//! LESSON 01 — Expansion and evaluation once
//! Run: cargo test --test chapter22 l01_
//!
//! PREDICT before running:
//! 1. Can a macro accidentally execute an argument twice?
//! 2. Do braces in a macro expansion make its locals visible afterward?
//!
//! IMPLEMENT the tasks; keep the tests.
//! EXPLAIN: Trace expansion and evaluation separately. Explain why syntax substitution can change control flow.
//! COMPILER LAB (examples/ch22_scratch.rs): Expand an argument containing return inside a function. Compare a macro with a function; record the different return scopes.

/// A. Evaluate an i32 expression exactly once; return twice its value.
/// Inputs guarantee no arithmetic overflow. Keep the temporary hygienic/local.
macro_rules! double_once {
    ($value:expr) => {{ todo!("A: implement the contract") }};
}
/// B. Return (file, line, module) using built-in macros at the invocation site.
macro_rules! location {
    () => {{ todo!("B: implement the contract") }};
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn side_effect_runs_once_and_local_is_hygienic() {
        let mut calls = 0;
        let value = 50;
        let answer: i32 = double_once!({
            calls += 1;
            7
        });
        assert_eq!(answer, 14);
        assert_eq!(calls, 1);
        assert_eq!(value, 50);
    }
    #[test]
    fn builtins_capture_invocation() {
        let (file, line, module): (&str, u32, &str) = location!();
        assert!(line > 0 && line <= line!());
        assert!(file.ends_with("l01_expansion.rs"));
        assert!(module.contains("l01_expansion"));
    }
}
