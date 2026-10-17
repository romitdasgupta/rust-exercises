//! LESSON 06 — Opaque results, lifetimes, and precise capture
//! Run: cargo test --test chapter11 l06_
//!
//! PREDICT before executing: Does impl Iterator permit returning two different iterator
//! types from branches? Which inputs may the opaque result borrow?
//!
//! IMPLEMENT the documented tasks, then keep the tests and add an edge case.
//! EXPLAIN afterward: Explain every bound in these signatures. Why does use<'a> list
//! captures while + 'a expresses an outlives requirement?
//!
//! COMPILER / MANUAL LAB (use examples/ch11_scratch.rs):
//! Write fn words<'a,'b>(text:&'a str, scratch:&'b str)->impl Iterator<Item=&'a str> +
//! use<'a>. Ignore scratch and return text.split_whitespace(). Drop scratch before
//! consuming the iterator. Remove use<'a> in edition 2024 and explain broader capture.
//! Separately remove Clone from duplicated; use diagnostics to reconstruct the minimal
//! bound.
//!
//! TRANSFER: invent a new edge case, predict it, then add a check. A lesson is
//! complete only when its tests, predictions, lab, and explanation are done.

/// A. Return borrowed whitespace-separated words; precisely capture only 'a.
#[allow(unreachable_code)]
fn words<'a, 'b>(text: &'a str, _scratch: &'b str) -> impl Iterator<Item = &'a str> + use<'a> {
    let iterator: std::str::SplitWhitespace<'a> =
        todo!("Task 1: implement the documented contract");
    iterator
}
/// B. Return a closure that borrows prefix. It must not allocate on every call.
fn starts_with<'a>(prefix: &'a str) -> impl Fn(&str) -> bool + 'a {
    move |_text: &str| -> bool { todo!("Task 2: implement the documented contract") }
}
/// C. Return two independently owned T values using only the necessary Clone bound.
fn duplicated<T: Clone>(value: T) -> (T, T) {
    todo!("Task 3: implement the documented contract")
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a_precise_capture_does_not_borrow_scratch() {
        let text = String::from(" air\tsea\n🦀 ");
        let iter;
        {
            let temporary = String::from("scratch");
            iter = words(&text, &temporary);
        }
        assert_eq!(iter.collect::<Vec<_>>(), ["air", "sea", "🦀"]);
        assert_eq!(words("", "ignored").count(), 0);
    }
    #[test]
    fn b_opaque_closure_borrows_the_prefix() {
        let prefix = String::from("α");
        let p = starts_with(&prefix);
        assert!(p("αβ"));
        assert!(!p("a"));
        assert!(starts_with("")("anything"));
    }
    #[test]
    fn c_clone_bound_supports_non_copy_values() {
        let (mut a, b) = duplicated(vec![1, 2]);
        a.push(3);
        assert_eq!(b, [1, 2]);
    }
}
