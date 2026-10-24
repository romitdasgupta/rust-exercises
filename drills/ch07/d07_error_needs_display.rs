//! DRILL 07-07 · What makes a type an Error
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch07`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: what does the Error trait require besides Debug?

use std::error::Error;

#[derive(Debug)]
struct QuotaError {
    used: u32,
}

impl Error for QuotaError {}

fn main() {
    let err: Box<dyn Error> = Box::new(QuotaError { used: 9 }); // KEEP
    assert_eq!(err.to_string(), "quota exceeded: 9 used"); // KEEP
}
