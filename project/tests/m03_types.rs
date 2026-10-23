//! MILESTONE m03 — Numbers with an explicit overflow policy (Chapter 3)
//! Run: cargo test --test m03_types
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider: what should a log analyzer do
//! with `latency_ms=99999999999999999999`: crash, wrap, clamp, or report it?
//! IMPLEMENT `src/num.rs`: `parse_u64` and `mean`. Keep these tests and add an edge case.
//! EXPLAIN afterward: why checked arithmetic instead of `wrapping_*` or plain `*`,
//! and why the byte index of the first invalid byte is always also its character index.

mod common;

use common::guess;
use loglens::num::{NumError, Overflow, mean, parse_u64};

#[test]
fn a_parses_plain_digits() {
    assert_eq!(parse_u64("0", Overflow::Reject), Ok(0));
    assert_eq!(parse_u64("42", Overflow::Reject), Ok(42));
    assert_eq!(parse_u64("0007", Overflow::Saturate), Ok(7));
    assert_eq!(
        parse_u64("18446744073709551615", Overflow::Reject),
        Ok(u64::MAX)
    );
}

#[test]
fn a_rejects_what_is_not_a_plain_number() {
    use NumError::{Empty, InvalidDigit};
    let cases = [
        ("", Err(Empty)),
        ("+1", Err(InvalidDigit { index: 0 })),
        ("-1", Err(InvalidDigit { index: 0 })),
        (" 1", Err(InvalidDigit { index: 0 })),
        ("1 ", Err(InvalidDigit { index: 1 })),
        ("1_000", Err(InvalidDigit { index: 1 })),
        ("42ms", Err(InvalidDigit { index: 2 })),
        ("4.5", Err(InvalidDigit { index: 1 })),
        // ARABIC-INDIC DIGIT THREE is a digit to Unicode, not to this format.
        ("٣", Err(InvalidDigit { index: 0 })),
        // The first non-digit is reported, even when it is not ASCII.
        ("1éx", Err(InvalidDigit { index: 1 })),
    ];
    for (text, expected) in cases {
        assert_eq!(
            parse_u64(text, Overflow::Reject),
            expected,
            "input {text:?}"
        );
        assert_eq!(
            parse_u64(text, Overflow::Saturate),
            expected,
            "input {text:?}"
        );
    }
}

#[test]
fn a_overflow_policy_is_explicit() {
    let just_over = "18446744073709551616";
    assert_eq!(
        parse_u64(just_over, Overflow::Reject),
        Err(NumError::Overflow)
    );
    assert_eq!(parse_u64(just_over, Overflow::Saturate), Ok(u64::MAX));

    let huge = "9".repeat(40);
    assert_eq!(parse_u64(&huge, Overflow::Reject), Err(NumError::Overflow));
    assert_eq!(parse_u64(&huge, Overflow::Saturate), Ok(u64::MAX));

    // Overflow does not hide a later invalid byte, under either policy.
    let bad = "99999999999999999999x";
    for policy in [Overflow::Reject, Overflow::Saturate] {
        assert_eq!(
            parse_u64(bad, policy),
            Err(NumError::InvalidDigit { index: 20 })
        );
    }
}

#[test]
fn b_mean() {
    assert_eq!(mean(0, 0), None);
    assert_eq!(mean(7, 0), None);
    assert_eq!(mean(10, 4), Some(2.5));
    assert_eq!(mean(0, 3), Some(0.0));
    assert_eq!(mean(u64::MAX, 1), Some(u64::MAX as f64));
}

/// P. Predict BEFORE running: replace each `guess()` with your answer.
/// A wrong prediction is useful: read the failure, then explain it in your notes.
#[test]
fn p_predictions() {
    // P1. Wrapping arithmetic at the top of u8.
    let p1: u8 = guess();
    assert_eq!(p1, 250u8.wrapping_add(10), "P1: wrapping_add is modulo 256");

    // P2. One step past u64::MAX / 10, multiplied by 10, checked.
    let p2: Option<u64> = guess();
    assert_eq!(
        p2,
        (u64::MAX / 10 + 1).checked_mul(10),
        "P2: checked_mul reports overflow"
    );

    // P3. Casting an integer to a narrower integer type with `as`.
    let p3: u8 = guess();
    assert_eq!(p3, 300u16 as u8, "P3: `as` keeps the low bits");

    // P4. Does 2^53 + 1 survive a trip through f64?
    let n = (1u64 << 53) + 1;
    let p4: bool = guess();
    assert_eq!(p4, n as f64 as u64 == n, "P4: f64 has a 53-bit significand");
}
