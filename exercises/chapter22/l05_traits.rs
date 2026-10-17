//! LESSON 05 — Traits handle macro leaf types
//! Run: cargo test --test chapter22 l05_
//!
//! PREDICT before running:
//! 1. Should a macro inspect the spelling of a variable to infer its type?
//! 2. Can a generic helper centralize type conversion for many macro invocations?
//!
//! IMPLEMENT the tasks; keep the tests.
//! EXPLAIN: Explain syntax/type separation and the trait bound error for an unsupported leaf.
//! COMPILER LAB (examples/ch22_scratch.rs): Try passing Vec<u8> to row!; add a trait implementation only if you can define an unambiguous representation.

trait Field {
    fn render(self) -> String;
}
impl Field for &str {
    fn render(self) -> String {
        self.to_owned()
    }
}
impl Field for i32 {
    fn render(self) -> String {
        self.to_string()
    }
}
impl Field for bool {
    fn render(self) -> String {
        if self { "yes".into() } else { "no".into() }
    }
}
/// A. Render a value by its Field implementation, without Debug formatting.
fn render<T: Field>(value: T) -> String {
    todo!("A: implement the contract")
}
/// B. Render zero or more expressions into a Vec<String>, via render. Support
/// trailing commas and evaluate all expressions exactly once, left to right.
macro_rules! row {
    ($($value:expr),* $(,)?) => {{ todo!("B: implement the contract") }};
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn leaves_use_trait_behavior() {
        let value: i32 = 12;
        let row: Vec<String> = row!("item", value, true,);
        assert_eq!(row, ["item", "12", "yes"]);
        let empty: Vec<String> = row!();
        assert!(empty.is_empty());
    }
    #[test]
    fn expressions_are_evaluated_once() {
        let mut n = 0;
        let out: Vec<String> = row!(
            {
                n += 1;
                n
            },
            {
                n += 1;
                n
            }
        );
        assert_eq!(out, ["1", "2"]);
        assert_eq!(n, 2);
    }
}
