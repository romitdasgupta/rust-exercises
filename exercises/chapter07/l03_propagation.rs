//! LESSON 03 — Propagating failures with question mark
//! Run: cargo test --test chapter07 l03_
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider: What must the enclosing return type support for ? to work? Does ? keep processing later inputs after the first Err?
//! IMPLEMENT the tasks below. Preserve the tests and add an edge case.
//! EXPLAIN afterward: Explain successful extraction, early return, and why parse errors are preserved rather than converted into a success value.
//! COMPILER / TRANSFER LAB: Put ? in a function returning u32 and inspect the diagnostic. Repair by returning Result. Rewrite one ? manually as match and return Err.
//! Use examples/ch07_scratch.rs; repair intentional failures before committing.

use std::num::ParseIntError;
/// A. Parse two independent trimmed u16 values, returning their widened u32
/// product. Propagate either ParseIntError using ?. No overflow of u32 here.
fn product(left: &str, right: &str) -> Result<u32, ParseIntError> {
    todo!("Implement this task")
}
/// B. Parse all inputs in order into Vec<i32>, stopping on the first error.
fn parse_all(inputs: &[&str]) -> Result<Vec<i32>, ParseIntError> {
    todo!("Implement this task")
}
#[cfg(test)]
mod tests {
    use super::*;
    use practice::guess;
    use std::num::IntErrorKind;
    #[test]
    fn a_product_propagates_both_operands() {
        assert_eq!(product(" 9 ", "8").unwrap(), 72);
        assert_eq!(product("65535", "65535").unwrap(), 4294836225);
        assert!(product("bad", "4").is_err());
        assert!(product("4", "-1").is_err());
        assert_eq!(product("0", "65535").unwrap(), 0);
    }
    #[test]
    fn a_left_operand_error_comes_first() {
        let err = product("", "x").unwrap_err();
        assert_eq!(err.kind(), &IntErrorKind::Empty, "parse left, then right");
        let err = product("7", "70000").unwrap_err();
        assert_eq!(err.kind(), &IntErrorKind::PosOverflow);
    }
    #[test]
    fn b_sequence_stops_on_error() {
        assert_eq!(parse_all(&["1", " -2 "]).unwrap(), [1, -2]);
        assert!(parse_all(&["1", "bad", "3"]).is_err());
        assert_eq!(parse_all(&[]).unwrap(), []);
    }
    #[test]
    fn b_the_first_error_is_reported() {
        let err = parse_all(&["1", "", "x", "99999999999"]).unwrap_err();
        assert_eq!(err.kind(), &IntErrorKind::Empty, "return the FIRST error");
    }

    fn sum_logged(items: &[&str], seen: &mut usize) -> Result<i32, ParseIntError> {
        let mut total = 0;
        for item in items {
            *seen += 1;
            total += item.parse::<i32>()?;
        }
        Ok(total)
    }

    fn third_char(text: &str) -> Option<char> {
        let mut chars = text.chars();
        chars.next()?;
        chars.next()?;
        chars.next()
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    #[test]
    fn p_predictions() {
        // P1. How many items does sum_logged look at before ? returns?
        let mut seen = 0;
        let result = sum_logged(&["1", "x", "3"], &mut seen);
        assert!(result.is_err());
        let p1: usize = guess();
        assert_eq!(p1, seen, "P1: ? returns early");

        // P2. Collecting into Result stops at the first Err. How many parse calls?
        let mut calls = 0;
        let parsed: Result<Vec<i32>, _> = ["1", "x", "3", "y"]
            .iter()
            .map(|s| {
                calls += 1;
                s.parse::<i32>()
            })
            .collect();
        assert!(parsed.is_err());
        let p2: usize = guess();
        assert_eq!(p2, calls, "P2: collect short-circuits");

        // P3. ? works on Option too.
        let p3: (Option<char>, Option<char>) = guess();
        assert_eq!(
            p3,
            (third_char("港x"), third_char("ab港c")),
            "P3: ? on Option returns None early"
        );
    }
}
