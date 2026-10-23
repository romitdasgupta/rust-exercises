//! Optional milestone m23 (Chapters 23–24): an unsafe fast path behind a
//! safe API.
//!
//! The scanner spends its time looking for single bytes (`=`, `"`, `\`,
//! spaces). [`find_byte`] checks eight bytes per step ("SWAR": SIMD within a
//! register) using unaligned 8-byte loads. Callers never see `unsafe`: the
//! functions are safe, and each `unsafe` block carries a `// SAFETY:`
//! comment proving its preconditions.
//!
//! Check your unsafe code with Miri (a nightly tool that detects undefined
//! behavior such as out-of-bounds reads):
//!
//! ```sh
//! rustup toolchain install nightly --component miri
//! cargo +nightly miri test --manifest-path project/Cargo.toml --test m23_unsafe
//! ```
//!
//! Tests passing is not a proof of soundness; Miri checks only the paths the
//! tests execute. Your written safety argument is part of the milestone.

/// m23 A. The index of the first `needle` in `haystack`; must equal
/// `haystack.iter().position(|&b| b == needle)` for every input.
///
/// Process 8 bytes at a time while at least 8 remain: load them with
/// `ptr.add(i).cast::<u64>().read_unaligned()`, XOR with `needle` repeated in
/// every byte (`0x0101_0101_0101_0101 * needle as u64`), and test for a zero
/// byte with `(x.wrapping_sub(0x0101..01)) & !x & 0x8080..80 != 0`. When a
/// word contains a match, find its exact position with a plain loop. Handle
/// the remaining tail bytes with a plain loop.
pub fn find_byte(haystack: &[u8], needle: u8) -> Option<usize> {
    todo!("m23 A: word-at-a-time search")
}

/// m23 B. Split `s` around the first `byte`: `(before, after)`, neither
/// containing that occurrence. Panics, with a message that contains
/// `ASCII`, if `byte` is not ASCII.
///
/// Use [`find_byte`], then `str::get_unchecked` for both halves (no second
/// boundary check). Explain in the `// SAFETY:` comment why an ASCII byte's
/// position, and the position after it, are always `char` boundaries.
pub fn split_ascii(s: &str, byte: u8) -> Option<(&str, &str)> {
    todo!("m23 B: unchecked split at a proven boundary")
}
