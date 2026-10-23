//! LESSON 02 — Integer representations, literals, and casts
//! Run: cargo test --test chapter03 l02_
//!
//! PREDICT: first answer `p_predictions` at the bottom (replace each `guess()`
//! with your answer). Then consider: Give the ranges of `u8` and `i8`. What do
//! `300_u16 as u8` and `255_u8 as i8` produce? How many bytes does a `u64`
//! occupy? What determines the width of `usize`, and why should an array index
//! use that type?
//!
//! EXPLAIN: Distinguish a numeric value from its bit pattern. When can `as`
//! discard information? Why is truncation appropriate for task B specifically?
//!
//! COMPILER LAB: assign a `u16` variable to a `u32` variable without a cast.
//! Does Rust insert that widening conversion? Now try a literal with no suffix.
//! What difference does context make? Try adding underscores to a base literal.

/// A. Express decimal 170 in decimal, hexadecimal, octal, and binary order.
/// Write four literals using the appropriate prefixes; don't repeat decimal 170.
fn same_number_four_bases() -> (u16, u16, u16, u16) {
    todo!("A: read and write integer literal bases")
}

/// B. Keep only the lowest eight bits. This is deliberate truncation,
/// not a checked numeric conversion. Use a cast.
fn low_byte(value: u32) -> u8 {
    todo!("B: cast to keep only the lowest eight bits")
}

/// C. Interpret the same eight bits as a signed integer using a cast.
fn signed_view(value: u8) -> i8 {
    todo!("C: reinterpret the same bits as a signed integer")
}

/// D. Count the one bits with an integer method, not a hand-written loop.
fn set_bits(value: u16) -> u32 {
    todo!("D: count one bits with an integer method")
}

#[cfg(test)]
mod tests {
    use super::*;
    use practice::guess;

    #[test]
    fn a_bases_change_notation_not_value() {
        assert_eq!(same_number_four_bases(), (170, 170, 170, 170));
    }

    #[test]
    fn b_narrow_cast_keeps_low_bits() {
        assert_eq!(low_byte(0x1234_abcd), 0xcd);
        assert_eq!(low_byte(256), 0);
        assert_eq!(low_byte(u32::MAX), u8::MAX);
        assert_eq!(low_byte(7), 7);
    }

    #[test]
    fn c_signed_view_has_a_different_range() {
        assert_eq!(signed_view(0), 0);
        assert_eq!(signed_view(127), 127);
        assert_eq!(signed_view(128), -128);
        assert_eq!(signed_view(255), -1);
    }

    #[test]
    fn d_count_bits_instead_of_digits() {
        assert_eq!(set_bits(0), 0);
        assert_eq!(set_bits(u16::MAX), 16);
        assert_eq!(set_bits(0xa00f), 6);
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    #[test]
    fn p_predictions() {
        // P1. Narrowing casts keep the low bits. What are these?
        let wide = 300_u16;
        let all_ones = 255_u8;
        let p1: (u8, i8) = guess();
        assert_eq!(
            p1,
            (wide as u8, all_ones as i8),
            "P1: truncating and reinterpreting casts"
        );

        // P2. Casting a negative i8 to the same width, and to wider types.
        let minus_one = -1_i8;
        let p2: (u8, i32, u32) = guess();
        assert_eq!(
            p2,
            (minus_one as u8, minus_one as i32, minus_one as u32),
            "P2: widening a signed value sign-extends it"
        );

        // P3. Sizes in bytes, and the width of usize on a 64-bit target.
        let p3: (usize, usize, u32) = guess();
        assert_eq!(
            p3,
            (size_of::<u64>(), size_of::<i128>(), usize::BITS),
            "P3: integer widths"
        );
    }
}
