//! LESSON 08 — Pattern contexts and public compatibility
//! Run: cargo test --test chapter10 l08_
//!
//! PREDICT before running: Can let bind a refutable Some pattern without else? Why should an external match on a non_exhaustive enum include a wildcard?
//! IMPLEMENT the tasks below. Preserve the tests and add an edge case.
//! EXPLAIN afterward: List pattern contexts: let, function parameters, for, match, if let, while let, and let else. Explain when each permits refutable patterns.
//! COMPILER / TRANSFER LAB: Make a disposable library with #[non_exhaustive] pub enum Event { Open, Closed } and an external binary matching it. Observe the wildcard requirement there versus inside the defining crate. Add a new variant and rerun. Also put #[non_exhaustive] on a struct-like variant and inspect external construction/pattern restrictions. Do not add a wildcard blindly to closed internal enums, because exhaustive checking can catch missing behavior.
//! Use examples/ch10_scratch.rs; repair intentional failures before committing.

/// A. Pop all owned strings using while let Some(value), returning them in
/// reverse input order. Do not clone, index, or call reverse.
fn drain_back(mut values: Vec<String>) -> Vec<String> {
    todo!("Implement this task")
}
/// B. Use let else with tuple destructuring. Require a '=' and a nonempty
/// left half; return the borrowed halves unchanged, with no trimming.
fn assignment(text: &str) -> Option<(&str, &str)> {
    todo!("Implement this task")
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a_repeated_pattern_moves_payloads() {
        assert_eq!(
            drain_back(vec!["a".into(), "b".into(), "c".into()]),
            ["c", "b", "a"]
        );
        assert!(drain_back(vec![]).is_empty());
    }
    #[test]
    fn b_refutable_let_has_a_diverging_else() {
        assert_eq!(assignment("x=y=z"), Some(("x", "y=z")));
        assert_eq!(assignment("x="), Some(("x", "")));
        assert_eq!(assignment("=y"), None);
        assert_eq!(assignment("none"), None);
        assert_eq!(assignment(" x = y "), Some((" x ", " y ")));
    }
}
