//! LESSON 06 — Exports and hygiene
//! Run: cargo test --test chapter22 l06_
//!
//! PREDICT before running:
//! 1. Does an exported macro know the caller has imported its helper?
//! 2. Can a caller variable with the same spelling capture an internal temporary?
//!
//! IMPLEMENT the tasks; keep the tests.
//! EXPLAIN: Explain $crate resolution and the different roles of hygienic locals and referenced item paths.
//! COMPILER LAB (examples/ch22_scratch.rs): Move the macro invocation into a nested module with a same-named helper. Then imagine the library dependency is renamed in Cargo.toml; explain why $crate still works.

/// A. Format a name using a helper that an exported macro can reach.
/// It must return "event:<name>".
pub fn format_event(name: &str) -> String {
    todo!("A: implement the contract")
}
/// B. Use a definition-site crate path for the helper, and evaluate name once.
/// Keep the intentionally chapter-specific export name to avoid crate collisions.
#[macro_export]
macro_rules! ch22_event {
    ($name:expr) => {{ todo!("B: implement the contract") }};
}

#[cfg(test)]
mod tests {
    use super::*;
    mod caller {
        fn format_event(_: &str) -> String {
            "wrong helper".into()
        }
        pub fn invoke() -> String {
            let name = "caller";
            let out: String = crate::ch22_event!(name);
            assert_eq!(name, "caller");
            out
        }
    }
    #[test]
    fn exported_macro_resolves_its_own_helper() {
        assert_eq!(caller::invoke(), "event:caller");
        let mut calls = 0;
        let out: String = crate::ch22_event!({
            calls += 1;
            "once"
        });
        assert_eq!(out, "event:once");
        assert_eq!(calls, 1);
        assert_eq!(format_event("x"), "event:x");
    }
}
