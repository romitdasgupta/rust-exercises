//! DRILL 08-09 · Two globs, one name
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch08`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: which parse does the call mean?

mod json {
    pub fn parse(text: &str) -> usize {
        text.len()
    }
}

mod toml {
    pub fn parse(text: &str) -> usize {
        text.lines().count()
    }
}

use json::*;
use toml::*;

fn main() {
    assert_eq!(parse("a = 1\nb = 2"), 2); // KEEP
}
