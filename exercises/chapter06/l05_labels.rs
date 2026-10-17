//! LESSON 05 — Loop values and labeled blocks
//! Run: cargo test --test chapter06 l05_
//!
//! PREDICT before running: Can break carry a value from a for loop? How does return inside a nested loop differ from break to a label?
//! IMPLEMENT the tasks below. Preserve the tests and add an edge case.
//! EXPLAIN afterward: Explain the never type, value-producing loop, labeled early exits, and why an unconditional loop can initialize a non-Option result.
//! COMPILER / TRANSFER LAB: Write a nested search with a labeled outer loop, then a labeled block. Compare unlabeled break, break to a label, and return. Type a branch as return and inspect why it coexists with another branch returning an integer.
//! Use examples/ch06_scratch.rs; repair intentional failures before committing.

/// A. Find the least u32 n whose square is >= bound (bound is u32).
/// Use a value-producing loop and break n. Widen before multiplying.
fn ceiling_root(bound: u32) -> u32 {
    todo!("Implement this task")
}
/// B. Use a labeled block with early breaks to return Some((left,right)) only
/// when a '/' exists and both halves are nonempty. Split at the first '/'.
fn nonempty_pair(text: &str) -> Option<(&str, &str)> {
    todo!("Implement this task")
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a_loop_value_matches_ceiling() {
        assert_eq!(ceiling_root(0), 0);
        assert_eq!(ceiling_root(16), 4);
        assert_eq!(ceiling_root(17), 5);
        assert_eq!(ceiling_root(u32::MAX), 65536);
    }
    #[test]
    fn b_labeled_block_has_early_exits() {
        assert_eq!(nonempty_pair("a/b/c"), Some(("a", "b/c")));
        assert_eq!(nonempty_pair("a"), None);
        assert_eq!(nonempty_pair("/b"), None);
        assert_eq!(nonempty_pair("a/"), None);
    }
}
