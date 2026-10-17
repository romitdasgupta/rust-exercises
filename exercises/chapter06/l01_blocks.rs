//! LESSON 01 — Expression blocks and declarations
//! Run: cargo test --test chapter06 l01_
//!
//! PREDICT before running: What are the types of { 3 } and { 3; }? Can a shadowed binding have a different type?
//! IMPLEMENT the tasks below. Preserve the tests and add an edge case.
//! EXPLAIN afterward: Explain tail expressions, unit, shadowing versus mutation, and where nested function items can be used.
//! COMPILER / TRANSFER LAB: Write an if whose branches have i32 and () types; repair the semicolon. Declare a nested function after its call and compare its visibility with a local let binding. Try reading a local variable from that function item.
//! Use examples/ch06_scratch.rs; repair intentional failures before committing.

/// A. Use a block expression to trim text, then shadow the trimmed &str with
/// its byte length. Return (length, length_is_zero), without allocating.
fn inspected(text: &str) -> (usize, bool) {
    todo!("Implement this task")
}
/// B. Define a nested function item that doubles i16 into i32; use it to
/// return the sum of the doubled inputs. Put the item after its first call.
fn local_item(a: i16, b: i16) -> i32 {
    todo!("Implement this task")
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a_tail_expression_returns_pair() {
        assert_eq!(inspected(" é "), (2, false));
        assert_eq!(inspected(" \n"), (0, true));
    }
    #[test]
    fn b_item_scope_and_widening() {
        assert_eq!(local_item(4, -1), 6);
        assert_eq!(local_item(i16::MAX, i16::MAX), 131068);
    }
}
