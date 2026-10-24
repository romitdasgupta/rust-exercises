//! LESSON 06 — Independent lifetimes and elision
//! Run: cargo test --test chapter05 l06_
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider: If two fields borrow different owners, must their lifetimes be identical? Which input does a method return borrow default to?
//! IMPLEMENT the tasks below. Preserve the tests and add an edge case.
//! EXPLAIN afterward: State the function lifetime elision rules and explain when independent lifetimes let one result outlive another input.
//! COMPILER / TRANSFER LAB: Write a two-input function returning one reference without annotations; inspect the ambiguity. Annotate only the selected input. Write an impl method with &self and another borrowed parameter; predict the elided output lifetime.
//! Use examples/ch05_scratch.rs; repair intentional failures before committing.

struct Pair<'left, 'right> {
    left: &'left str,
    right: &'right str,
}
/// A. Construct a view from independently borrowed strings.
fn pair<'a, 'b>(left: &'a str, right: &'b str) -> Pair<'a, 'b> {
    todo!("Implement this task")
}
/// B. Return left text with its original lifetime, regardless of the right
/// field or the wrapper's lifetime.
fn left_text<'a>(pair: &Pair<'a, '_>) -> &'a str {
    todo!("Implement this task")
}
#[cfg(test)]
mod tests {
    use super::*;
    use practice::guess;
    #[test]
    fn a_fields_can_have_distinct_owners() {
        let a = "left".to_owned();
        {
            let b = "right".to_owned();
            let p = pair(&a, &b);
            assert_eq!((p.left, p.right), ("left", "right"));
        }
    }
    #[test]
    fn b_left_survives_short_right_owner() {
        let a = String::from("long");
        let out;
        {
            let b = String::from("short");
            let p = pair(&a, &b);
            out = left_text(&p);
        }
        assert_eq!(out, "long");
        assert_eq!(out.as_ptr(), a.as_ptr());
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    #[test]
    fn p_predictions() {
        // P1. Do lifetime parameters take up space at run time?
        let p1: bool = guess(); // true if the sizes are equal
        assert_eq!(
            p1,
            size_of::<Pair<'static, 'static>>() == size_of::<(&str, &str)>(),
            "P1: lifetimes are erased after type checking"
        );

        // P2. Both fields may borrow one owner. How many bytes apart do they start?
        let s = String::from("leftright");
        let p = Pair {
            left: &s[..4],
            right: &s[4..],
        };
        let p2: usize = guess();
        assert_eq!(
            p2,
            p.right.as_ptr() as usize - p.left.as_ptr() as usize,
            "P2: two views into one buffer"
        );
    }
}
