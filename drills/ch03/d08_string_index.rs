//! DRILL 03-08 · Indexing a String
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch03`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: what unit would s[0] count in?

fn main() {
    let word = String::from("hello");
    let first = word[0];
    assert_eq!(first, 'h'); // KEEP
}
