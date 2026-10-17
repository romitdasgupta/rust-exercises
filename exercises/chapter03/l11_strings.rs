//! LESSON 11 — Building text and writing literals
//! Run: cargo test --test chapter03 l11_
//!
//! PREDICT: What are the types of "hi", 'h', "hi".to_string(), and r"\n"?
//! How many bytes are in r"\n" versus "\n"? What does String::pop return
//! for a final emoji? What changes between Display and Debug output for text?
//!
//! EXPLAIN: Why accept &str when reading input, and return String for newly
//! built text? Which value owns the UTF-8 buffer? Does `let mut s = "hi"`
//! turn that borrowed text into a growable owned string?
//!
//! COMPILER LAB: print a string containing a newline with `{}` and `{:?}`.
//! Try a multiline literal, then escape the source line break with a trailing
//! backslash. Compare included whitespace. Try embedding quotes in a raw string.

/// A. Build exactly: "Hi NAME, you have N unread messages."
/// Use format!, including when N is 1. Do not trim or alter the supplied name.
fn inbox_message(name: &str, unread: usize) -> String {
    todo!("A: build an owned String from borrowed input and a number")
}

/// B. Append a space, '[', the tag, and ']' to the existing String.
/// Use String's append methods; preserve all existing content.
fn append_tag(message: &mut String, tag: &str) {
    todo!("B: append both string slices and individual characters")
}

/// C. Trim each item and join the results with " | ". Preserve empty items.
/// Empty input yields an empty String. Use join; a small loop may prepare items.
fn trimmed_list(items: &[&str]) -> String {
    todo!("C: borrow trimmed pieces, then allocate the joined text")
}

/// D. Remove and return the final Unicode scalar, or None for an empty String.
fn remove_last(text: &mut String) -> Option<char> {
    todo!("D: remove a scalar without corrupting UTF-8")
}

/// E. Return the same text twice: Open "draft.txt" in C:\notes
/// First use an ordinary escaped literal; second use a raw literal with hashes.
/// The supplied 'static annotation describes literal text that lasts for the
/// program's lifetime. You do not need to design lifetimes in this exercise.
fn literal_forms() -> (&'static str, &'static str) {
    todo!("E: compare escaped and raw forms of identical text")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_format_builds_an_owned_result() {
        assert_eq!(
            inbox_message("Ada", 3),
            "Hi Ada, you have 3 unread messages."
        );
        assert_eq!(inbox_message("李", 1), "Hi 李, you have 1 unread messages.");
    }

    #[test]
    fn b_append_preserves_existing_utf8() {
        let mut message = "ready 🦀".to_string();
        append_tag(&mut message, "lab");
        assert_eq!(message, "ready 🦀 [lab]");
        append_tag(&mut message, "");
        assert_eq!(message, "ready 🦀 [lab] []");
    }

    #[test]
    fn c_trim_and_join_keep_empty_fields() {
        assert_eq!(
            trimmed_list(&["  alpha", "β  ", "\n gamma \t"]),
            "alpha | β | gamma"
        );
        assert_eq!(trimmed_list(&[" a ", "", " b "]), "a |  | b");
        assert_eq!(trimmed_list(&[]), "");
        assert_eq!(trimmed_list(&["   "]), "");
    }

    #[test]
    fn d_pop_removes_a_whole_scalar() {
        let mut text = "a🦀".to_string();
        assert_eq!(remove_last(&mut text), Some('🦀'));
        assert_eq!(text, "a");
        assert_eq!(remove_last(&mut text), Some('a'));
        assert_eq!(remove_last(&mut text), None);
        let mut combining = "e\u{301}".to_string();
        assert_eq!(remove_last(&mut combining), Some('\u{301}'));
        assert_eq!(combining, "e");
    }

    #[test]
    fn e_raw_and_escaped_literals_can_mean_the_same_text() {
        let (escaped, raw) = literal_forms();
        assert_eq!(escaped, "Open \"draft.txt\" in C:\\notes");
        assert_eq!(escaped, raw);
    }
}
