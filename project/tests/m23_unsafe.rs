//! OPTIONAL MILESTONE m23 — An unsafe fast path behind a safe API (Chapters 23–24)
//! Run: cargo test --test m23_unsafe
//! Miri: cargo +nightly miri test --manifest-path project/Cargo.toml --test m23_unsafe
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider: which reads would be out of bounds at
//! the end of the slice, and what makes an unaligned read legal?
//! IMPLEMENT `src/fast.rs`: `find_byte`, `split_ascii`. Write a `// SAFETY:` comment for
//! every `unsafe` block before running anything.
//! REFACTOR (optional): use `find_byte` in `src/scan.rs` to find `=`, `"`, and `\`.
//! Every earlier milestone must stay green: `cargo test --tests`.
//! EXPLAIN afterward: your safety argument, what Miri checks, and what it cannot prove.

mod common;

use common::{borrows_from, guess};
use loglens::fast::{find_byte, split_ascii};

fn naive(haystack: &[u8], needle: u8) -> Option<usize> {
    haystack.iter().position(|&b| b == needle)
}

#[test]
fn a_find_byte_matches_a_plain_search() {
    // Every length up to 40, every alignment of the slice start, and the needle
    // absent or at every position, among bytes that differ from it in one bit.
    let backing: Vec<u8> = (0..64).collect();
    for needle in [b'=', 0x00, 0x7f, 0x80, 0xff] {
        for offset in 0..8 {
            for len in 0..=40 {
                let mut hay: Vec<u8> = backing[..offset + len]
                    .iter()
                    .map(|&i| needle ^ (1 << (i % 8)))
                    .collect();
                let slice = &hay[offset..];
                assert_eq!(find_byte(slice, needle), None);
                for pos in 0..len {
                    hay[offset + pos] = needle;
                    let slice = &hay[offset..];
                    assert_eq!(
                        find_byte(slice, needle),
                        naive(slice, needle),
                        "needle {needle:#x}, offset {offset}, len {len}, pos {pos}"
                    );
                    hay[offset + pos] = needle ^ 1;
                }
            }
        }
    }
}

#[test]
fn a_find_byte_returns_the_first_match() {
    let hay = b"key=value=more==";
    assert_eq!(find_byte(hay, b'='), Some(3));
    assert_eq!(find_byte(&hay[4..], b'='), Some(5));
    assert_eq!(find_byte(b"", b'='), None);
    let long = [b"abcdefgh".repeat(10), b"=".to_vec()].concat();
    assert_eq!(find_byte(&long, b'='), Some(80));
    let high: Vec<u8> = (0x80..=0xff).collect();
    assert_eq!(find_byte(&high, 0x00), None);
    assert_eq!(find_byte(&high, 0xff), Some(127));
}

#[test]
fn b_split_ascii() {
    assert_eq!(split_ascii("key=value=x", b'='), Some(("key", "value=x")));
    assert_eq!(split_ascii("é=ü", b'='), Some(("é", "ü")));
    assert_eq!(split_ascii("=", b'='), Some(("", "")));
    assert_eq!(split_ascii("ab=", b'='), Some(("ab", "")));
    assert_eq!(split_ascii("no equals", b'='), None);
    assert_eq!(split_ascii("", b' '), None);
    assert_eq!(split_ascii("🦀 crab", b' '), Some(("🦀", "crab")));
}

#[test]
fn b_split_ascii_borrows() {
    let s = String::from("ключ=значение");
    let (k, v) = split_ascii(&s, b'=').unwrap();
    assert!(borrows_from(&s, k) && borrows_from(&s, v));
    assert_eq!(v.as_ptr(), s[9..].as_ptr());
}

#[test]
#[should_panic(expected = "ASCII")]
fn b_split_ascii_rejects_non_ascii_bytes() {
    // 0xA9 is the second byte of "é": splitting there would break UTF-8.
    split_ascii("é", 0xa9);
}

/// P. Predict BEFORE running: replace each `guess()` with your answer.
/// The sizes assume a 64-bit target.
#[test]
fn p_predictions() {
    // P1. The size of Option<&u8>.
    let p1: usize = guess();
    assert_eq!(
        p1,
        size_of::<Option<&u8>>(),
        "P1: references are never null"
    );

    // P2. Eight bytes read as a little-endian u64.
    let p2: u64 = guess();
    assert_eq!(
        p2,
        u64::from_le_bytes(*b"\x01\0\0\0\0\0\0\0"),
        "P2: the first byte is the lowest"
    );

    // P3. Do all bytes of "é" have the high bit set?
    let p3: bool = guess();
    assert_eq!(
        p3,
        "é".as_bytes().iter().all(|b| b & 0x80 != 0),
        "P3: no byte of a multi-byte character is ASCII"
    );

    // P4. A byte repeated in every lane of a u64, in hexadecimal.
    let p4: u64 = guess();
    assert_eq!(
        p4,
        0x0101_0101_0101_0101u64 * 0x3d,
        "P4: the broadcast pattern"
    );
}
