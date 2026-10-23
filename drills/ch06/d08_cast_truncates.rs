//! DRILL 06-08 · Casts never panic
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch06`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: what does as do with a value that does not fit?

fn main() {
    let big: i32 = 300;
    let low = big as u8;
    let negative = -1i32 as u32;
    assert_eq!(low, 44); // KEEP
    assert_eq!(negative, u32::MAX); // KEEP
}
