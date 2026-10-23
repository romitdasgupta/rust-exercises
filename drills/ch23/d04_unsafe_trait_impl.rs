//! DRILL 23-04 · Implementing an unsafe trait
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch23`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: who must uphold Plain's promise, and how do they say so?

/// # Safety
/// Every bit pattern of the implementing type must be a valid value.
unsafe trait Plain: Copy {}

impl Plain for u32 {}

fn zeroed<T: Plain>() -> T {
    // SAFETY: T: Plain promises that all-zero bits are a valid T.
    unsafe { std::mem::zeroed() }
}

fn main() {
    assert_eq!(zeroed::<u32>(), 0); // KEEP
}
