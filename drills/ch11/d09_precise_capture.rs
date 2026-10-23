//! DRILL 11-09 · impl Trait captures every lifetime in scope
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch11`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: in edition 2024, which inputs may the returned iterator borrow?

fn words<'a>(text: &'a str, separator: &str) -> impl Iterator<Item = &'a str> {
    let _ = separator;
    text.split_whitespace()
}

fn main() {
    let text = String::from("air sea land");
    let iter;
    {
        let separator = String::from(" ");
        iter = words(&text, &separator); // KEEP
    }
    assert_eq!(iter.count(), 3); // KEEP
}
