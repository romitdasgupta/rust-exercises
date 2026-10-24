//! LESSON 02 — Reference bindings and identity
//! Run: cargo test --test chapter05 l02_
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider: Can a mutable binding to &i32 be rebound without mutating either integer? Does == on references compare addresses?
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
    use practice::guess;
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
        // Empty Strings own no heap buffer, so their buffer pointers can be
        // equal even though they are different String objects.
        let (e1, e2) = (String::new(), String::new());
        assert_eq!(compare(&e1, &e2), (true, false));
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    #[test]
    fn p_predictions() {
        // P1. Equal values in different places: `==` versus `ptr::eq`.
        let a = 7;
        let b = 7;
        let (ra, rb) = (&a, &b);
        let p1: (bool, bool) = guess();
        assert_eq!(p1, (ra == rb, std::ptr::eq(ra, rb)), "P1: value or address");

        // P2. Assigning through a reference to a reference.
        let x = 1;
        let y = 2;
        let mut r = &x;
        let rr = &mut r;
        *rr = &y;
        let p2: (i32, i32) = guess(); // (*r, x)
        assert_eq!(p2, (*r, x), "P2: rebinding r, not changing x");

        // P3. ptr::eq on slices compares the address AND the length.
        let data = [1, 2, 3];
        let p3: (bool, bool) = guess();
        assert_eq!(
            p3,
            (
                std::ptr::eq(&data[..2], &data[..3]),
                std::ptr::eq(&data[..2], &data[0..2])
            ),
            "P3: fat-pointer identity"
        );
    }
}
