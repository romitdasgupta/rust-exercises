//! LESSON 03 — if let and let else
//! Run: cargo test --test chapter06 l03_
//!
//! PREDICT before running: Must let else have a diverging else block? When is a variable bound by if let available afterward?
//! IMPLEMENT the tasks below. Preserve the tests and add an edge case.
//! EXPLAIN afterward: Explain refutable versus irrefutable patterns and the scope of successful bindings.
//! COMPILER / TRANSFER LAB: Write let Some(x) = input else { 0 }; and inspect the error. Repair using return. Compare an if let expression that provides a fallback with a let else statement that guards the rest of a function.
//! Use examples/ch06_scratch.rs; repair intentional failures before committing.

/// A. Add a present i16 bonus to the i32 base using if let; absent means base.
fn bonus(base: i32, extra: Option<i16>) -> i32 {
    todo!("Implement this task")
}
/// B. Use let else to require a colon and a nonempty trimmed key. Return the
/// trimmed borrowed key; reject missing colon or empty key with None.
fn required_key(line: &str) -> Option<&str> {
    todo!("Implement this task")
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a_if_let_has_an_absent_path() {
        assert_eq!(bonus(10, Some(-3)), 7);
        assert_eq!(bonus(10, None), 10);
    }
    #[test]
    fn b_let_else_guards_remainder() {
        assert_eq!(required_key(" mode :quiet"), Some("mode"));
        assert_eq!(required_key(":v"), None);
        assert_eq!(required_key("x"), None);
        assert_eq!(required_key("港:"), Some("港"));
    }
}
