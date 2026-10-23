//! DRILL 24-02 · Calling a foreign function
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch24`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: can the compiler check what abs does with its argument?

use std::ffi::c_int;

unsafe extern "C" {
    fn abs(x: c_int) -> c_int;
}

fn main() {
    let v = abs(-5);
    assert_eq!(v, 5); // KEEP
}
