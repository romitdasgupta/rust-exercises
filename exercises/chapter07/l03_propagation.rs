//! LESSON 03 — Propagating failures with question mark
//! Run: cargo test --test chapter07 l03_
//!
//! PREDICT before running: What must the enclosing return type support for ? to work? Does ? keep processing later inputs after the first Err?
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
    #[test]
    fn a_product_propagates_both_operands() {
        assert_eq!(product(" 9 ", "8").unwrap(), 72);
        assert_eq!(product("65535", "65535").unwrap(), 4294836225);
        assert!(product("bad", "4").is_err());
        assert!(product("4", "-1").is_err());
    }
    #[test]
    fn b_sequence_stops_on_error() {
        assert_eq!(parse_all(&["1", " -2 "]).unwrap(), [1, -2]);
        assert!(parse_all(&["1", "bad", "3"]).is_err());
        assert_eq!(parse_all(&[]).unwrap(), []);
    }
}
