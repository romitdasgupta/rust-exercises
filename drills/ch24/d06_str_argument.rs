//! DRILL 24-06 · A &str is not a C string
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch24`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: is a Rust string slice NUL-terminated?

use std::ffi::{CStr, c_char};

fn length(s: *const c_char) -> usize {
    // SAFETY: callers pass a live, NUL-terminated string.
    unsafe { CStr::from_ptr(s) }.to_bytes().len()
}

fn main() {
    let name = "ferris";
    assert_eq!(length(name), 6); // KEEP
}
