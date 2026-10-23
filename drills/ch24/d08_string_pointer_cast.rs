//! DRILL 24-08 · Casting a String reference
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch24`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: what does `&name` point at: the text, or the String's header?

use std::ffi::{CStr, c_char};

fn main() {
    let name = String::from("ferris\0");
    let p = &name as *const c_char;
    // SAFETY: p must point at NUL-terminated bytes that outlive `back`.
    let back = unsafe { CStr::from_ptr(p) };
    assert_eq!(back.to_bytes(), b"ferris"); // KEEP
}
