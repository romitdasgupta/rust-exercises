//! LESSON 10 — UTF-8 bytes, scalar values, and string slices
//! Run: cargo test --test chapter03 l10_
//!
//! PREDICT: Give byte lengths and `chars().count()` results for "cat", "ñ🦀",
//! and "e\u{301}". How many bytes store one char? Is every u32 a valid char?
//! Which byte offsets in "ñ🦀" are valid boundaries?
//!
//! EXPLAIN: A char is a Unicode scalar value. Why can one user-perceived
//! character consist of multiple chars? Contrast String metadata, its UTF-8
//! buffer, and an &str view. Do string literals need a runtime heap allocation?
//!
//! COMPILER LAB: try `"ñ🦀"[0]`, then `&"ñ🦀"[..1]`. Distinguish a compile
//! error from a boundary panic. Inspect `char::from_u32(0xd800)` and
//! `char::from_u32(0x1f980)`; explain the Option results.

/// A. Return (UTF-8 byte length, Unicode scalar count), not grapheme count.
fn text_metrics(text: &str) -> (usize, usize) {
    todo!("A: compare byte length with chars().count()")
}

/// B. Return the first Unicode scalar, or None for empty input.
fn first_scalar(text: &str) -> Option<char> {
    todo!("B: step through scalars instead of indexing bytes")
}

/// C. Return (storage size of this char in bytes, its UTF-8 encoding length).
fn scalar_widths(value: char) -> (usize, usize) {
    todo!("C: distinguish the char representation from its UTF-8 encoding")
}

/// D. Safely split at a byte offset. Return None if mid is out of bounds or
/// inside a multi-byte scalar. Return borrowed pieces without allocating.
fn split_at_byte(text: &str, mid: usize) -> Option<(&str, &str)> {
    todo!("D: check the UTF-8 boundary before slicing")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_bytes_and_scalars_count_different_things() {
        assert_eq!(text_metrics("cat"), (3, 3));
        assert_eq!(text_metrics("ñ🦀"), (6, 2));
        assert_eq!(text_metrics("e\u{301}"), (3, 2));
        assert_eq!(text_metrics(""), (0, 0));
    }

    #[test]
    fn b_read_scalars_instead_of_raw_bytes() {
        assert_eq!(first_scalar("🦀rust"), Some('🦀'));
        assert_eq!(first_scalar("ñ"), Some('ñ'));
        assert_eq!(first_scalar("e\u{301}"), Some('e'));
        assert_eq!(first_scalar(""), None);
    }

    #[test]
    fn c_char_storage_is_fixed_but_utf8_is_variable() {
        for (value, utf8_len) in [('a', 1), ('ñ', 2), ('€', 3), ('🦀', 4)] {
            assert_eq!(scalar_widths(value), (4, utf8_len));
        }
    }

    #[test]
    fn d_split_rejects_invalid_byte_offsets() {
        let text = "naïve🦀";
        assert_eq!(split_at_byte(text, 0), Some(("", text)));
        assert_eq!(split_at_byte(text, 2), Some(("na", "ïve🦀")));
        assert_eq!(split_at_byte(text, 3), None);
        assert_eq!(split_at_byte(text, 6), Some(("naïve", "🦀")));
        assert_eq!(split_at_byte(text, 7), None);
        assert_eq!(split_at_byte(text, 10), Some((text, "")));
        assert_eq!(split_at_byte(text, 11), None);
        assert_eq!(split_at_byte(text, usize::MAX), None);
        assert_eq!(split_at_byte("", 0), Some(("", "")));
        assert_eq!(split_at_byte("", 1), None);
    }
}
