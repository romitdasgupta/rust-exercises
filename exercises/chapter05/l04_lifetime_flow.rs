//! LESSON 04 — Lifetimes through functions
//! Run: cargo test --test chapter05 l04_
//!
//! PREDICT before running: Does a lifetime annotation keep an owner alive? Can a function return a reference to its local String?
//! IMPLEMENT the tasks below. Preserve the tests and add an edge case.
//! EXPLAIN afterward: Describe the input-to-output lifetime relationship and why it does not allocate or extend any lifetime.
//! COMPILER / TRANSFER LAB: Write a function returning a reference to a local integer; inspect the diagnostic. Repair by taking a borrowed input, then by returning an owned integer. Add an unnecessary static bound and observe which calls it forbids.
//! Use examples/ch05_scratch.rs; repair intentional failures before committing.

/// A. Split at the FIRST colon; return two borrowed halves without the colon.
/// Return None when absent; empty halves are valid.
fn split_record(line: &str) -> Option<(&str, &str)> {
    todo!("Implement this task")
}
/// B. Return a reference to the first greatest integer; None for empty input.
/// Use an explicit lifetime parameter to connect output to input.
fn greatest<'a>(values: &'a [i32]) -> Option<&'a i32> {
    todo!("Implement this task")
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a_halves_borrow_the_original() {
        let s = String::from("港:ok:later");
        let (a, b) = split_record(&s).unwrap();
        assert_eq!((a, b), ("港", "ok:later"));
        assert_eq!(a.as_ptr(), s.as_ptr());
        assert_eq!(split_record(":"), Some(("", "")));
        assert_eq!(split_record("none"), None);
    }
    #[test]
    fn b_result_borrows_first_maximum() {
        let a = [-2, 8, 8, 1];
        assert!(std::ptr::eq(greatest(&a).unwrap(), &a[1]));
        assert_eq!(greatest(&[]), None);
        assert_eq!(greatest(&[-3]), Some(&-3));
    }
}
