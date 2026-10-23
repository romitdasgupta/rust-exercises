//! LESSON 08 — UTF-8, borrowed text views, and delayed allocation
//! Run: cargo test --test chapter17 l08_
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider: Does into_bytes copy a String allocation? What information
//! does FromUtf8Error retain? Which Cow variant does from_utf8_lossy return for valid
//! input?
//!
//! IMPLEMENT the documented tasks, then keep the tests and add an edge case.
//! EXPLAIN afterward: Why should OS strings and paths not be forced through
//! to_str().unwrap()? Which conversions guarantee UTF-8 and which preserve arbitrary bytes?
//!
//! COMPILER / MANUAL LAB (use examples/ch17_scratch.rs):
//! Borrow String as str, [u8], OsStr, and Path. Build text with collect<char>(), extend,
//! and join, then split ownership with into_bytes. Try from_utf8 and from_utf8_lossy on
//! [0x66,0x80,0x6f]; record exact errors/replacement scalars. Paths are covered further in
//! chapter 18.
//!
//! TRANSFER: invent a new edge case, predict it, then add a check. A lesson is
//! complete only when its tests, predictions, lab, and explanation are done.

use std::borrow::Cow;
/// A. Strictly convert owned bytes to String; on invalid UTF-8 return original bytes.
fn decode(bytes: Vec<u8>) -> Result<String, Vec<u8>> {
    todo!("Task 1: implement the documented contract")
}
/// B. Repair invalid UTF-8 with U+FFFD. Borrow bytes as text when valid.
fn display_bytes(bytes: &[u8]) -> Cow<'_, str> {
    todo!("Task 2: implement the documented contract")
}
/// C. Trim words and join nonempty words with one ASCII space.
/// Treat strings as a collection of borrowed slices, allocate no intermediate owned Strings.
fn joined(words: &[&str]) -> String {
    todo!("Task 3: implement the documented contract")
}
/// D. Return the same allocation as owned bytes; do not clone.
fn into_packet(text: String) -> Vec<u8> {
    todo!("Task 4: implement the documented contract")
}
#[cfg(test)]
mod tests {
    use super::*;
    use practice::{alloc::count, guess};
    #[test]
    fn a_strict_conversion_preserves_invalid_input() {
        let bytes = "é".as_bytes().to_vec();
        let ptr = bytes.as_ptr();
        let text = decode(bytes).unwrap();
        assert_eq!(text, "é");
        assert_eq!(text.as_ptr(), ptr);
        assert_eq!(decode(vec![b'a', 0xff]), Err(vec![b'a', 0xff]));
        let bad = vec![b'x', 0xe9, b'y'];
        let ptr = bad.as_ptr();
        let back = decode(bad).unwrap_err();
        assert_eq!(back.as_ptr(), ptr, "the original buffer comes back");
        assert_eq!(decode(vec![]), Ok(String::new()));
    }
    #[test]
    fn b_lossy_conversion_allocates_only_when_repairing() {
        assert!(matches!(display_bytes(b"ok"), Cow::Borrowed(_)));
        assert_eq!(display_bytes(&[b'a', 0xff, b'b']), "a�b");
        assert!(matches!(display_bytes(&[0xff]), Cow::Owned(_)));
    }
    #[test]
    fn c_join_omits_blanks_without_modifying_words() {
        assert_eq!(joined(&[" a ", "", " \t", "é"]), "a é");
        assert_eq!(joined(&[]), "");
        assert_eq!(joined(&["\u{2003}x\u{2003}", "y z"]), "x y z");
    }
    #[test]
    fn c_join_allocates_no_string_per_word() {
        let words: Vec<String> = (0..100).map(|i| format!(" w{i} ")).collect();
        let words: Vec<&str> = words.iter().map(String::as_str).collect();
        let (out, allocations) = count(|| joined(&words));
        assert!(out.starts_with("w0 w1 w2"));
        assert!(
            allocations <= 16,
            "100 words cost {allocations} allocations"
        );
    }
    #[test]
    fn d_owned_utf8_can_become_bytes_without_copy() {
        let s = String::from("🦀");
        let ptr = s.as_ptr();
        let bytes = into_packet(s);
        assert_eq!(bytes, "🦀".as_bytes());
        assert_eq!(bytes.as_ptr(), ptr);
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    #[test]
    fn p_predictions() {
        let bytes = vec![0x66, 0x80, 0x6f];
        // P1. How many bytes are valid before the error?
        let p1: usize = guess();
        assert_eq!(
            p1,
            String::from_utf8(bytes.clone())
                .unwrap_err()
                .utf8_error()
                .valid_up_to(),
            "P1: valid_up_to"
        );

        // P2. After lossy repair: (scalar count, byte length).
        let repaired = String::from_utf8_lossy(&bytes);
        let p2: (usize, usize) = guess();
        assert_eq!(
            p2,
            (repaired.chars().count(), repaired.len()),
            "P2: U+FFFD is 3 bytes"
        );

        // P3. Allocations made by String::into_bytes.
        let s = String::from("payload");
        let (raw, allocations) = count(|| s.into_bytes());
        let p3: usize = guess();
        assert_eq!(p3, allocations, "P3: into_bytes");
        assert_eq!(raw.len(), 7);

        // P4. A borrowed Cow after to_mut(): which variant is it now?
        let mut text: Cow<'_, str> = Cow::Borrowed("draft");
        text.to_mut().push('!');
        let p4: bool = guess(); // true if Owned
        assert_eq!(p4, matches!(text, Cow::Owned(_)), "P4: clone on write");
    }
}
