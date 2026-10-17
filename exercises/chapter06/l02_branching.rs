//! LESSON 02 — if and match as values
//! Run: cargo test --test chapter06 l02_
//!
//! PREDICT before running: Must if have an else when its value is used as i32? Can match arms overlap, and which arm wins?
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
    #[test]
    fn a_if_returns_selected_value() {
        assert_eq!(bounded(-3, 0, 8), 0);
        assert_eq!(bounded(4, 0, 8), 4);
        assert_eq!(bounded(99, 0, 8), 8);
        assert_eq!(bounded(7, 2, 2), 2);
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
}
