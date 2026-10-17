//! LESSON 02 — Reference bindings and identity
//! Run: cargo test --test chapter05 l02_
//!
//! PREDICT before running: Can a mutable binding to &i32 be rebound without mutating either integer? Does == on references compare addresses?
//! IMPLEMENT the tasks below. Preserve the tests and add an edge case.
//! EXPLAIN afterward: Explain reference binding mutability, referent mutability, nested references, and std::ptr::eq.
//! COMPILER / TRANSFER LAB: Create equal Strings in two allocations. Compare == and ptr::eq. Then borrow a reference again and count which dereferences are explicit when accessing an integer.
//! Use examples/ch05_scratch.rs; repair intentional failures before committing.

/// A. Start with a reference to first, reassign it to second when requested,
/// and return the selected integer through a reference-to-reference.
fn selected_value(first: &i32, second: &i32, use_second: bool) -> i32 {
    todo!("Implement this task")
}
/// B. Return (equal_contents, same_String_object), not buffer identity.
fn compare(left: &String, right: &String) -> (bool, bool) {
    todo!("Implement this task")
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a_rebinding_does_not_modify_inputs() {
        let a = 3;
        let b = 9;
        assert_eq!(selected_value(&a, &b, false), 3);
        assert_eq!(selected_value(&a, &b, true), 9);
        assert_eq!((a, b), (3, 9));
    }
    #[test]
    fn b_identity_differs_from_equality() {
        let a = "same".to_owned();
        let b = a.clone();
        assert_eq!(compare(&a, &b), (true, false));
        assert_eq!(compare(&a, &a), (true, true));
        assert_eq!(compare(&a, &"other".to_owned()), (false, false));
    }
}
