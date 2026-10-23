//! DRILL 03-09 · Slicing inside a character
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch03`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: how many bytes does ñ take in UTF-8?

fn main() {
    let word = "ñandú";
    let first = &word[..1];
    assert_eq!(first, "ñ"); // KEEP
}
