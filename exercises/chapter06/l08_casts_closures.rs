//! LESSON 08 — Casts and closure expressions
//! Run: cargo test --test chapter06 l08_
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider: What does 300_u16 as u8 produce? Does `move` capture always make a closure callable only once?
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
    use practice::guess;
    #[test]
    fn a_cast_policies_differ() {
        assert_eq!(cast_pair(300, 300.9), (44, 255));
        assert_eq!(cast_pair(256, -2.0), (0, 0));
        assert_eq!(cast_pair(255, f64::NAN), (255, 0));
        assert_eq!(cast_pair(0, 3.9), (0, 3));
        assert_eq!(cast_pair(u16::MAX, f64::INFINITY), (255, 255));
        assert_eq!(cast_pair(0x1234, f64::NEG_INFINITY), (0x34, 0));
        assert_eq!(cast_pair(1, -0.9), (1, 0));
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
        assert!(at_least(i32::MIN)(i32::MIN));
        assert!(!at_least(i32::MAX)(i32::MAX - 1));
    }
    #[test]
    fn b_closure_owns_a_copy_of_the_minimum() {
        let test = at_least(7);
        assert_eq!(
            size_of_val(&test),
            size_of::<i32>(),
            "the closure's environment should be exactly the moved i32"
        );
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    #[test]
    fn p_predictions() {
        // P1. Integer and float casts: reinterpret, truncate toward zero.
        let (negative, small, below) = (-1i8, 2.9f32, -2.9f64);
        let p1: (u8, i8, i32) = guess();
        assert_eq!(
            p1,
            (negative as u8, small as i8, below as i32),
            "P1: as between numeric types"
        );

        // P2. char to u32, then char to u8.
        let c = '港';
        let p2: (u32, u8) = guess();
        assert_eq!(p2, (c as u32, c as u8), "P2: char casts keep low bits");

        // P3. Size in bytes of a borrowing closure and a moving closure over a u8.
        let x = 5u8;
        let borrowing = || x + 1;
        let moving = move || x + 1;
        let p3: (usize, usize) = guess();
        assert_eq!(
            p3,
            (size_of_val(&borrowing), size_of_val(&moving)),
            "P3: what a closure stores"
        );
        assert_eq!((borrowing(), moving()), (6, 6));

        // P4. A move closure copies a Copy value. What does it see later?
        let mut threshold = 5;
        let check = move |v: i32| v >= threshold;
        threshold = 100;
        let p4: (bool, i32) = guess();
        assert_eq!(
            p4,
            (check(50), threshold),
            "P4: the closure has its own copy"
        );
    }
}
