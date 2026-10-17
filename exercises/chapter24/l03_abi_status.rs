//! LESSON 03 — ABI calls and out-parameters
//! Run: cargo test --test chapter24 l03_
//!
//! PREDICT before running:
//! 1. May an output slot be read after a foreign function returns failure?
//! 2. Does extern C by itself guarantee a function is safe to call?
//!
//! IMPLEMENT the tasks; keep the tests.
//! EXPLAIN: Explain success-dependent initialization and why a function-pointer ABI must match exactly.
//! COMPILER LAB (examples/ch24_scratch.rs): Write an unsafe extern C declaration for a local C function in the lab; try an ordinary Rust fn pointer assignment and retain the ABI mismatch diagnostic.

use std::{ffi::c_int, mem::MaybeUninit};
// Supplied ABI scaffold: never panics, writes out only on success, returns -1
// for invalid arithmetic or null output. Does not retain any pointer.
unsafe extern "C" fn foreign_divide(a: c_int, b: c_int, out: *mut c_int) -> c_int {
    let Some(value) = a.checked_div(b) else {
        return -1;
    };
    if out.is_null() {
        return -1;
    }
    // SAFETY: caller supplies a writable aligned output slot on non-null path.
    unsafe {
        out.write(value);
    }
    0
}
/// A. Safe wrapper: supply uninitialized output storage, call ABI function,
/// assume_init only on status 0; map other status to Err(status).
fn divide(a: c_int, b: c_int) -> Result<c_int, c_int> {
    todo!("A: implement the contract")
}
extern "C" fn add(a: c_int, b: c_int) -> c_int {
    a.wrapping_add(b)
}
extern "C" fn subtract(a: c_int, b: c_int) -> c_int {
    a.wrapping_sub(b)
}
/// B. Select an ABI-compatible function pointer; true adds, false subtracts.
fn operation(addition: bool) -> extern "C" fn(c_int, c_int) -> c_int {
    todo!("B: implement the contract")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn success_controls_output_initialization() {
        assert_eq!(divide(7, 2), Ok(3));
        assert_eq!(divide(-7, 2), Ok(-3));
        assert_eq!(divide(1, 0), Err(-1));
        assert_eq!(divide(c_int::MIN, -1), Err(-1));
    }
    #[test]
    fn function_pointer_preserves_abi() {
        assert_eq!(operation(true)(7, 3), 10);
        assert_eq!(operation(false)(7, 3), 4);
    }
}
