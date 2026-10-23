//! LESSON 06 — Array and slice patterns
//! Run: cargo test --test chapter10 l06_
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider: How do [a,b] and [first,..,last] differ on a slice? Can .. bind the middle slice?
//! IMPLEMENT the tasks below. Preserve the tests and add an edge case.
//! EXPLAIN afterward: Explain fixed array length in a type versus a runtime slice shape and the borrowed rest binding.
//! COMPILER / TRANSFER LAB: Match a three-element array with a two-element pattern and compare the diagnostic to matching a slice of runtime length. Try multiple .. patterns in one slice. Use rest @ .. and inspect the resulting slice length.
//! Use examples/ch10_scratch.rs; repair intentional failures before committing.

/// A. Return borrowed (first,middle,last) for slices of at least two Strings;
/// return None for lengths 0 or 1. Use one slice pattern; no allocation.
fn ends(values: &[String]) -> Option<(&String, &[String], &String)> {
    todo!("Implement this task")
}
/// B. Destructure exactly three bytes and return first XOR middle XOR last.
fn checksum([a, b, c]: [u8; 3]) -> u8 {
    todo!("Implement this task")
}
#[cfg(test)]
mod tests {
    use super::*;
    use practice::{alloc::count, guess};
    #[test]
    fn a_rest_is_a_borrowed_slice() {
        let a = vec!["a".into(), "b".into(), "c".into(), "d".into()];
        let (f, m, l) = ends(&a).unwrap();
        assert_eq!((f.as_str(), l.as_str()), ("a", "d"));
        assert_eq!(m, ["b", "c"]);
        assert_eq!(m.as_ptr(), a[1..].as_ptr());
        assert!(ends(&[]).is_none());
        assert!(ends(&a[..1]).is_none());
        assert!(ends(&a[..2]).unwrap().1.is_empty());
        let (f, m, l) = ends(&a[..3]).unwrap();
        assert_eq!((f.as_str(), m, l.as_str()), ("a", &a[1..2], "c"));
        assert!(
            std::ptr::eq(l, &a[2]),
            "last must borrow the slice's element"
        );
        let (_, allocations) = count(|| ends(&a).map(|(_, m, _)| m.len()));
        assert_eq!(
            allocations, 0,
            "a slice pattern borrows; it never allocates"
        );
    }
    #[test]
    fn b_array_pattern_has_known_length() {
        assert_eq!(checksum([1, 2, 3]), 0);
        assert_eq!(checksum([0xff, 0x0f, 0xf0]), 0);
        assert_eq!(checksum([1, 0, 0]), 1);
        assert_eq!(checksum([0, 0, 0x80]), 0x80);
        assert_eq!(checksum([0, 0x41, 0]), 0x41);
    }

    fn shape(values: &[i32]) -> &'static str {
        match values {
            [] => "empty",
            [a, .., b] if a == b => "same ends",
            [_, ..] => "other",
        }
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    #[test]
    fn p_predictions() {
        // P1. Does [a, .., b] match a one-element slice?
        let p1: [&str; 3] = guess();
        assert_eq!(
            p1,
            [shape(&[5]), shape(&[1, 2, 1]), shape(&[1, 2])],
            "P1: minimum lengths of slice patterns"
        );

        // P2. On an ARRAY, what does `mid @ ..` bind? Answer with its size in
        // bytes.
        let [_, mid @ .., _] = [1i32, 2, 3, 4, 5];
        let p2: usize = guess();
        assert_eq!(p2, size_of_val(&mid), "P2: rest patterns on arrays");
    }
}
