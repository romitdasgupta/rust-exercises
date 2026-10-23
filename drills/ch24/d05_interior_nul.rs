//! DRILL 24-05 · An interior NUL
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch24`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: where would a C reader think this string ends?

use std::ffi::CString;

fn main() {
    let arg = CString::new("path\0with nul").unwrap();
    assert_eq!(arg.as_bytes().len(), 13); // KEEP
}
