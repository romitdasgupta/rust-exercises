//! LESSON 05 — Loop values and labeled blocks
//! Run: cargo test --test chapter06 l05_
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider: Can break carry a value from a for loop? How does return inside a nested loop differ from break to a label?
//! IMPLEMENT the tasks below. Preserve the tests and add an edge case.
//! EXPLAIN afterward: Explain the never type, value-producing loop, labeled early exits, and why an unconditional loop can initialize a non-Option result.
//! COMPILER / TRANSFER LAB: Write a nested search with a labeled outer loop, then a labeled block. Compare unlabeled break, break to a label, and return. Type a branch as return and inspect why it coexists with another branch returning an integer.
//! Use examples/ch06_scratch.rs; repair intentional failures before committing.

/// A. Find the least u32 n whose square is >= bound (bound is u32).
/// Use a value-producing loop and break n. Widen before multiplying.
fn ceiling_root(bound: u32) -> u32 {
    todo!("Implement this task")
}
/// B. Use a labeled block with early breaks to return Some((left,right)) only
/// when a '/' exists and both halves are nonempty. Split at the first '/'.
fn nonempty_pair(text: &str) -> Option<(&str, &str)> {
    todo!("Implement this task")
}
#[cfg(test)]
mod tests {
    use super::*;
    use practice::guess;
    #[test]
    fn a_loop_value_matches_ceiling() {
        assert_eq!(ceiling_root(0), 0);
        assert_eq!(ceiling_root(1), 1);
        assert_eq!(ceiling_root(16), 4);
        assert_eq!(ceiling_root(17), 5);
        assert_eq!(ceiling_root(4_294_836_225), 65535);
        assert_eq!(ceiling_root(4_294_836_226), 65536);
        assert_eq!(ceiling_root(u32::MAX), 65536);
    }
    #[test]
    fn b_labeled_block_has_early_exits() {
        assert_eq!(nonempty_pair("a/b/c"), Some(("a", "b/c")));
        assert_eq!(nonempty_pair("a"), None);
        assert_eq!(nonempty_pair("/b"), None);
        assert_eq!(nonempty_pair("a/"), None);
        assert_eq!(nonempty_pair("/"), None);
        assert_eq!(nonempty_pair(""), None);
        assert_eq!(nonempty_pair("港/é"), Some(("港", "é")));
    }
    #[test]
    fn b_halves_borrow_the_input() {
        let text = String::from("left/right");
        let (left, right) = nonempty_pair(&text).unwrap();
        assert_eq!(left.as_ptr(), text.as_ptr());
        assert_eq!(right.as_ptr() as usize, text.as_ptr() as usize + 5);
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    #[test]
    fn p_predictions() {
        // P1. `continue 'rows` abandons the rest of the inner loop.
        let mut cells = 0;
        'rows: for r in 0..4 {
            for c in 0..4 {
                if c > r {
                    continue 'rows;
                }
                cells += 1;
            }
        }
        let p1: i32 = guess();
        assert_eq!(p1, cells, "P1: continue to a label");

        // P2. The value carried by break becomes the value of the loop.
        let mut n = 1;
        let first_big = loop {
            if n > 100 {
                break n;
            }
            n *= 3;
        };
        let p2: i32 = guess();
        assert_eq!(p2, first_big, "P2: break with a value");

        // P3. A labeled block with early exits. Which label for "港"?
        let text = "港";
        let kind = 'kind: {
            if text.is_empty() {
                break 'kind "empty";
            }
            if text.len() > 3 {
                break 'kind "long";
            }
            "short"
        };
        let p3: &str = guess();
        assert_eq!(p3, kind, "P3: the block's tail runs if no break fires");
    }
}
