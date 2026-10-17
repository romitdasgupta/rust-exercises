//! LESSON 02 — Catching Result errors and printing context
//! Run: cargo test --test chapter07 l02_
//!
//! PREDICT before running: Does matching an Err require unwinding? What is lost by replacing every error with zero?
//! IMPLEMENT the tasks below. Preserve the tests and add an edge case.
//! EXPLAIN afterward: Explain recovery policy and how a user-facing message differs from the underlying parse error.
//! COMPILER / TRANSFER LAB: Parse an invalid integer, print its Display and Debug forms, and walk through a match with separate Ok/Err arms. Explain why stderr is a sensible destination for a CLI diagnostic.
//! Use examples/ch07_scratch.rs; repair intentional failures before committing.

/// A. Parse a trimmed u16 using match; return the fallback on any parse error.
fn parse_or(text: &str, fallback: u16) -> u16 {
    todo!("Implement this task")
}
/// B. Render either "NAME=VALUE" or "NAME: invalid count". This deliberately
/// stable boundary message must not depend on ParseIntError's Display wording.
fn describe(name: &str, result: Result<u16, std::num::ParseIntError>) -> String {
    todo!("Implement this task")
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a_fallback_is_an_explicit_policy() {
        assert_eq!(parse_or(" 12 ", 7), 12);
        assert_eq!(parse_or("65536", 7), 7);
        assert_eq!(parse_or("", 8), 8);
    }
    #[test]
    fn b_context_survives_error_rendering() {
        assert_eq!(describe("jobs", "4".parse()), "jobs=4");
        assert_eq!(describe("jobs", "no".parse()), "jobs: invalid count");
    }
}
