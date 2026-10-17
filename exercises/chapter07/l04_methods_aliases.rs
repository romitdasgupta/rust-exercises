//! LESSON 04 — Result methods and aliases
//! Run: cargo test --test chapter07 l04_
//!
//! PREDICT before running: What is the difference between map and and_then? Is unwrap_or_else lazy? Does a Result alias create a new type?
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
    #[test]
    fn a_error_categories_are_distinct() {
        assert_eq!(positive(" 3 "), Ok(3));
        assert_eq!(positive("0"), Err("zero count"));
        assert_eq!(positive("oops"), Err("not a count"));
    }
    #[test]
    fn b_optional_value_preserves_absence() {
        assert_eq!(optional(None), Ok(None));
        assert_eq!(optional(Some("8")), Ok(Some(8)));
        assert_eq!(optional(Some("0")), Err("zero count"));
        assert_eq!(optional(Some("bad")), Err("not a count"));
    }
}
