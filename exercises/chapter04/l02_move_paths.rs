//! LESSON 02 — Moves across assignments and fields
//! Run: cargo test --test chapter04 l02_
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider: After moving one String out of a tuple, can you use its other field? Can you use the whole tuple?
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
    use practice::{alloc::count, guess};
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

    #[test]
    fn a_b_moves_do_not_allocate() {
        let pair = ("a".to_owned(), "b".to_owned());
        let (_, allocations) = count(|| reverse_pair(pair));
        assert_eq!(allocations, 0, "reverse_pair should move, not clone");
        let mut values = Vec::with_capacity(2);
        values.push("first".to_owned());
        let label = "last".to_owned();
        let (_, allocations) = count(|| append_owned(values, label));
        assert_eq!(allocations, 0, "append_owned should move the label in");
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    #[test]
    fn p_predictions() {
        // P1. How many machine words (usize) does a String's stack header take?
        let p1: usize = guess();
        assert_eq!(
            p1 * size_of::<usize>(),
            size_of::<String>(),
            "P1: String header size"
        );

        // P2. Pushing a String into a vector: does the text get a new buffer?
        let mut v = Vec::with_capacity(1);
        let s = String::from("cargo");
        let before = s.as_ptr();
        v.push(s);
        let p2: bool = guess(); // true if v[0] still uses the same buffer
        assert_eq!(p2, v[0].as_ptr() == before, "P2: push moves the header");

        // P3. How many heap allocations does moving a 3-element Vec<String> make?
        let v = vec!["a".to_owned(), "b".to_owned(), "c".to_owned()];
        let (moved, allocations) = count(|| {
            let moved = v;
            moved
        });
        let p3: usize = guess();
        assert_eq!(p3, allocations, "P3: allocations made by a move");
        assert_eq!(moved.len(), 3);
    }
}
