//! LESSON 02 — Moves across assignments and fields
//! Run: cargo test --test chapter04 l02_
//!
//! PREDICT before running: After moving one String out of a tuple, can you use its other field? Can you use the whole tuple?
//! IMPLEMENT the tasks below. Preserve the tests and add an edge case.
//! EXPLAIN afterward: Explain partial moves, the remaining fields, and the difference between moving a tuple and destructuring it.
//! COMPILER / TRANSFER LAB: Move a String into a function, then try printing the old binding. Repair by accepting the returned value. Destructure only one field of a tuple and test which expressions compile.
//! Use examples/ch04_scratch.rs; repair intentional failures before committing.

/// A. Consume a pair; return the second string followed by the first. No clones.
fn reverse_pair(pair: (String, String)) -> (String, String) {
    todo!("Implement this task")
}
/// B. Consume a vector and one label; append that label, then return the vector.
fn append_owned(mut values: Vec<String>, label: String) -> Vec<String> {
    todo!("Implement this task")
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a_fields_move_independently() {
        let a = "a".to_owned();
        let b = "b".to_owned();
        let pa = a.as_ptr();
        let pb = b.as_ptr();
        let (b, a) = reverse_pair((a, b));
        assert_eq!((b.as_str(), a.as_str()), ("b", "a"));
        assert_eq!(a.as_ptr(), pa);
        assert_eq!(b.as_ptr(), pb);
    }
    #[test]
    fn b_insertion_takes_ownership() {
        assert_eq!(
            append_owned(vec!["first".into()], "last".into()),
            ["first", "last"]
        );
        assert_eq!(append_owned(vec![], "".into()), [""]);
    }
}
