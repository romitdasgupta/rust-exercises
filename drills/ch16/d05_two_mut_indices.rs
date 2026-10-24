//! DRILL 16-05 · Two mutable indices
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch16`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: how would the compiler know that index 0 and index 2 are different places?

fn main() {
    let mut slots = vec![1, 2, 3];
    let a = &mut slots[0];
    let b = &mut slots[2];
    std::mem::swap(a, b);
    assert_eq!(slots, [3, 2, 1]); // KEEP
}
