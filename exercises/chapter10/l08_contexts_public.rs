//! LESSON 08 — Pattern contexts and public compatibility
//! Run: cargo test --test chapter10 l08_
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider: Can let bind a refutable Some pattern without else? Why should an external match on a non_exhaustive enum include a wildcard?
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
#[allow(clippy::question_mark)] // `?` would also work; this task practices let else
fn assignment(text: &str) -> Option<(&str, &str)> {
    todo!("Implement this task")
}
#[cfg(test)]
mod tests {
    use super::*;
    use practice::{alloc::count, guess};
    #[test]
    fn a_repeated_pattern_moves_payloads() {
        assert_eq!(
            drain_back(vec!["a".into(), "b".into(), "c".into()]),
            ["c", "b", "a"]
        );
        assert!(drain_back(vec![]).is_empty());
        let values: Vec<String> = (0..100).map(|i| format!("v{i}")).collect();
        let first = values[0].as_ptr();
        let (out, allocations) = count(|| drain_back(values));
        assert_eq!((out[0].as_str(), out[99].as_str()), ("v99", "v0"));
        assert_eq!(out[99].as_ptr(), first, "Strings move; they are not cloned");
        assert!(
            allocations <= 10,
            "100 moves cost {allocations} allocations"
        );
    }
    #[test]
    fn b_refutable_let_has_a_diverging_else() {
        assert_eq!(assignment("x=y=z"), Some(("x", "y=z")));
        assert_eq!(assignment("x="), Some(("x", "")));
        assert_eq!(assignment("=y"), None);
        assert_eq!(assignment("none"), None);
        assert_eq!(assignment(" x = y "), Some((" x ", " y ")));
        assert_eq!(assignment("="), None);
        assert_eq!(assignment(""), None);
        assert_eq!(assignment("ключ=значение"), Some(("ключ", "значение")));
        let text = String::from("k=v");
        let (k, v) = assignment(&text).unwrap();
        assert_eq!(
            (k.as_ptr(), v.as_ptr()),
            (text.as_ptr(), text[2..].as_ptr())
        );
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    #[test]
    fn p_predictions() {
        // P1. let else inside a loop: what is collected?
        let mut parsed = Vec::new();
        for line in ["a=1", "b", "=2", "c=x"] {
            let Some((_, value)) = line.split_once('=') else {
                continue;
            };
            let Ok(n) = value.parse::<i32>() else {
                parsed.push(-1);
                continue;
            };
            parsed.push(n);
        }
        let p1: Vec<i32> = guess();
        assert_eq!(p1, parsed, "P1: let else diverges");

        // P2. A let chain (Rust 2024): which branch runs?
        let a = Some(5);
        let b: Result<i32, ()> = Err(());
        let r = if let Some(x) = a
            && x > 3
            && let Ok(y) = b
        {
            x + y
        } else if let Some(x) = a {
            x * 100
        } else {
            0
        };
        let p2: i32 = guess();
        assert_eq!(p2, r, "P2: every condition in a chain must hold");
    }
}
