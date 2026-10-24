//! DRILL 16-03 · Sorting floats
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch16`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: what does sort need to know about every pair of values, and what about NaN?

fn main() {
    let mut readings = vec![2.5, -1.0, 0.5];
    readings.sort();
    assert_eq!(readings, [-1.0, 0.5, 2.5]); // KEEP
}
