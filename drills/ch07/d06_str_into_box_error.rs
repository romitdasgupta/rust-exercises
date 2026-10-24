//! DRILL 07-06 · A &str is not a boxed error
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch07`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: what type does Err("empty input") have here?

use std::error::Error;

fn first_word(text: &str) -> Result<&str, Box<dyn Error>> {
    match text.split_whitespace().next() {
        Some(word) => Ok(word),
        None => Err("empty input"),
    }
}

fn main() {
    assert_eq!(first_word(" hi there").ok(), Some("hi")); // KEEP
    assert!(first_word("  ").is_err()); // KEEP
}
