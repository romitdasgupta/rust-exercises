//! LESSON 02 — C strings and ownership
//! Run: cargo test --test chapter24 l02_
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider: Can a CString contain an interior NUL? Does CStr::to_str accept non-UTF-8 bytes?
//! IMPLEMENT the tasks below. Preserve the tests and add an edge case.
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
    use practice::guess;
    #[test]
    fn a_b_arguments_own_terminated_storage() {
        let s = argument("é").unwrap();
        assert_eq!(s.as_bytes_with_nul(), b"\xc3\xa9\0");
        assert!(argument("a\0b").is_err());
        let text = owned_text(&s).unwrap();
        drop(s);
        assert_eq!(text, "é");
    }
    #[test]
    fn a_empty_and_nul_positions() {
        assert_eq!(argument("").unwrap().as_bytes_with_nul(), b"\0");
        assert_eq!(argument("\0").unwrap_err().nul_position(), 0);
        assert_eq!(argument("ab\0").unwrap_err().nul_position(), 2);
    }
    #[test]
    fn b_c_bytes_are_not_automatically_unicode() {
        assert!(owned_text(c"\xff").is_err());
        assert_eq!(owned_text(c"").unwrap(), "");
        assert_eq!(owned_text(c"caf\xc3\xa9").unwrap(), "café");
    }
    #[test]
    fn c_buffer_must_hold_exactly_one_final_nul() {
        assert_eq!(from_buffer(b"abc\0").unwrap(), c"abc");
        assert!(from_buffer(b"abc").is_err());
        assert!(from_buffer(b"a\0b\0").is_err());
        assert!(
            from_buffer(b"abc\0\0").is_err(),
            "bytes after the final NUL"
        );
        assert!(from_buffer(b"").is_err());
        assert_eq!(from_buffer(b"\0").unwrap(), c"");
    }
    #[test]
    fn c_buffer_is_borrowed_not_copied() {
        let bytes = b"borrowed\0".to_vec();
        let view = from_buffer(&bytes).unwrap();
        assert_eq!(view.as_ptr().cast::<u8>(), bytes.as_ptr());
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    #[test]
    fn p_predictions() {
        // P1. Byte lengths: a CString's bytes including its NUL, and a C
        // string literal's bytes excluding it.
        let p1: (usize, usize) = guess();
        assert_eq!(
            p1,
            (
                CString::new("hi").unwrap().as_bytes_with_nul().len(),
                c"hi".to_bytes().len()
            ),
            "P1: where the NUL is counted"
        );

        // P2. Where does CString::new report the interior NUL in "a\0b"?
        let p2: usize = guess();
        assert_eq!(
            p2,
            CString::new("a\0b").unwrap_err().nul_position(),
            "P2: NulError"
        );

        // P3. One buffer, two constructors: what does from_bytes_until_nul
        // return, and does from_bytes_with_nul reject the buffer?
        let buffer = b"ab\0cd\0";
        let until = CStr::from_bytes_until_nul(buffer).unwrap().to_bytes();
        let p3: (&[u8], bool) = guess();
        assert_eq!(
            p3,
            (until, CStr::from_bytes_with_nul(buffer).is_err()),
            "P3: until_nul versus with_nul"
        );
    }
}
