//! LESSON 07 — Capstone: checked foreign transaction
//! Run: cargo test --test chapter24 l07_
//!
//! PREDICT before running:
//! 1. Must foreign error text be copied before another library call?
//! 2. Can a failed out-parameter be inspected for debugging?
//!
//! IMPLEMENT the tasks; keep the tests.
//! EXPLAIN: Audit arguments, status, initialized output, owned error copying, and no unwinding across the ABI. Explain which real libgit2 obligations remain in LABS.md.
//! COMPILER LAB (examples/ch24_scratch.rs): Add a second call after an error and show that the already-copied Rust error remains valid. Design an RAII session that initializes a library before handles and shuts it down after all handles.

use std::{
    ffi::{CStr, CString, c_char, c_int},
    mem::MaybeUninit,
};
// Supplied typed ABI mock: on success initializes out; on failure writes a
// borrowed static C error pointer to error, leaving out UNINITIALIZED.
// # Safety: name is a valid NUL-terminated string; out/error are writable aligned
// slots. No pointer is retained. The error string must be copied by the wrapper.
unsafe extern "C" fn foreign_score(
    name: *const c_char,
    weight: c_int,
    out: *mut c_int,
    error: *mut *const c_char,
) -> c_int {
    // SAFETY: caller supplies valid argument storage for the synchronous call.
    let name = unsafe { CStr::from_ptr(name) };
    let result = c_int::try_from(name.to_bytes().len())
        .ok()
        .and_then(|n| n.checked_mul(weight));
    if weight < 0 || result.is_none() {
        // SAFETY: error is a writable pointer slot.
        unsafe {
            error.write(c"invalid weight".as_ptr());
        }
        return -1;
    }
    // SAFETY: success writes the output slot exactly once.
    unsafe {
        out.write(result.unwrap());
    }
    0
}
/// A. Validate a Rust name as CString (NUL error => "interior NUL"), call the
/// foreign function with MaybeUninit output/error slots, and inspect status.
/// Success reads only out; failure reads/copies only error into an owned String.
fn score(name: &str, weight: c_int) -> Result<c_int, String> {
    todo!("A: implement the contract")
}
/// B. Compute scores sequentially; stop at first error and return it with the
/// failing input index. Success returns every score in order.
fn batch(items: &[(&str, c_int)]) -> Result<Vec<c_int>, (usize, String)> {
    todo!("B: implement the contract")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn boundary_checks_every_representation() {
        assert_eq!(score("é", 3), Ok(6));
        assert_eq!(score("", 9), Ok(0));
        assert_eq!(score("bad\0name", 2), Err("interior NUL".into()));
        let error = score("abc", -1).unwrap_err();
        assert_eq!(score("ok", 4), Ok(8));
        assert_eq!(error, "invalid weight");
        assert_eq!(score("abc", c_int::MAX), Err("invalid weight".into()));
    }
    #[test]
    fn batch_reports_index_and_stops() {
        assert_eq!(batch(&[("a", 2), ("bb", 3)]), Ok(vec![2, 6]));
        assert_eq!(
            batch(&[("a", 2), ("b", -1), ("c", 4)]),
            Err((1, "invalid weight".into()))
        );
        assert_eq!(batch(&[]), Ok(vec![]));
    }
}
