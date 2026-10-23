//! DRILL 03-07 · Array lengths are part of the type
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch03`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: when must an array's length be known?

fn zeroed(n: usize) -> usize {
    let buffer = [0_u8; n];
    buffer.len()
}

fn main() {
    assert_eq!(zeroed(5), 5); // KEEP
}
