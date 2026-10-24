//! DRILL 23-07 · transmute between sizes
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch23`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: how many bytes does each side have?

fn main() {
    let bits: u64 = unsafe { std::mem::transmute(1.5_f32) };
    assert_eq!(bits, 1.5_f32.to_bits() as u64); // KEEP
}
