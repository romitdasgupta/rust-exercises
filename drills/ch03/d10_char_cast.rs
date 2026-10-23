//! DRILL 03-10 · Casting an integer to char
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch03`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: is every u32 a valid Unicode scalar value?

fn main() {
    let code: u32 = 0x1F980;
    let crab = code as char;
    assert_eq!(crab, '🦀'); // KEEP
}
