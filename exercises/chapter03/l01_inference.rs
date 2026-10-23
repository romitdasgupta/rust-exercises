//! LESSON 01 — Inference and numeric method lookup
//! Run: cargo test --test chapter03 l01_
//!
//! PREDICT, without running: first answer `p_predictions` at the bottom (replace
//! each `guess()` with your answer). Then consider:
//! 1. What types do `let n = 7;` and `let x = 7.0;` default to without other clues?
//! 2. Can a function's return type constrain the type of a local empty vector?
//! 3. Do `(-17_i32).abs()` and `-17_i32.abs()` produce the same value?
//!
//! IMPLEMENT the two functions below. Avoid local type annotations in task A.
//! EXPLAIN: Which signatures must be explicit? How does information from a
//! return type flow back to `Vec::new()` and the loop's integers?
//!
//! COMPILER LAB: try `println!("{}", (-17).abs());` in the scratch program.
//! Why doesn't the usual integer default resolve this method call? Repair it
//! using a suffix, then using `i32::abs`. Explain the parentheses.

/// A. Return three packet lengths: header, header + 16, header + 32.
/// Practice inference: use `Vec::new`, a loop over `0..3`, and `push`.
/// Inputs are at most 1_000; overflow is not part of this task.
fn packet_lengths(header: u16) -> Vec<u16> {
    todo!("A: let the return type constrain the vector and arithmetic")
}

/// B. Compute the absolute value of -17 with the `abs` method.
/// Make the literal's type explicit; do not just return the answer.
fn absolute_distance() -> i32 {
    todo!("B: resolve the numeric method's receiver type")
}

#[cfg(test)]
mod tests {
    use super::*;
    use practice::guess;

    #[test]
    fn a_packet_lengths_follow_the_header() {
        assert_eq!(packet_lengths(4), [4_u16, 20, 36]);
        assert_eq!(packet_lengths(0), [0_u16, 16, 32]);
        assert_eq!(packet_lengths(1_000), [1_000_u16, 1_016, 1_032]);
    }

    #[test]
    fn b_abs_applies_to_the_negative_value() {
        assert_eq!(absolute_distance(), 17);
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    /// A wrong prediction is useful: read the failure, then explain it in NOTES.md.
    #[test]
    fn p_predictions() {
        // P1. With no other clues, how many bytes does each default type take?
        let n = 7;
        let x = 7.0;
        let p1: (usize, usize) = guess();
        assert_eq!(
            p1,
            (size_of_val(&n), size_of_val(&x)),
            "P1: default integer and float types"
        );

        // P2. Unary minus versus a method call: what do these evaluate to?
        let p2: (i32, i32) = guess();
        assert_eq!(
            p2,
            (-17_i32.abs(), (-17_i32).abs()),
            "P2: a method call binds tighter than unary minus"
        );

        // P3. `a` has no suffix. Its type is decided by the next line.
        let a = 1;
        let narrow: u8 = a;
        let p3: u32 = guess();
        assert_eq!(
            p3,
            a.leading_zeros(),
            "P3: a later use can fix a literal's type"
        );
    }
}
