//! LESSON 04 — Text iteration, trimming, and casing
//! Run: cargo test --test chapter17 l04_
//!
//! PREDICT before executing: How do lines, split, and split_terminator differ at a final
//! delimiter? Does trim return an owned string?
//!
//! IMPLEMENT the documented tasks, then keep the tests and add an edge case.
//! EXPLAIN afterward: Why is lowercasing not a universal Unicode case-insensitive
//! comparison? Why do chars and bytes answer different questions?
//!
//! COMPILER / MANUAL LAB (use examples/ch17_scratch.rs):
//! Compare lines(), split('\n'), split_terminator('\n'), split_whitespace(), and
//! split_ascii_whitespace() on "a\r\nb\n" and Unicode whitespace. Compare trim,
//! trim_start_matches, and trim_end_matches. Observe lowercase Greek sigma in whole strings
//! versus per-char conversion.
//!
//! TRANSFER: invent a new edge case, predict it, then add a check. A lesson is
//! complete only when its tests, predictions, lab, and explanation are done.

/// A. Return trimmed nonblank borrowed lines. lines() handles LF/CRLF.
fn clean_lines(text: &str) -> Vec<&str> {
    todo!("Task 1: implement the documented contract")
}
/// B. Uppercase whole text, including one-to-many Unicode mappings.
fn shout(text: &str) -> String {
    todo!("Task 2: implement the documented contract")
}
/// C. Trim only ASCII '-' and '_' from both ends, keeping inner occurrences.
fn unframe(text: &str) -> &str {
    todo!("Task 3: implement the documented contract")
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a_lines_and_trim_borrow_valid_utf8() {
        assert_eq!(clean_lines(" a\r\n \r\n\u{2003}é \n"), ["a", "é"]);
        assert!(clean_lines("").is_empty());
    }
    #[test]
    fn b_whole_string_case_conversion_can_expand() {
        assert_eq!(shout("Straße é"), "STRASSE É");
        assert_eq!(shout(""), "");
    }
    #[test]
    fn c_pattern_trimming_preserves_interior_and_spaces() {
        assert_eq!(unframe("__a-b__"), "a-b");
        assert_eq!(unframe(" _a_ "), " _a_ ");
        assert_eq!(unframe("---"), "");
    }
}
