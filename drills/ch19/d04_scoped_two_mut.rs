//! DRILL 19-04 · Two scoped workers, one mutable slice
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch19`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: can two threads hold `&mut` to the same slice?

use std::thread;

fn main() {
    let mut pixels = [0u8; 6];
    thread::scope(|s| {
        s.spawn(|| pixels[..3].fill(1));
        s.spawn(|| pixels[3..].fill(2));
    });
    assert_eq!(pixels, [1, 1, 1, 2, 2, 2]); // KEEP
}
