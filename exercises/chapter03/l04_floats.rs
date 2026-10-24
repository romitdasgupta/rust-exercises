//! LESSON 04 — Floating-point values and explicit conversions
//! Run: cargo test --test chapter03 l04_
//!
//! PREDICT: first answer `p_predictions` at the bottom (replace each `guess()`
//! with your answer). Then consider: Are `0.1_f64 + 0.2 == 0.3`, `f64::NAN ==
//! f64::NAN`, and `0.0_f64 == -0.0` true? Do equal zeros have equal
//! reciprocals? What does `(-1.01_f64).floor()` produce? Compare f32 and f64
//! precision.
//!
//! EXPLAIN: Why do the area tests use a tolerance? Does casting an f32 to f64
//! recover precision already lost when that f32 was created?
//!
//! COMPILER LAB: try `let n = 3_i16; let x: f64 = n;`. Repair it explicitly.
//! Try `(2.0).sqrt()` and resolve any method-lookup ambiguity. Inspect the
//! `is_sign_negative` method on positive and negative zero.

/// A. Return (is_finite, is_infinite, is_nan), using classification methods.
fn classify(value: f64) -> (bool, bool, bool) {
    todo!("A: classify IEEE values without comparing NaN for equality")
}

/// B. Area of a circle. Radius is finite and between 0 and 1_000.
/// Use the standard-library f64 PI constant; do not hard-code its digits.
fn circle_area(radius: f64) -> f64 {
    todo!("B: use a floating-point constant and arithmetic")
}

/// C. Compute, in order, the reciprocal of positive zero and negative zero.
/// Calculate these with division rather than returning infinity constants.
fn zero_reciprocals() -> (f64, f64) {
    todo!("C: observe the sign of IEEE zero")
}

/// D. Convert both operands to f64 BEFORE multiplying, and return the product.
fn scale_reading(raw: i16, scale: f32) -> f64 {
    todo!("D: Rust does not insert numeric conversions")
}

#[cfg(test)]
mod tests {
    use super::*;
    use practice::guess;

    #[test]
    fn a_special_values_need_classification() {
        assert_eq!(classify(12.5), (true, false, false));
        assert_eq!(classify(f64::INFINITY), (false, true, false));
        assert_eq!(classify(f64::NEG_INFINITY), (false, true, false));
        assert_eq!(classify(f64::NAN), (false, false, true));
        assert_eq!(classify(-0.0), (true, false, false));
    }

    #[test]
    #[allow(clippy::approx_constant)] // The literal is the point of this test.
    fn b_circle_area_uses_double_precision() {
        for (radius, expected) in [
            (0.0, 0.0),
            (1.0, 3.141592653589793),
            (2.0, 12.566370614359172),
        ] {
            assert!((circle_area(radius) - expected).abs() < 1e-12);
        }
    }

    #[test]
    fn c_equal_zeros_have_opposite_reciprocals() {
        let (positive, negative) = zero_reciprocals();
        assert_eq!(positive, f64::INFINITY);
        assert_eq!(negative, f64::NEG_INFINITY);
    }

    #[test]
    fn d_convert_before_multiplication() {
        assert_eq!(scale_reading(20, 0.125), 2.5);
        assert_eq!(scale_reading(-3, 0.5), -1.5);
        // The f64 product preserves information that f32 multiplication loses.
        assert_eq!(scale_reading(32_767, 1.000_000_1), 32_767.003_906_130_79);
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    #[test]
    fn p_predictions() {
        // P1. The same sum in f64 and in f32. Rounding differs by type.
        let p1: (bool, bool) = guess();
        assert_eq!(
            p1,
            (0.1_f64 + 0.2 == 0.3, 0.1_f32 + 0.2 == 0.3),
            "P1: float equality depends on rounding"
        );

        // P2. NaN equality, signed-zero equality, and max with a NaN operand.
        let nan = f64::NAN;
        let copy = nan;
        let p2: (bool, bool, f64) = guess();
        assert_eq!(
            p2,
            (nan == copy, 0.0_f64 == -0.0, nan.max(1.0)),
            "P2: NaN and signed zero"
        );

        // P3. Four ways to get an integer-valued float.
        let p3: (f64, f64, f64, f64) = guess();
        assert_eq!(
            p3,
            (
                (-1.01_f64).floor(),
                (-1.5_f64).round(),
                2.5_f64.round(),
                (-1.7_f64).trunc()
            ),
            "P3: floor, round, trunc"
        );

        // P4. Precision: an i32 through f32 and back; an f32 widened to f64.
        let big = 16_777_217_i32;
        let tenth = 0.1_f32;
        let p4: (i32, bool) = guess();
        assert_eq!(
            p4,
            (big as f32 as i32, f64::from(tenth) == 0.1_f64),
            "P4: f32 has a 24-bit significand"
        );
    }
}
