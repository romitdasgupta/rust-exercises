//! DRILL 24-03 · Safe items in an unsafe extern block
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch24`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: what does `safe fn` inside the block promise, and who promises it?

use std::ffi::c_int;

unsafe extern "C" {
    safe fn abs(x: c_int) -> c_int;
}

fn main() {
    assert_eq!(abs(-5), 5); // KEEP
}
