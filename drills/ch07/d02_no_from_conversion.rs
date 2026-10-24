//! DRILL 07-02 · ? converts errors only with From
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch07`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: is there a From conversion from io::Error into ParseIntError?

use std::io::Read;
use std::num::ParseIntError;

fn read_number(mut input: impl Read) -> Result<u32, ParseIntError> {
    let mut text = String::new();
    input.read_to_string(&mut text)?;
    text.trim().parse()
}

fn main() {
    assert_eq!(read_number("42".as_bytes()).ok(), Some(42)); // KEEP
}
