//! DRILL 07-09 · ? with a From impl
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch07`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: can ? turn a ParseIntError into an AppError here?

use std::num::ParseIntError;

#[derive(Debug)]
enum AppError {
    BadNumber(ParseIntError),
}

impl From<ParseIntError> for AppError {
    fn from(e: ParseIntError) -> Self {
        AppError::BadNumber(e)
    }
}

fn workers(text: &str) -> Result<u8, AppError> {
    let n: u8 = text.trim().parse()?;
    Ok(n)
}

fn main() {
    assert_eq!(workers(" 4 ").ok(), Some(4)); // KEEP
    assert!(matches!(workers("many"), Err(AppError::BadNumber(_)))); // KEEP
}
