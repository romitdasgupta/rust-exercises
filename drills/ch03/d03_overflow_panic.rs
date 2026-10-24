//! DRILL 03-03 · Plain addition overflows
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch03`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: what does a debug build do when a u8 sum exceeds 255?

fn brighten(level: u8, amount: u8) -> u8 {
    level + amount
}

fn main() {
    assert_eq!(brighten(100, 50), 150); // KEEP
    assert_eq!(brighten(250, 10), 255); // KEEP
}
