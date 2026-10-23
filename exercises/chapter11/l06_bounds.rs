//! LESSON 06 — Opaque results, lifetimes, and precise capture
//! Run: cargo test --test chapter11 l06_
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider: Does impl Iterator permit returning two
//! different iterator types from branches? Which inputs may the opaque result borrow?
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
    use practice::{alloc::count, guess};
    use std::hint::black_box;
    use std::rc::Rc;
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

    #[test]
    fn a_words_split_on_unicode_whitespace() {
        let text = "a\u{2003}b\u{3000}c";
        assert_eq!(words(text, "").collect::<Vec<_>>(), ["a", "b", "c"]);
        let first = words(text, "").next().unwrap();
        assert_eq!(first.as_ptr(), text.as_ptr(), "words borrow from text");
    }

    #[test]
    fn b_predicate_does_not_allocate_per_call() {
        let prefix = String::from("ab");
        let p = starts_with(&prefix);
        let (hits, allocations) = count(|| {
            ["abc", "xab", "ab", ""]
                .into_iter()
                .filter(|t| p(black_box(*t)))
                .count()
        });
        assert_eq!(hits, 2);
        assert_eq!(
            allocations, 0,
            "the closure should borrow prefix, not copy it"
        );
    }

    fn duplicate<T: Clone>(value: T) -> (T, T) {
        (value.clone(), value)
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    #[test]
    fn p_predictions() {
        // P1. A returned closure is a value made of its captures. Sizes in machine words?
        let prefix: &str = "ab";
        let none = |t: &str| t.is_empty();
        let borrowing = move |t: &str| t.starts_with(prefix);
        let p1: (usize, usize) = guess();
        assert_eq!(
            (p1.0 * size_of::<usize>(), p1.1 * size_of::<usize>()),
            (size_of_val(&none), size_of_val(&borrowing)),
            "P1: closure sizes"
        );

        // P2. `duplicate` needs T: Clone. T may be an Rc, or a reference.
        let shared = Rc::new(vec![1, 2, 3]);
        let (a, b) = duplicate(Rc::clone(&shared));
        let text = String::from("owned");
        let (_, allocations) = count(|| duplicate(black_box(&text)));
        let p2: (usize, usize) = guess();
        assert_eq!(
            p2,
            (Rc::strong_count(&shared), allocations),
            "P2: what Clone copies"
        );
        drop((a, b));
    }
}
