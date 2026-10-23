//! Milestone m03 (Chapter 3): numeric fields with an explicit overflow policy.
//!
//! Log fields such as `latency_ms=42` arrive as text. Production data is
//! messy: a counter can exceed `u64::MAX`, a value can be `42ms` or `-1`. Decide
//! what happens in each case instead of letting it wrap or panic.

use std::fmt;

/// What to do when a decimal number does not fit in a `u64`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Overflow {
    /// Return [`NumError::Overflow`].
    Reject,
    /// Clamp the result to `u64::MAX`.
    Saturate,
}

/// Why a numeric field could not be read.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NumError {
    /// The text was empty.
    Empty,
    /// The byte at `index` (a byte offset into the text) is not an ASCII digit.
    InvalidDigit {
        /// Byte offset of the first offending byte.
        index: usize,
    },
    /// The number does not fit in a `u64` and the policy was [`Overflow::Reject`].
    Overflow,
}

// Provided: trait implementations are practiced from m07 on.
impl fmt::Display for NumError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NumError::Empty => f.write_str("empty number"),
            NumError::InvalidDigit { index } => write!(f, "invalid digit at byte {index}"),
            NumError::Overflow => f.write_str("number too large"),
        }
    }
}

impl std::error::Error for NumError {}

/// m03 A. Parse an unsigned decimal number.
///
/// - Accept only ASCII digits `0`–`9`. Leading zeros are fine. No sign, no
///   whitespace, no `_` separators, no non-ASCII digits.
/// - `""` is [`NumError::Empty`].
/// - The first byte that is not a digit is reported as
///   [`NumError::InvalidDigit`] with its byte index, even when the digits
///   before it already overflowed: `"99999999999999999999x"` reports index 20.
/// - A value above `u64::MAX` is [`NumError::Overflow`] under
///   [`Overflow::Reject`] and `u64::MAX` under [`Overflow::Saturate`].
///
/// Do not call `str::parse`: accumulate the value yourself with the checked
/// integer methods, so the overflow policy is visible in your code.
///
/// ```
/// use loglens::num::{Overflow, NumError, parse_u64};
///
/// assert_eq!(parse_u64("042", Overflow::Reject), Ok(42));
/// assert_eq!(parse_u64("4x", Overflow::Reject), Err(NumError::InvalidDigit { index: 1 }));
/// assert_eq!(parse_u64("18446744073709551616", Overflow::Saturate), Ok(u64::MAX));
/// ```
pub fn parse_u64(text: &str, policy: Overflow) -> Result<u64, NumError> {
    todo!("m03 A: parse digits with checked arithmetic")
}

/// m03 B. The arithmetic mean of `count` values whose sum is `total`, or
/// `None` when `count` is zero.
///
/// Convert with `as f64` and divide in floating point. (Explain in your
/// notes which `u64` values `as f64` cannot represent exactly.)
pub fn mean(total: u64, count: u64) -> Option<f64> {
    todo!("m03 B: integer-to-float mean")
}
