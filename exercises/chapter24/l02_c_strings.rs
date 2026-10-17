//! LESSON 02 — C strings and ownership
//! Run: cargo test --test chapter24 l02_
//!
//! PREDICT before running:
//! 1. Can a CString contain an interior NUL?
//! 2. Does CStr::to_str accept non-UTF-8 bytes?
//!
//! IMPLEMENT the tasks; keep the tests.
//! EXPLAIN: Distinguish owned CString, borrowed CStr, raw pointer lifetime, and UTF-8 validation.
//! COMPILER LAB (examples/ch24_scratch.rs): Take as_ptr from a temporary CString in a compile-only example and explain the dangling-pointer risk. Repair by binding the CString owner through the foreign call; do not execute the dangling version.

use std::ffi::{CStr, CString, NulError};
/// A. Make an owned C-compatible argument (Rust retains ownership), rejecting interior NUL before any FFI call.
fn argument(input: &str) -> Result<CString, NulError> {
    todo!("A: implement the contract")
}
/// B. Copy an already-valid borrowed C string to owned Rust text. Return the
/// UTF-8 error instead of lossy conversion; the caller may later drop its owner.
fn owned_text(input: &CStr) -> Result<String, std::str::Utf8Error> {
    todo!("B: implement the contract")
}
/// C. Borrow one C string from a bounded byte buffer, rejecting a missing NUL
/// or bytes after its one final NUL. Do not scan unbounded raw memory.
fn from_buffer(bytes: &[u8]) -> Result<&CStr, std::ffi::FromBytesWithNulError> {
    todo!("C: implement the contract")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn arguments_own_terminated_storage() {
        let s = argument("é").unwrap();
        assert_eq!(s.as_bytes_with_nul(), b"\xc3\xa9\0");
        assert!(argument("a\0b").is_err());
        let text = owned_text(&s).unwrap();
        drop(s);
        assert_eq!(text, "é");
    }
    #[test]
    fn c_bytes_are_not_automatically_unicode() {
        assert!(owned_text(c"\xff").is_err());
        assert_eq!(from_buffer(b"abc\0").unwrap(), c"abc");
        assert!(from_buffer(b"abc").is_err());
        assert!(from_buffer(b"a\0b\0").is_err());
    }
}
