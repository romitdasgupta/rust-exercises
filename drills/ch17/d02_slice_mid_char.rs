//! DRILL 17-02 · Slicing inside a character
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch17`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: is byte 2 the start of a character in "héllo"?

fn main() {
    let word = "héllo";
    let head = &word[..2];
    assert_eq!(head, "hé"); // KEEP
}
