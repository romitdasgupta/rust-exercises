//! LESSON 07 — ToOwned and clone-on-write
//! Run: cargo test --test chapter13 l07_
//!
//! PREDICT before executing: What owned type corresponds to str::to_owned()? Does
//! Cow::Borrowed allocate when read? When is it promoted to Owned?
//!
//! IMPLEMENT the documented tasks, then keep the tests and add an edge case.
//! EXPLAIN afterward: Explain why ToOwned generalizes Clone for unsized borrowed data.
//! Distinguish Cow::into_owned from Cow::to_mut.
//!
//! COMPILER / MANUAL LAB (use examples/ch13_scratch.rs):
//! Create Cow<[u8]> from a slice; call to_mut twice, edit once, and confirm the original
//! slice stays unchanged. Do the same with Cow<str> and explain the owned type.
//!
//! TRANSFER: invent a new edge case, predict it, then add a check. A lesson is
//! complete only when its tests, predictions, lab, and explanation are done.

use std::borrow::Cow;
/// A. Remove every '\r'. Borrow unchanged input, own changed output.
fn unix_text(text: &str) -> Cow<'_, str> {
    todo!("Task 1: implement the documented contract")
}
/// B. Use ToOwned to turn arbitrary borrowed text into an independent String.
fn own_text(text: &str) -> String {
    todo!("Task 2: implement the documented contract")
}
/// C. Append a suffix through Cow::to_mut, returning the resulting Cow.
/// An empty suffix must preserve a borrowed representation.
fn append<'a>(mut text: Cow<'a, str>, suffix: &str) -> Cow<'a, str> {
    todo!("Task 3: implement the documented contract")
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a_allocation_is_conditional() {
        assert!(matches!(unix_text("a\nb"), Cow::Borrowed(_)));
        assert_eq!(unix_text("a\r\nb\r"), "a\nb");
        assert!(matches!(unix_text("\r"), Cow::Owned(_)));
    }
    #[test]
    fn b_to_owned_copies_unsized_text() {
        let source = String::from("β");
        let mut owned = own_text(&source);
        owned.push('!');
        assert_eq!(source, "β");
        assert_eq!(owned, "β!");
    }
    #[test]
    fn c_clone_on_write_preserves_source_and_skips_empty_edits() {
        let s = String::from("a");
        let out = append(Cow::Borrowed(&s), "🦀");
        assert!(matches!(out, Cow::Owned(_)));
        assert_eq!(out, "a🦀");
        assert_eq!(s, "a");
        assert!(matches!(append(Cow::Borrowed("a"), ""), Cow::Borrowed(_)));
        assert_eq!(append(Cow::Owned("x".into()), "!"), "x!");
    }
}
