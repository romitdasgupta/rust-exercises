//! LESSON 04 — Callbacks and userdata
//! Run: cargo test --test chapter24 l04_
//!
//! PREDICT before running:
//! 1. How long must a stack-based userdata value remain alive?
//! 2. What happens if a Rust panic crosses an extern C boundary?
//!
//! IMPLEMENT the tasks; keep the tests.
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
    #[test]
    fn userdata_lives_through_all_callbacks() {
        assert_eq!(scaled(&[2, -3, 0], 4), [8, -12, 0]);
        assert_eq!(scaled(&[c_int::MAX], 2), [c_int::MAX]);
        assert!(scaled(&[], 7).is_empty());
    }
    #[test]
    fn nullable_callback_representation() {
        assert!(optional_callback(false).is_none());
        let callback = optional_callback(true).unwrap();
        let mut factor = 3;
        let result = unsafe { callback(5, (&mut factor as *mut c_int).cast()) };
        assert_eq!(result, 15);
    }
}
