//! LESSON 08 — Casts and closure expressions
//! Run: cargo test --test chapter06 l08_
//!
//! PREDICT before running: What does 300_u16 as u8 produce? Does `move` capture always make a closure callable only once?
//! IMPLEMENT the tasks below. Preserve the tests and add an edge case.
//! EXPLAIN afterward: Explain narrowing and float-to-integer casts, closure environment capture, and why move captures and consuming captured data are different.
//! COMPILER / TRANSFER LAB: Create a closure borrowing a threshold, then mutate threshold before another call; inspect the borrow conflict. Change to move for a Copy threshold and explain the independent captured value. Try a function item and an equivalent closure with a captured local.
//! Use examples/ch06_scratch.rs; repair intentional failures before committing.

/// A. Return (truncated_low_byte, saturating_float_cast). Cast as u8 in both
/// cases. Rust float casts saturate to 0..=255, with NaN becoming zero.
fn cast_pair(integer: u16, float: f64) -> (u8, u8) {
    todo!("Implement this task")
}
/// B. Complete the move closure predicate: test value >= captured minimum.
/// Its typed skeleton keeps the unfinished opaque return type compilable.
/// Explain why moving the threshold into this closure permits repeated calls.
fn at_least(minimum: i32) -> impl Fn(i32) -> bool {
    move |value: i32| todo!("Implement this task")
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a_cast_policies_differ() {
        assert_eq!(cast_pair(300, 300.9), (44, 255));
        assert_eq!(cast_pair(256, -2.0), (0, 0));
        assert_eq!(cast_pair(255, f64::NAN), (255, 0));
        assert_eq!(cast_pair(0, 3.9), (0, 3));
    }
    #[test]
    fn b_closure_retains_its_environment() {
        let test = at_least(7);
        assert!(!test(6));
        assert!(test(7));
        assert!(test(8));
        let negative = at_least(-2);
        assert!(negative(-1));
        assert!(!negative(-3));
    }
}
