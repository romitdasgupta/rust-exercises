//! LESSON 01 — Ownership scopes
//! Run: cargo test --test chapter04 l01_
//!
//! PREDICT before running: Does dropping a Vec<String> also drop its elements? Does a move allocate a new string buffer?
//! IMPLEMENT the tasks below. Preserve the tests and add an edge case.
//! EXPLAIN afterward: Draw the owner tree for a vector of strings and explain when destruction happens.
//! COMPILER / TRANSFER LAB: Create a nested block containing a String; try using it outside the block. Then move it into an outer binding and show the repaired owner tree. Use a tiny Drop logger to observe reverse declaration order.
//! Use examples/ch04_scratch.rs; repair intentional failures before committing.

/// A. Build two owned labels, "north" and "south", inside a nested block;
/// return their vector so its ownership leaves the block.
fn labels() -> Vec<String> {
    todo!("Implement this task")
}
/// B. Consume the String and return it unchanged. Do not clone it.
fn handoff(value: String) -> String {
    todo!("Implement this task")
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a_nested_scope_returns_owners() {
        assert_eq!(labels(), ["north", "south"]);
    }
    #[test]
    fn b_handoff_preserves_allocation() {
        let s = String::from("🦀 route");
        let p = s.as_ptr();
        let out = handoff(s);
        assert_eq!(out, "🦀 route");
        assert_eq!(out.as_ptr(), p);
        assert_eq!(handoff(String::new()), "");
    }
}
