//! LESSON 04 — Callbacks and userdata
//! Run: cargo test --test chapter24 l04_
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider: How long must a stack-based userdata value remain alive? What happens if a Rust panic crosses an extern C boundary?
//! IMPLEMENT the tasks below. Preserve the tests and add an edge case.
//! EXPLAIN: Explain callback lifetime, pointer retention, and why this provided callback uses nonpanicking saturation. The extension lab handles arbitrary closure panics.
//! COMPILER LAB (examples/ch24_scratch.rs): Change a local callback type to an ordinary Rust fn pointer and inspect the mismatch. Draw the synchronous call/lifetime boundary.

use std::ffi::{c_int, c_void};
type Callback = unsafe extern "C" fn(c_int, *mut c_void) -> c_int;
// # Safety: input/output cover len readable/writable c_int elements, disjoint;
// callback and ctx remain valid for the synchronous call; no pointers retained.
unsafe extern "C" fn foreign_map(
    input: *const c_int,
    len: usize,
    output: *mut c_int,
    callback: Callback,
    ctx: *mut c_void,
) {
    for index in 0..len {
        // SAFETY: caller supplies bounded arrays and callable context.
        unsafe {
            output
                .add(index)
                .write(callback(input.add(index).read(), ctx));
        }
    }
}
// # Safety: ctx points to a live, aligned, initialized c_int. No retention.
unsafe extern "C" fn scale(value: c_int, ctx: *mut c_void) -> c_int {
    // SAFETY: foreign_map's caller supplies the borrowed factor for this call.
    value.saturating_mul(unsafe { ctx.cast::<c_int>().read() })
}
/// A. Allocate output, pass a stack factor as userdata, and invoke foreign_map.
/// Keep input, output, and factor alive for the whole synchronous call.
fn scaled(values: &[c_int], factor: c_int) -> Vec<c_int> {
    todo!("A: implement the contract")
}
/// B. Return an optional callback: None when disabled, Some(scale) otherwise.
/// Option<extern fn> is the standard nullable-function-pointer representation.
fn optional_callback(enabled: bool) -> Option<Callback> {
    todo!("B: implement the contract")
}

#[cfg(test)]
mod tests {
    use super::*;
    use practice::{alloc::count, guess};
    #[test]
    fn a_userdata_lives_through_all_callbacks() {
        assert_eq!(scaled(&[2, -3, 0], 4), [8, -12, 0]);
        assert_eq!(scaled(&[c_int::MAX], 2), [c_int::MAX]);
        assert!(scaled(&[], 7).is_empty());
    }
    #[test]
    fn a_saturates_in_both_directions() {
        assert_eq!(scaled(&[c_int::MIN, 5, -1], -1), [c_int::MAX, -5, 1]);
        assert_eq!(scaled(&[c_int::MIN], 2), [c_int::MIN]);
    }
    #[test]
    fn a_output_is_allocated_once() {
        let input: Vec<c_int> = (0..100).collect();
        let (out, allocations) = count(|| scaled(&input, 3));
        assert_eq!(out[99], 297);
        assert!(
            allocations <= 1,
            "one output buffer for 100 values; got {allocations} allocations"
        );
    }
    #[test]
    fn b_nullable_callback_representation() {
        assert!(optional_callback(false).is_none());
        let callback = optional_callback(true).unwrap();
        let mut factor = 3;
        // SAFETY: ctx points to a live c_int for the duration of the call.
        let result = unsafe { callback(5, (&mut factor as *mut c_int).cast()) };
        assert_eq!(result, 15);
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    #[test]
    fn p_predictions() {
        // P1. Sizes on a 64-bit target: Option<Callback> and Callback.
        let p1: (usize, usize) = guess();
        assert_eq!(
            p1,
            (size_of::<Option<Callback>>(), size_of::<Callback>()),
            "P1: nullable function pointers"
        );

        // P2. Saturating versus wrapping multiplication at the edge.
        let p2: (c_int, c_int) = guess();
        assert_eq!(
            p2,
            (c_int::MAX.saturating_mul(-2), c_int::MAX.wrapping_mul(2)),
            "P2: overflow policies"
        );

        // P3. A callback that counts its calls through the userdata pointer.
        // How many calls for three inputs, and what does it return each time?
        unsafe extern "C" fn tally(_: c_int, ctx: *mut c_void) -> c_int {
            // SAFETY: ctx is the live, exclusive counter passed below.
            let calls = unsafe { &mut *ctx.cast::<c_int>() };
            *calls += 1;
            *calls
        }
        let input = [7, 7, 7];
        let mut output = [0; 3];
        let mut calls: c_int = 0;
        // SAFETY: both arrays hold 3 elements; calls outlives the call.
        unsafe {
            foreign_map(
                input.as_ptr(),
                input.len(),
                output.as_mut_ptr(),
                tally,
                (&mut calls as *mut c_int).cast(),
            )
        };
        let p3: (c_int, [c_int; 3]) = guess();
        assert_eq!(p3, (calls, output), "P3: userdata across calls");
    }
}
