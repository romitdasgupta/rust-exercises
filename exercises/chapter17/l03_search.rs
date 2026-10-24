//! LESSON 03 — Search patterns, byte positions, and replacement
//! Run: cargo test --test chapter17 l03_
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider: Does find return a character number or byte offset? Can a
//! predicate closure serve as a string search pattern?
//!
//! IMPLEMENT the documented tasks, then keep the tests and add an edge case.
//! EXPLAIN afterward: How does a literal string pattern differ from a char or character
//! predicate? Why does replacen need an explicit replacement count?
//!
//! COMPILER / MANUAL LAB (use examples/ch17_scratch.rs):
//! On "aé:aé" compare find/rfind, starts_with/ends_with, contains, split_once/rsplit_once,
//! matches/match_indices, and replacen. Test empty patterns separately. Try a char-slice
//! pattern and a predicate matching ASCII punctuation.
//!
//! TRANSFER: invent a new edge case, predict it, then add a check. A lesson is
//! complete only when its tests, predictions, lab, and explanation are done.

/// A. Split around the first separator, excluding it. Return None when absent.
fn key_value(text: &str, separator: char) -> Option<(&str, &str)> {
    todo!("Task 1: implement the documented contract")
}
/// B. Return non-overlapping literal match byte positions. Needle is nonempty.
fn occurrences(text: &str, needle: &str) -> Vec<usize> {
    todo!("Task 2: implement the documented contract")
}
/// C. Replace at most count occurrences of OLD with NEW, preserving the rest.
fn replace_first(text: &str, old: &str, new: &str, count: usize) -> String {
    todo!("Task 3: implement the documented contract")
}
/// D. Find first Unicode whitespace using a predicate pattern.
fn first_space(text: &str) -> Option<usize> {
    todo!("Task 4: implement the documented contract")
}
#[cfg(test)]
mod tests {
    use super::*;
    use practice::guess;
    #[test]
    fn a_split_once_preserves_borrowed_substrings() {
        assert_eq!(key_value("a=1=2", '='), Some(("a", "1=2")));
        assert_eq!(key_value("🦀:é", ':'), Some(("🦀", "é")));
        assert_eq!(key_value("none", '='), None);
        assert_eq!(key_value("=", '='), Some(("", "")));
        assert_eq!(key_value("k→v", '→'), Some(("k", "v")));
    }
    #[test]
    fn b_matches_are_nonoverlapping_byte_offsets() {
        assert_eq!(occurrences("éaaéaa", "aa"), [2, 6]);
        assert_eq!(occurrences("aaaa", "aa"), [0, 2]);
        assert!(occurrences("abc", "z").is_empty());
        assert_eq!(occurrences("🦀🦀", "🦀"), [0, 4]);
    }
    #[test]
    fn c_bounded_replacement_uses_original_match_order() {
        assert_eq!(replace_first("a-a-a", "a", "é", 2), "é-é-a");
        assert_eq!(replace_first("aaa", "a", "x", 0), "aaa");
        assert_eq!(replace_first("ab", "b", "cc", 5), "acc");
        assert_eq!(replace_first("aaaa", "aa", "a", 1), "aaa");
    }
    #[test]
    fn d_predicate_pattern_finds_unicode_space() {
        assert_eq!(first_space("é\u{2003}a"), Some(2));
        assert_eq!(first_space("abc"), None);
        assert_eq!(first_space("🦀\t x"), Some(4));
        assert_eq!(first_space(""), None);
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    #[test]
    fn p_predictions() {
        // P1. find returns a position. Which unit?
        let p1: Option<usize> = guess();
        assert_eq!(p1, "aé:b".find(':'), "P1: find");

        // P2. Do matches overlap? How many times does "ana" occur in "banana"?
        let p2: usize = guess();
        assert_eq!(p2, "banana".matches("ana").count(), "P2: non-overlapping");

        // P3. A predicate pattern: what is left after trimming leading numerics?
        let p3: &str = guess();
        assert_eq!(
            p3,
            "12a3b".trim_start_matches(char::is_numeric),
            "P3: predicate patterns"
        );

        // P4. rfind of a 4-byte scalar in "🦀🦀".
        let p4: Option<usize> = guess();
        assert_eq!(p4, "🦀🦀".rfind('🦀'), "P4: rfind");
    }
}
