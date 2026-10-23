//! LESSON 01 — Unicode scalar operations and encoding
//! Run: cargo test --test chapter17 l01_
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider: How many bytes/scalars are in "e\u{301}" and "🦀"? Can char
//! represent a surrogate or an entire grapheme cluster?
//!
//! IMPLEMENT the documented tasks, then keep the tests and add an edge case.
//! EXPLAIN afterward: Why can case conversion yield multiple scalars? Why is reversing
//! logical scalar order unsuitable as a text-direction algorithm?
//!
//! COMPILER / MANUAL LAB (use examples/ch17_scratch.rs):
//! Compare "é" and "e\u{301}" byte/scalar counts and equality. Print mixed Latin and Hebrew
//! text inside delimiters; record logical char_indices separately from rendering. Decode
//! [0xE9] with from_utf8 and contrast Latin-1 with [0xC3,0xA9]. No normalization or
//! bidirectional layout is implemented by chars().
//!
//! DIGIT LAB: predict and check 'f'.to_digit(16), '7'.to_digit(2),
//! char::from_digit(15, 16), and char::from_digit(16, 16). Explain why
//! to_digit is not a parser for every Unicode numeric character. Compare
//! char::is_numeric with is_ascii_digit on Arabic-Indic digits.
//!
//! TRANSFER: invent a new edge case, predict it, then add a check. A lesson is
//! complete only when its tests, predictions, lab, and explanation are done.

/// A. Return (UTF-8 bytes,Unicode scalar count,scalar byte positions).
fn measures(text: &str) -> (usize, usize, Vec<(usize, char)>) {
    todo!("Task 1: implement the documented contract")
}
/// B. Convert an integer to a scalar, then uppercase all resulting scalars.
/// Invalid scalars yield None; uppercase expansion such as ß -> SS is required.
fn uppercase_codepoint(code: u32) -> Option<String> {
    todo!("Task 2: implement the documented contract")
}
/// C. Return (alphabetic,whitespace,ASCII decimal digit value).
/// Unicode digits outside ASCII must have None in the last component.
fn classify(ch: char) -> (bool, bool, Option<u32>) {
    todo!("Task 3: implement the documented contract")
}
#[cfg(test)]
mod tests {
    use super::*;
    use practice::guess;
    #[test]
    fn a_byte_offsets_differ_from_scalar_positions() {
        assert_eq!(
            measures("aé🦀"),
            (7, 3, vec![(0, 'a'), (1, 'é'), (3, '🦀')])
        );
        assert_eq!(measures("e\u{301}").0, 3);
        assert_eq!(measures("e\u{301}").1, 2);
        assert_eq!(measures(""), (0, 0, vec![]));
    }
    #[test]
    fn b_codepoints_validate_and_case_mapping_can_expand() {
        assert_eq!(uppercase_codepoint('ß' as u32), Some("SS".into()));
        assert_eq!(uppercase_codepoint('é' as u32), Some("É".into()));
        assert_eq!(uppercase_codepoint(0xD800), None);
        assert_eq!(uppercase_codepoint(0x110000), None);
        assert_eq!(uppercase_codepoint('7' as u32), Some("7".into()));
        assert_eq!(
            uppercase_codepoint(0x149),
            Some("\u{2BC}N".into()),
            "ŉ -> ʼN"
        );
    }
    #[test]
    fn c_classification_distinguishes_ascii_digits() {
        assert_eq!(classify('é'), (true, false, None));
        assert_eq!(classify('\u{2003}'), (false, true, None));
        assert_eq!(classify('7'), (false, false, Some(7)));
        assert_eq!(classify('٧'), (false, false, None));
        assert_eq!(classify('a'), (true, false, None), "not a hex digit here");
        assert_eq!(classify('F'), (true, false, None));
        assert_eq!(classify('\n'), (false, true, None));
        assert_eq!(classify('0'), (false, false, Some(0)));
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    /// A wrong prediction is useful: read the failure, then explain it in NOTES.md.
    #[test]
    fn p_predictions() {
        // P1. UTF-8 byte lengths of three strings that each look like one character.
        let p1: (usize, usize, usize) = guess();
        assert_eq!(
            p1,
            ("é".len(), "e\u{301}".len(), "🦀".len()),
            "P1: bytes per string"
        );

        // P2. A surrogate code point, and the digit 15 in base 16.
        let p2: (Option<char>, Option<char>) = guess();
        assert_eq!(
            p2,
            (char::from_u32(0xD800), char::from_digit(15, 16)),
            "P2: char conversions"
        );

        // P3. Lowercasing 'İ' (capital I with dot above): how many scalars come out?
        let p3: usize = guess();
        assert_eq!(
            p3,
            'İ'.to_lowercase().count(),
            "P3: case mapping can expand"
        );

        // P4. Distance from 'A' to 'Z' as code points, and the UTF-8 length of 'é'.
        let p4: (u32, usize) = guess();
        assert_eq!(
            p4,
            ('Z' as u32 - 'A' as u32, 'é'.len_utf8()),
            "P4: char as u32"
        );
    }
}
