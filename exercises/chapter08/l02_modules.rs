//! LESSON 02 — Nested modules and private implementation
//! Run: cargo test --test chapter08 l02_
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider: Can a parent access a private item in a child module? Can a child access private items in an ancestor?
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
    use practice::guess;
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

    mod english {
        pub fn greet() -> &'static str {
            "hello"
        }
        pub fn farewell() -> &'static str {
            "bye"
        }
    }
    mod french {
        pub fn greet() -> &'static str {
            "bonjour"
        }
    }
    mod phrases {
        pub use super::english::*;
        pub use super::french::greet;
    }

    const LEVEL: &str = "tests";
    mod outer {
        pub const LEVEL: &str = "outer";
        pub mod inner {
            const LEVEL: &str = "inner";
            pub fn levels() -> (&'static str, &'static str) {
                (self::LEVEL, super::LEVEL)
            }
        }
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    #[test]
    fn p_predictions() {
        // P1. A glob import and an explicit import both offer `greet`.
        let p1: (&str, &str) = guess();
        assert_eq!(
            p1,
            (phrases::greet(), phrases::farewell()),
            "P1: explicit imports shadow glob imports"
        );

        // P2. What do self:: and super:: name inside outer::inner?
        let p2: (&str, &str) = guess();
        assert_eq!(p2, outer::inner::levels(), "P2: relative paths");
        assert_eq!(LEVEL, "tests");
    }
}
