//! DRILL 24-10 · Returning a borrowed C string
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch24`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: who owns the bytes after make_name returns?

use std::ffi::{CStr, CString};

fn make_name(label: &str) -> &CStr {
    let owned = CString::new(label).unwrap();
    owned.as_c_str()
}

fn main() {
    assert_eq!(make_name("repo").to_bytes(), b"repo"); // KEEP
}
