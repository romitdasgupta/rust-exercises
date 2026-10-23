//! LESSON 02 — if and match as values
//! Run: cargo test --test chapter06 l02_
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider: Must if have an else when its value is used as i32? Can match arms overlap, and which arm wins?
//! IMPLEMENT the tasks below. Preserve the tests and add an edge case.
//! EXPLAIN afterward: Explain exhaustiveness and why every reachable value-producing branch needs a compatible type.
//! COMPILER / TRANSFER LAB: Remove the final wildcard of an integer match and inspect non-exhaustiveness. Try if with an integer condition. Place a broad pattern before a specific one and explain the warning.
//! Use examples/ch06_scratch.rs; repair intentional failures before committing.

/// A. Clamp value to the inclusive range low..=high, using an if expression.
/// Precondition: low <= high. Do not call clamp/min/max.
fn bounded(value: i32, low: i32, high: i32) -> i32 {
    todo!("Implement this task")
}
/// B. Classify code using match: 0 => "idle", 1 or 2 => "busy",
/// 3 through 9 => "queued", all other u8 values => "unknown".
fn status(code: u8) -> &'static str {
    todo!("Implement this task")
}
#[cfg(test)]
mod tests {
    use super::*;
    use practice::guess;
    #[test]
    fn a_if_returns_selected_value() {
        assert_eq!(bounded(-3, 0, 8), 0);
        assert_eq!(bounded(4, 0, 8), 4);
        assert_eq!(bounded(99, 0, 8), 8);
        assert_eq!(bounded(7, 2, 2), 2);
        assert_eq!(bounded(0, 0, 8), 0);
        assert_eq!(bounded(i32::MIN, -5, 5), -5);
        assert_eq!(bounded(i32::MAX, i32::MIN, i32::MAX), i32::MAX);
    }
    #[test]
    fn b_match_is_exhaustive() {
        for (c, s) in [
            (0, "idle"),
            (1, "busy"),
            (2, "busy"),
            (3, "queued"),
            (9, "queued"),
            (10, "unknown"),
            (255, "unknown"),
        ] {
            assert_eq!(status(c), s);
        }
    }

    fn size(n: i32) -> &'static str {
        match n {
            n if n < 10 => "small",
            5 => "five",
            _ => "large",
        }
    }

    fn next(calls: &mut u32) -> u32 {
        *calls += 1;
        *calls
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    #[test]
    fn p_predictions() {
        // P1. Arms are tried top to bottom. What does size(5) return?
        let p1: &str = guess();
        assert_eq!(p1, size(5), "P1: the first matching arm wins");

        // P2. An `@` binding keeps the matched value. What is `r`?
        let r = match 42u8 {
            n @ 40..=49 => n - 40,
            _ => 0,
        };
        let p2: u8 = guess();
        assert_eq!(p2, r, "P2: binding with @ in a range pattern");

        // P3. Each `else if` condition is a new call. Which branch, after how many calls?
        let mut calls = 0;
        let branch = if next(&mut calls) == 2 {
            "a"
        } else if next(&mut calls) == 2 {
            "b"
        } else {
            "c"
        };
        let p3: (&str, u32) = guess();
        assert_eq!(p3, (branch, calls), "P3: conditions run in order");

        // P4. The same test as a match: the scrutinee is evaluated once.
        let mut calls = 0;
        let branch = match next(&mut calls) {
            2 => "a",
            3 => "b",
            _ => "c",
        };
        let p4: (&str, u32) = guess();
        assert_eq!(
            p4,
            (branch, calls),
            "P4: a match evaluates its scrutinee once"
        );
    }
}
