//! LESSON 06 — Tests and executable documentation
//! Run: cargo test --test chapter08 l06_
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider: Can an integration test see private library items? Are rustdoc examples compiled by cargo test by default?
//! IMPLEMENT the tasks below. Preserve the tests and add an edge case.
//! EXPLAIN afterward: Explain unit versus integration tests and doc-tests, including what a public consumer can prove.
//! COMPILER / TRANSFER LAB: In a disposable package move normalize into src/lib.rs, add a rustdoc example with a success assertion, add a compile_fail example showing forbidden private access, then add tests/public_api.rs. Run cargo test, cargo test --doc, cargo doc --no-deps. Intentionally break one documented example to confirm it is executed. Inspect the rendered docs; use # lines only for necessary setup, not to hide the behavior being taught.
//! Use examples/ch08_scratch.rs; repair intentional failures before committing.

/// A. Normalize words by splitting Unicode whitespace and joining with one
/// ASCII space. Preserve case and all non-whitespace characters.
pub fn normalize(text: &str) -> String {
    todo!("Implement this task")
}
/// B. Return true only when text already equals the public normalized form.
/// Reuse normalize, so callers observe one consistent rule.
pub fn is_normalized(text: &str) -> bool {
    todo!("Implement this task")
}
#[cfg(test)]
mod tests {
    use super::*;
    use practice::guess;
    use std::panic::catch_unwind;
    #[test]
    fn a_public_contract_is_deterministic() {
        assert_eq!(normalize(" \tHello\n 港  "), "Hello 港");
        assert_eq!(normalize("\u{2003}a\u{2003}b"), "a b");
        assert_eq!(normalize(""), "");
        assert_eq!(normalize("a\u{a0}b"), "a b");
        assert_eq!(normalize("ÀB c"), "ÀB c");
        assert_eq!(normalize(" \n\t "), "");
    }
    #[test]
    fn b_idempotent_public_behavior() {
        assert!(is_normalized("Hello 港"));
        assert!(is_normalized(""));
        assert!(!is_normalized(" a"));
        assert!(!is_normalized("a  b"));
        assert!(!is_normalized("a\tb"));
        assert!(!is_normalized("a "));
        let out = normalize(" a \n b ");
        assert!(is_normalized(&out));
    }

    fn payload_text(payload: Box<dyn std::any::Any + Send>) -> String {
        match payload.downcast::<&str>() {
            Ok(text) => text.to_string(),
            Err(payload) => *payload.downcast::<String>().expect("a text payload"),
        }
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    /// The caught panic's message in the test output is expected.
    #[test]
    fn p_predictions() {
        // P1. The exact panic message of a failing assert! with no message.
        let failure = catch_unwind(|| {
            let total = 3;
            let expected = 4;
            assert!(total == expected);
        });
        let p1: &str = guess();
        assert_eq!(
            p1,
            payload_text(failure.unwrap_err()),
            "P1: assert! reports the expression, not the values"
        );

        // P2. U+00A0 is a no-break space. Words found by each splitter:
        let text = "a\u{a0}b c";
        let p2: (usize, usize) = guess();
        assert_eq!(
            p2,
            (
                text.split_whitespace().count(),
                text.split_ascii_whitespace().count()
            ),
            "P2: Unicode versus ASCII whitespace"
        );
    }
}
