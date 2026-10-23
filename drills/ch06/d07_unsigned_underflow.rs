//! DRILL 06-07 · Unsigned subtraction
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch06`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: can a u8 hold the result of 2 - 3?

fn main() {
    let used: u8 = "3".parse().unwrap();
    let capacity: u8 = 2;
    let left = capacity - used;
    assert_eq!(left, 0); // KEEP
}
