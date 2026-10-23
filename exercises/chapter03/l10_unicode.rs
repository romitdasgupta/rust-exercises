//! LESSON 10 — UTF-8 bytes, scalar values, and string slices
//! Run: cargo test --test chapter03 l10_
//!
//! PREDICT: first answer `p_predictions` at the bottom (replace each `guess()`
//! with your answer). Then consider: Give byte lengths and `chars().count()`
//! results for "cat", "ñ🦀", and "e\u{301}". How many bytes store one char? Is
//! every u32 a valid char? Which byte offsets in "ñ🦀" are valid boundaries?
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
    use practice::{alloc::count, guess};

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

    #[test]
    fn a_d_measuring_and_splitting_do_not_allocate() {
        let text = String::from("naïve🦀");
        let (metrics, allocations) = count(|| text_metrics(&text));
        assert_eq!(metrics, (10, 6));
        assert_eq!(allocations, 0, "count scalars without collecting them");
        let (pieces, allocations) = count(|| split_at_byte(&text, 6));
        assert_eq!(pieces, Some(("naïve", "🦀")));
        assert_eq!(allocations, 0, "return borrowed pieces");
        assert_eq!(pieces.unwrap().1.as_ptr(), text[6..].as_ptr());
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    #[test]
    fn p_predictions() {
        // P1. UTF-8 bytes and Unicode scalars in a German greeting.
        let greeting = "Grüße";
        let p1: (usize, usize) = guess();
        assert_eq!(
            p1,
            (greeting.len(), greeting.chars().count()),
            "P1: bytes versus scalars"
        );

        // P2. Is each u32 a valid char? (0xD800 is a UTF-16 surrogate.)
        let p2: (bool, bool) = guess();
        assert_eq!(
            p2,
            (
                char::from_u32(0xD800).is_some(),
                char::from_u32(0x1F980).is_some()
            ),
            "P2: not every u32 is a char"
        );

        // P3. Every byte offset from 0 to 6 that is a char boundary in "ñ🦀".
        let text = "ñ🦀";
        let boundaries: Vec<usize> = (0..=text.len())
            .filter(|&i| text.is_char_boundary(i))
            .collect();
        let p3: Vec<usize> = guess();
        assert_eq!(p3, boundaries, "P3: char boundaries");

        // P4. "e" + combining acute accent, versus the precomposed "é".
        let p4: bool = guess();
        assert_eq!(p4, "e\u{301}" == "\u{e9}", "P4: no normalization in ==");
    }
}
