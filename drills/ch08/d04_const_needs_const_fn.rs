//! DRILL 08-04 · A const needs a const fn
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch08`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: can the compiler run this function while compiling?

fn kib(n: usize) -> usize {
    n * 1024
}

const BUFFER: usize = kib(4);

fn main() {
    assert_eq!(BUFFER, 4096); // KEEP
}
