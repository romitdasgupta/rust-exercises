//! LESSON 07 — Sharing versus mutation
//! Run: cargo test --test chapter05 l07_
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider: Can a vector grow while a reference to one of its elements is still used? Does a borrow always last to the closing brace?
//! IMPLEMENT the tasks below. Preserve the tests and add an edge case.
//! EXPLAIN afterward: Explain last use, reallocation invalidation, exclusive access, and why runtime capacity is not enough to permit an overlapping borrow.
//! COMPILER / TRANSFER LAB: Borrow v[0], push another element, then print the borrow. Move the print before the push. Try simultaneous &mut and & access to one integer. Reborrow an &mut temporarily and show when the original becomes usable again.
//! Use examples/ch05_scratch.rs; repair intentional failures before committing.

/// A. Return the old byte length, then append the suffix in place.
fn measure_then_append(text: &mut String, suffix: &str) -> usize {
    todo!("Implement this task")
}
/// B. If nonempty, append a CLONE of the first String and return true.
/// Empty input stays empty and returns false. End the borrow before push.
fn duplicate_first(values: &mut Vec<String>) -> bool {
    todo!("Implement this task")
}
#[cfg(test)]
mod tests {
    use super::*;
    use practice::{alloc::count, guess};
    #[test]
    fn a_read_ends_before_write() {
        let mut s = "é".to_owned();
        assert_eq!(measure_then_append(&mut s, "!"), 2);
        assert_eq!(s, "é!");
        assert_eq!(measure_then_append(&mut s, ""), 3);
    }
    #[test]
    fn b_growth_keeps_existing_values_valid() {
        let mut v = vec!["a".into(), "b".into()];
        assert!(duplicate_first(&mut v));
        assert_eq!(v, ["a", "b", "a"]);
        assert_ne!(v[0].as_ptr(), v[2].as_ptr());
        let mut empty = vec![];
        assert!(!duplicate_first(&mut empty));
        assert!(empty.is_empty());
    }

    #[test]
    fn a_append_happens_in_place() {
        let mut text = String::with_capacity(32);
        text.push_str("log");
        let buffer = text.as_ptr();
        let (old, allocations) = count(|| measure_then_append(&mut text, ": ok"));
        assert_eq!((old, text.as_str()), (3, "log: ok"));
        assert_eq!(text.as_ptr(), buffer, "push onto the same buffer");
        assert_eq!(allocations, 0, "no new String is needed");
    }

    #[test]
    fn b_duplicate_first_clones_only_one_string() {
        let mut values = Vec::with_capacity(3);
        values.push("first".to_string());
        values.push("second".to_string());
        let (_, allocations) = count(|| duplicate_first(&mut values));
        assert_eq!(values, ["first", "second", "first"]);
        assert!(
            allocations <= 1,
            "clone one String, not the vector; got {allocations} allocations"
        );
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    #[test]
    fn p_predictions() {
        // P1. Copying an element out ends the read before the push.
        let mut v = vec![1, 2, 3];
        let first = v[0];
        v.push(first * 10);
        let p1: Vec<i32> = guess();
        assert_eq!(p1, v, "P1: copy out, then mutate");

        // P2. Swap through two mutable references, then write through one.
        let mut a = 1;
        let mut b = 2;
        {
            let (x, y) = (&mut a, &mut b);
            std::mem::swap(x, y);
            *x += 10;
        }
        let p2: (i32, i32) = guess();
        assert_eq!(p2, (a, b), "P2: x still refers to a");

        // P3. A reborrow is used, ends, and the original is usable again.
        let mut n = 0;
        let r = &mut n;
        {
            let r2 = &mut *r;
            *r2 += 5;
        }
        *r *= 2;
        let p3: i32 = guess();
        assert_eq!(p3, n, "P3: reborrowing &mut");
    }
}
