//! DRILL 18-06 · A byte slice is a reader
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch18`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: what happens to `input` each time you read from it?

use std::io::Read;

fn main() {
    let mut input: &[u8] = b"abc";
    let mut first = [0u8; 2];
    input.read_exact(&mut first).unwrap();
    assert_eq!(&first, b"ab"); // KEEP
    assert_eq!(input, b"c"); // KEEP
}
