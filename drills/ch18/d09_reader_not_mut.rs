//! DRILL 18-09 · Reading needs a mutable reader
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch18`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: what does a read change inside the reader?

use std::io::Read;

fn first_byte<R: Read>(input: R) -> u8 {
    let mut byte = [0u8; 1];
    input.read_exact(&mut byte).unwrap();
    byte[0]
}

fn main() {
    assert_eq!(first_byte(&b"xyz"[..]), b'x'); // KEEP
}
