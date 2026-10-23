//! DRILL 18-01 · Read methods need the trait in scope
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch18`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: where is `read_to_string` defined?

use std::io::Cursor;

fn main() {
    let mut input = Cursor::new("config");
    let mut text = String::new();
    input.read_to_string(&mut text).unwrap();
    assert_eq!(text, "config"); // KEEP
}
