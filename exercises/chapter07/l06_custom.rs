//! LESSON 06 — Custom errors with a source chain
//! Run: cargo test --test chapter07 l06_
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider: Why should an error preserve a lower-level source rather than only its formatted message? Must every error have a source?
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
    use practice::guess;
    use std::io;
    #[test]
    fn a_display_and_source_are_separate() {
        let p = LimitError::Parse("bad".parse::<u32>().unwrap_err());
        assert_eq!(p.to_string(), "invalid limit");
        assert!(p.source().unwrap().is::<ParseIntError>());
        let t = LimitError::TooLarge(101);
        assert_eq!(t.to_string(), "limit 101 exceeds 100");
        assert!(t.source().is_none());
        assert_eq!(
            LimitError::TooLarge(4_000_000_000).to_string(),
            "limit 4000000000 exceeds 100"
        );
    }
    #[test]
    fn b_custom_error_preserves_domain() {
        assert_eq!(limit(" 100 ").unwrap(), 100);
        assert_eq!(limit("0").unwrap(), 0);
        assert!(matches!(limit("101"), Err(LimitError::TooLarge(101))));
        assert!(matches!(limit("bad"), Err(LimitError::Parse(_))));
        assert!(matches!(
            limit("4294967295"),
            Err(LimitError::TooLarge(4_294_967_295))
        ));
        assert!(matches!(limit("4294967296"), Err(LimitError::Parse(_))));
        assert!(matches!(limit("-1"), Err(LimitError::Parse(_))));
    }
    #[test]
    fn b_parse_error_is_kept_as_the_source() {
        let err = limit("").unwrap_err();
        let source = err.source().and_then(|s| s.downcast_ref::<ParseIntError>());
        assert_eq!(
            source.map(ParseIntError::kind),
            Some(&std::num::IntErrorKind::Empty)
        );
    }

    #[derive(Debug)]
    struct Outer(ParseIntError);
    impl fmt::Display for Outer {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "could not read setting")
        }
    }
    impl Error for Outer {
        fn source(&self) -> Option<&(dyn Error + 'static)> {
            Some(&self.0)
        }
    }

    /// Counts the errors visited by following source() from `error`.
    fn chain_len(error: &dyn Error) -> usize {
        let mut count = 1;
        let mut current = error.source();
        while let Some(next) = current {
            count += 1;
            current = next.source();
        }
        count
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    #[test]
    fn p_predictions() {
        // P1. Chain lengths: Outer wrapping a ParseIntError, and io::Error::other
        // wrapping one. (io::Error's source() reports its inner error's source.)
        let parse = || "x".parse::<u8>().unwrap_err();
        let outer = Outer(parse());
        let wrapped = io::Error::other(parse());
        let p1: (usize, usize) = guess();
        assert_eq!(
            p1,
            (chain_len(&outer), chain_len(&wrapped)),
            "P1: walking source()"
        );

        // P2. Derived Debug output for a variant (Display is what you wrote).
        let p2: &str = guess();
        assert_eq!(
            p2,
            format!("{:?}", LimitError::TooLarge(101)),
            "P2: derived Debug shows the variant"
        );
    }
}
