//! LESSON 03 — if let and let else
//! Run: cargo test --test chapter06 l03_
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider: Must let else have a diverging else block? When is a variable bound by if let available afterward?
//! IMPLEMENT the tasks below. Preserve the tests and add an edge case.
//! EXPLAIN afterward: Explain refutable versus irrefutable patterns and the scope of successful bindings.
//! COMPILER / TRANSFER LAB: Write let Some(x) = input else { 0 }; and inspect the error. Repair using return. Compare an if let expression that provides a fallback with a let else statement that guards the rest of a function.
//! Use examples/ch06_scratch.rs; repair intentional failures before committing.

/// A. Add a present i16 bonus to the i32 base using if let; absent means base.
fn bonus(base: i32, extra: Option<i16>) -> i32 {
    todo!("Implement this task")
}
/// B. Use let else to require a colon and a nonempty trimmed key. The key is
/// the text before the FIRST colon. Return the trimmed key borrowed from
/// `line`; reject a missing colon or an empty key with None. (Clippy suggests
/// `?` for the colon guard; Chapter 7 shows `?` is shorthand for this let else.)
fn required_key(line: &str) -> Option<&str> {
    todo!("Implement this task")
}
#[cfg(test)]
mod tests {
    use super::*;
    use practice::guess;
    #[test]
    fn a_if_let_has_an_absent_path() {
        assert_eq!(bonus(10, Some(-3)), 7);
        assert_eq!(bonus(10, None), 10);
        assert_eq!(bonus(0, Some(i16::MIN)), -32768);
        assert_eq!(bonus(i32::MAX - 1, Some(1)), i32::MAX);
    }
    #[test]
    fn b_let_else_guards_remainder() {
        assert_eq!(required_key(" mode :quiet"), Some("mode"));
        assert_eq!(required_key(":v"), None);
        assert_eq!(required_key("x"), None);
        assert_eq!(required_key("港:"), Some("港"));
        assert_eq!(required_key(" \t :v"), None);
        assert_eq!(required_key("a:b:c"), Some("a"));
        assert_eq!(required_key("\u{3000}key\u{3000}: v"), Some("key"));
    }
    #[test]
    fn b_key_is_borrowed_from_the_line() {
        let line = String::from("  name : value");
        let key = required_key(&line).unwrap();
        let start = line.as_ptr() as usize;
        assert_eq!(key.as_ptr() as usize, start + 2, "return a slice of `line`");
    }

    fn first_even(values: &[i32]) -> i32 {
        let [first, ..] = values else {
            return -1;
        };
        if first % 2 == 0 { *first } else { 0 }
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    #[test]
    fn p_predictions() {
        // P1. `while let` runs until the pattern stops matching.
        let mut stack = vec![1, 2, 3];
        let mut digits = 0;
        while let Some(top) = stack.pop() {
            digits = digits * 10 + top;
        }
        let p1: i32 = guess();
        assert_eq!(p1, digits, "P1: while let over pop");

        // P2. let else on a slice pattern: values for [], [3, 4] and [8].
        let p2: (i32, i32, i32) = guess();
        assert_eq!(
            p2,
            (first_even(&[]), first_even(&[3, 4]), first_even(&[8])),
            "P2: the else branch runs only when the pattern fails"
        );

        // P3. if let with a literal inside the pattern.
        let reading = Some(0u8.wrapping_sub(1));
        let p3: &str = guess();
        let kind = if let Some(255) = reading {
            "max"
        } else {
            "other"
        };
        assert_eq!(p3, kind, "P3: patterns can contain literals");
    }
}
