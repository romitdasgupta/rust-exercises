//! DRILL 06-06 · Chained comparisons
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch06`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: how would Rust parse a < b < c?

fn main() {
    let (low, value, high) = (1, 5, 9);
    let inside = low < value < high;
    assert!(inside); // KEEP
}
