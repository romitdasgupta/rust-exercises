//! DRILL 10-02 · A refutable let
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch10`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: what should happen when the value is None?

fn first_word(text: &str) -> &str {
    let Some(word) = text.split_whitespace().next();
    word
}

fn main() {
    assert_eq!(first_word("hello world"), "hello"); // KEEP
}
