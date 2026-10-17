//! LESSON 02 — Nested modules and private implementation
//! Run: cargo test --test chapter08 l02_
//!
//! PREDICT before running: Can a parent access a private item in a child module? Can a child access private items in an ancestor?
//! IMPLEMENT the tasks below. Preserve the tests and add an edge case.
//! EXPLAIN afterward: Explain why a module boundary should expose operations that preserve invariants instead of raw representation.
//! COMPILER / TRANSFER LAB: Copy the inline module to a disposable library, then split it into src/tariff.rs and src/tariff/rules.rs without changing caller paths. Try access to the private helper from the parent. Compare mod tariff; with use tariff::cost;
//! Use examples/ch08_scratch.rs; repair intentional failures before committing.

mod tariff {
    mod rules {
        /// A. Private-to-tariff fee calculation: base 3 plus 2 per item.
        pub(super) fn fee(items: u16) -> u32 {
            todo!("Implement this task")
        }
    }
    /// B. Public API: free for zero items; otherwise delegate to rules::fee.
    pub fn cost(items: u16) -> u32 {
        todo!("Implement this task")
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a_public_boundary_uses_private_rules() {
        assert_eq!(tariff::cost(1), 5);
        assert_eq!(tariff::cost(4), 11);
        assert_eq!(tariff::cost(u16::MAX), 131073);
    }
    #[test]
    fn b_empty_order_is_free() {
        assert_eq!(tariff::cost(0), 0);
    }
}
