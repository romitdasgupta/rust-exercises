//! DRILL 11-08 · Sorting floats
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch11`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: what does sort() require, and do floats promise it?

fn main() {
    let mut readings = vec![2.5, -1.0, 9.75];
    readings.sort();
    assert_eq!(readings, [-1.0, 2.5, 9.75]); // KEEP
}
