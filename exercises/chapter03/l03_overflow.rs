//! LESSON 03 — Choosing an overflow policy
//! Run: cargo test --test chapter03 l03_
//! Then: cargo test --release --test chapter03 l03_
//!
//! PREDICT: For 250_u8 plus 10, write the result of wrapping, checked,
//! saturating, and overflowing addition. What does strict addition do?
//! What does ordinary addition do by default in debug and release builds?
//!
//! EXPLAIN: Pick policies for image brightness, a cyclic counter, and a buffer
//! size that must reject overflow. Does a checked shift detect bits shifted out,
//! or an invalid shift distance? Why is casting AFTER addition too late?
//!
//! COMPILER LAB: put `std::hint::black_box(250_u8) + 10` in the scratch program
//! and print it. Compare normal and `--release` runs. black_box keeps this a
//! runtime operation for the experiment. Repair it with an explicit policy.

/// A. Return (wrapping, checked, saturating, overflowing) addition results.
/// Each field must use its corresponding method, with profile-independent behavior.
fn addition_policies(a: u8, b: u8) -> (u8, Option<u8>, u8, (u8, bool)) {
    (a.wrapping_add(b), a.checked_add(b), a.saturating_add(b), a.overflowing_add(b))
}

/// B. Multiply using the strict method: overflow must panic in EVERY profile.
fn strict_product(a: u16, b: u16) -> u16 {
    a.strict_mul(b)
}

/// C. Average any two i32 values without overflow, rounding toward zero.
/// Use a wider intermediate or the built-in midpoint method.
fn midpoint(a: i32, b: i32) -> i32 {
    a.midpoint(b)
}

/// D. Return (checked shift-left, wrapping shift-left).
/// The policy applies to the DISTANCE; this is not checked multiplication.
fn left_shift_policies(value: u8, distance: u32) -> (Option<u8>, u8) {
    (value.checked_shl(distance), value.wrapping_shl(distance))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_safe_and_overflowing_addition() {
        assert_eq!(addition_policies(10, 20), (30, Some(30), 30, (30, false)));
        assert_eq!(addition_policies(250, 10), (4, None, 255, (4, true)));
        assert_eq!(
            addition_policies(255, 0),
            (255, Some(255), 255, (255, false))
        );
    }

    #[test]
    fn b_strict_product_still_multiplies_safe_inputs() {
        assert_eq!(strict_product(200, 300), 60_000);
        assert_eq!(strict_product(0, u16::MAX), 0);
    }

    #[test]
    #[should_panic(expected = "attempt to multiply with overflow")]
    fn b_strict_product_panics_on_overflow() {
        strict_product(500, 500);
    }

    #[test]
    fn c_midpoint_handles_both_extremes() {
        assert_eq!(midpoint(4, 10), 7);
        assert_eq!(midpoint(7, i32::MAX), 1_073_741_827);
        assert_eq!(midpoint(i32::MIN, i32::MAX), 0);
        assert_eq!(midpoint(i32::MIN, i32::MIN), i32::MIN);
        assert_eq!(midpoint(-4, -1), -2);
    }

    #[test]
    fn d_shift_checks_the_distance() {
        assert_eq!(left_shift_policies(3, 1), (Some(6), 6));
        assert_eq!(left_shift_policies(128, 1), (Some(0), 0));
        assert_eq!(left_shift_policies(3, 8), (None, 3));
        assert_eq!(left_shift_policies(3, 9), (None, 6));
        assert_eq!(left_shift_policies(0, 8), (None, 0));
    }
}
