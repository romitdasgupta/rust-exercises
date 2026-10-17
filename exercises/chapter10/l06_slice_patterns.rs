//! LESSON 06 — Array and slice patterns
//! Run: cargo test --test chapter10 l06_
//!
//! PREDICT before running: How do [a,b] and [first,..,last] differ on a slice? Can .. bind the middle slice?
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
    }
    #[test]
    fn b_array_pattern_has_known_length() {
        assert_eq!(checksum([1, 2, 3]), 0);
        assert_eq!(checksum([0xff, 0x0f, 0xf0]), 0);
        assert_eq!(checksum([1, 0, 0]), 1);
    }
}
