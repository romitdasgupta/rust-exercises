//! LESSON 06 — Custom errors with a source chain
//! Run: cargo test --test chapter07 l06_
//!
//! PREDICT before running: Why should an error preserve a lower-level source rather than only its formatted message? Must every error have a source?
//! IMPLEMENT the tasks below. Preserve the tests and add an edge case.
//! EXPLAIN afterward: Explain the roles of Debug, Display, Error::source, and typed variants for caller recovery.
//! COMPILER / TRANSFER LAB: Print a custom error with Display and Debug, then manually traverse source() until None. Design a new domain-specific variant and explain whether it has a lower-level cause.
//! Use examples/ch07_scratch.rs; repair intentional failures before committing.

use std::{error::Error, fmt, num::ParseIntError};
#[derive(Debug)]
enum LimitError {
    Parse(ParseIntError),
    TooLarge(u32),
}
impl fmt::Display for LimitError {
    /// A1. Parse => "invalid limit"; TooLarge(n) => "limit N exceeds 100".
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!("Implement this task")
    }
}
impl Error for LimitError {
    /// A2. Preserve ParseIntError as source. TooLarge has no source.
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        todo!("Implement this task")
    }
}
/// B. Parse trimmed u32; reject values above 100; zero is valid.
fn limit(text: &str) -> Result<u32, LimitError> {
    todo!("Implement this task")
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a_display_and_source_are_separate() {
        let p = LimitError::Parse("bad".parse::<u32>().unwrap_err());
        assert_eq!(p.to_string(), "invalid limit");
        assert!(p.source().unwrap().is::<ParseIntError>());
        let t = LimitError::TooLarge(101);
        assert_eq!(t.to_string(), "limit 101 exceeds 100");
        assert!(t.source().is_none());
    }
    #[test]
    fn b_custom_error_preserves_domain() {
        assert_eq!(limit(" 100 ").unwrap(), 100);
        assert_eq!(limit("0").unwrap(), 0);
        assert!(matches!(limit("101"), Err(LimitError::TooLarge(101))));
        assert!(matches!(limit("bad"), Err(LimitError::Parse(_))));
    }
}
