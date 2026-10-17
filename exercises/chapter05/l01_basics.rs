//! LESSON 01 — Borrowing and dereferencing
//! Run: cargo test --test chapter05 l01_
//!
//! PREDICT before running: Does passing &String move the String? Does calling len on &&String need explicit stars?
//! IMPLEMENT the tasks below. Preserve the tests and add an edge case.
//! EXPLAIN afterward: Explain who owns storage and why a method receiver can be dereferenced automatically while assignment through &mut needs *.
//! COMPILER / TRANSFER LAB: Try mutating through &String, then through &mut String. Compare `(*reference).len()` with `reference.len()` and with indexing a borrowed vector.
//! Use examples/ch05_scratch.rs; repair intentional failures before committing.

/// A. Return the string byte length without taking ownership or cloning.
fn borrowed_length(text: &String) -> usize {
    todo!("Implement this task")
}
/// B. Replace an integer through its mutable reference, returning its old value.
fn overwrite(slot: &mut i32, value: i32) -> i32 {
    todo!("Implement this task")
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a_owner_remains_usable() {
        let text = String::from("é🦀");
        assert_eq!(borrowed_length(&text), 6);
        assert_eq!(text, "é🦀");
        assert_eq!(borrowed_length(&String::new()), 0);
    }
    #[test]
    fn b_exclusive_borrow_updates_referent() {
        let mut n = -8;
        assert_eq!(overwrite(&mut n, 4), -8);
        assert_eq!(n, 4);
        assert_eq!(overwrite(&mut n, 0), 4);
    }
}
