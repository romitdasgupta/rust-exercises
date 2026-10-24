//! LESSON 03 — ABI calls and out-parameters
//! Run: cargo test --test chapter24 l03_
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider: May an output slot be read after a foreign function returns failure? Does extern C by itself guarantee a function is safe to call?
//! IMPLEMENT the tasks below. Preserve the tests and add an edge case.
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
    use practice::guess;
    #[test]
    fn a_success_controls_output_initialization() {
        assert_eq!(divide(7, 2), Ok(3));
        assert_eq!(divide(-7, 2), Ok(-3));
        assert_eq!(divide(1, 0), Err(-1));
        assert_eq!(divide(c_int::MIN, -1), Err(-1));
    }
    #[test]
    fn a_zero_and_extreme_quotients() {
        assert_eq!(divide(0, 5), Ok(0), "0 is a valid result, not a failure");
        assert_eq!(divide(c_int::MIN, 1), Ok(c_int::MIN));
        assert_eq!(divide(c_int::MAX, -1), Ok(-c_int::MAX));
    }
    #[test]
    fn b_function_pointer_preserves_abi() {
        assert_eq!(operation(true)(7, 3), 10);
        assert_eq!(operation(false)(7, 3), 4);
        assert_eq!(operation(false)(3, 7), -4, "order of operands matters");
        assert_eq!(operation(true)(c_int::MAX, 1), c_int::MIN, "wrapping add");
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    #[test]
    fn p_predictions() {
        // P1. Integer division truncates toward zero (as in C). Quotient,
        // remainder, and the Euclidean quotient for -7 and 2.
        let p1: (i32, i32, i32) = guess();
        assert_eq!(
            p1,
            ((-7) / 2, (-7) % 2, (-7_i32).div_euclid(2)),
            "P1: division"
        );

        // P2. The output slot is initialized here, so reading it is safe.
        // After a failed division and after a null output: statuses and slot.
        let mut slot: c_int = 99;
        // SAFETY: slot is a live, aligned c_int; null is checked by the callee.
        let failed = unsafe { foreign_divide(1, 0, &mut slot) };
        // SAFETY: a null output is allowed; the callee checks it.
        let null = unsafe { foreign_divide(8, 2, std::ptr::null_mut()) };
        let p2: (c_int, c_int, c_int) = guess(); // (failed, slot, null)
        assert_eq!(p2, (failed, slot, null), "P2: status and output slot");
    }
}
