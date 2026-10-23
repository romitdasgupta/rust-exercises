//! LESSON 08 — Capstone: validated names with cheap views
//! Run: cargo test --test chapter13 l08_
//!
//! PREDICT before running: Plan which operations borrow, which consume, and which allocate.
//! Should a blank name have a Default implementation?
//!
//! IMPLEMENT the documented tasks, then keep the tests and add an edge case.
//! EXPLAIN afterward: Which utility traits would be misleading for this type? Why does
//! AsRef expose a view while TryFrom performs validation?
//!
//! COMPILER / MANUAL LAB (use examples/ch13_scratch.rs):
//! Add Borrow<str> only after defining equality/hash identity. Write a HashMap lookup test.
//! Explain why normalized display and original identity cannot be chosen independently.
//!
//! TRANSFER: invent a new edge case, predict it, then add a check. A lesson is
//! complete only when its tests, predictions, lab, and explanation are done.

use std::borrow::Cow;
#[derive(Debug, Clone, PartialEq, Eq)]
struct Name(String);
impl TryFrom<&str> for Name {
    type Error = &'static str;
    /// A. Trim Unicode whitespace. Reject blank with "blank name". Own the rest.
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        todo!("Task 1: implement the documented contract")
    }
}
impl AsRef<str> for Name {
    /// B. Expose the validated text without copying.
    fn as_ref(&self) -> &str {
        todo!("Task 2: implement the documented contract")
    }
}
/// C. For empty tag return a borrowed name; otherwise own "NAME [TAG]".
fn annotated<'a>(name: &'a Name, tag: &str) -> Cow<'a, str> {
    todo!("Task 3: implement the documented contract")
}
#[cfg(test)]
mod tests {
    use super::*;
    use practice::alloc::count;
    use std::hint::black_box;
    #[test]
    fn a_validation_trims_and_rejects_blank() {
        assert_eq!(Name::try_from(" \u{2003}"), Err("blank name"));
        assert_eq!(Name::try_from(" 🦀 "), Ok(Name("🦀".into())));
    }
    #[test]
    fn b_view_and_clone_keep_ownership_clear() {
        let name = Name("Ada".into());
        assert_eq!(name.as_ref(), "Ada");
        let copy = name.clone();
        assert_eq!(name, copy);
        assert_ne!(name.0.as_ptr(), copy.0.as_ptr());
    }
    #[test]
    fn c_annotation_allocates_only_for_added_text() {
        let name = Name("Ada".into());
        assert!(matches!(annotated(&name, ""), Cow::Borrowed(_)));
        assert_eq!(annotated(&name, "staff"), "Ada [staff]");
        assert!(matches!(annotated(&name, "x"), Cow::Owned(_)));
    }

    #[test]
    fn a_validation_keeps_interior_whitespace() {
        assert_eq!(
            Name::try_from("\tAda Lovelace\n"),
            Ok(Name("Ada Lovelace".into()))
        );
        assert_eq!(Name::try_from(""), Err("blank name"));
    }

    #[test]
    fn a_validation_allocates_only_the_final_name() {
        let (name, allocations) = count(|| Name::try_from(black_box("   Grace   ")));
        assert_eq!(name, Ok(Name("Grace".into())));
        assert!(
            allocations <= 1,
            "trim the borrowed input, then own it once; got {allocations}"
        );
    }

    #[test]
    fn b_c_views_borrow_the_stored_text() {
        let name = Name("Ada".into());
        assert_eq!(name.as_ref().as_ptr(), name.0.as_ptr());
        let (view, allocations) = count(|| annotated(&name, ""));
        assert_eq!(allocations, 0, "an empty tag needs no new text");
        assert_eq!(view.as_ptr(), name.0.as_ptr());
    }
}
