//! DRILL 07-08 · Returning a borrowed error
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch07`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: what is the type of e inside a match on &result?

use std::num::ParseIntError;

fn check(result: &Result<u8, ParseIntError>) -> Result<u8, ParseIntError> {
    match result {
        Ok(n) => Ok(*n),
        Err(e) => Err(e),
    }
}

fn main() {
    let bad: Result<u8, _> = "x".parse();
    assert!(check(&bad).is_err()); // KEEP
    assert_eq!(check(&Ok(3)), Ok(3)); // KEEP
}
