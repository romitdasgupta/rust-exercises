//! LESSON 04 — Result methods and aliases
//! Run: cargo test --test chapter07 l04_
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider: What is the difference between map and and_then? Is unwrap_or_else lazy? Does a Result alias create a new type?
//! IMPLEMENT the tasks below. Preserve the tests and add an edge case.
//! EXPLAIN afterward: Explain how combinators transform either the success or error side while retaining explicit failure.
//! COMPILER / TRANSFER LAB: Use a counter in unwrap_or and unwrap_or_else fallback expressions and compare successful Results. Expand the alias to its full type and show the unchanged API.
//! Use examples/ch07_scratch.rs; repair intentional failures before committing.

type CountResult<T> = Result<T, &'static str>;
/// A. Parse a trimmed u16; map parse failures to "not a count" and reject
/// zero with "zero count". Use map_err and and_then.
fn positive(text: &str) -> CountResult<u16> {
    todo!("Implement this task")
}
/// B. None means no requested count, Some text must pass positive. Use map
/// and transpose instead of treating absence as failure.
fn optional(text: Option<&str>) -> CountResult<Option<u16>> {
    todo!("Implement this task")
}
#[cfg(test)]
mod tests {
    use super::*;
    use practice::guess;
    use std::any::TypeId;
    #[test]
    fn a_error_categories_are_distinct() {
        assert_eq!(positive(" 3 "), Ok(3));
        assert_eq!(positive("0"), Err("zero count"));
        assert_eq!(positive("oops"), Err("not a count"));
        assert_eq!(positive("65536"), Err("not a count"));
        assert_eq!(positive("-0"), Err("not a count"));
        assert_eq!(positive("65535"), Ok(65535));
    }
    #[test]
    fn b_optional_value_preserves_absence() {
        assert_eq!(optional(None), Ok(None));
        assert_eq!(optional(Some("8")), Ok(Some(8)));
        assert_eq!(optional(Some("0")), Err("zero count"));
        assert_eq!(optional(Some("bad")), Err("not a count"));
        assert_eq!(optional(Some("")), Err("not a count"), "present but empty");
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    #[test]
    fn p_predictions() {
        // P1. unwrap_or evaluates its argument first; unwrap_or_else only on Err.
        let mut calls = 0;
        let mut fallback = || {
            calls += 1;
            0
        };
        let ok: Result<u8, &str> = "4".parse().map_err(|_| "bad");
        let values = (ok.unwrap_or(fallback()), ok.unwrap_or_else(|_| fallback()));
        assert_eq!(values, (4, 4));
        let p1: i32 = guess();
        assert_eq!(p1, calls, "P1: eager versus lazy fallbacks");

        // P2. map wraps the closure's result; and_then uses it as the Result.
        let three: Result<u8, &str> = Ok(3);
        let p2: (Result<Option<u8>, &str>, Result<u8, &str>) = guess();
        assert_eq!(
            p2,
            (
                three.map(|n| n.checked_mul(100)),
                three.and_then(|n| n.checked_mul(100).ok_or("overflow"))
            ),
            "P2: map versus and_then"
        );

        // P3. transpose turns Option<Result<T, E>> into Result<Option<T>, E>.
        type Swapped = Result<Option<u8>, &'static str>;
        let p3: (Swapped, Swapped) = guess();
        assert_eq!(
            p3,
            (
                Some(Err::<u8, &str>("x")).transpose(),
                None::<Result<u8, &str>>.transpose()
            ),
            "P3: transpose"
        );

        // P4. Is CountResult<u8> a new type, or another name for an existing one?
        let p4: bool = guess(); // true if the two TypeIds are equal
        assert_eq!(
            p4,
            TypeId::of::<CountResult<u8>>() == TypeId::of::<Result<u8, &'static str>>(),
            "P4: a type alias is only a name"
        );
    }
}
