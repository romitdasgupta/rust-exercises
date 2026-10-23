//! DRILL 03-04 · Dividing a float by zero
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch03`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: does IEEE floating point treat division by zero like integers do?

fn ratio(part: f64, whole: f64) -> f64 {
    part / whole
}

fn main() {
    let r = ratio(1.0, 0.0);
    assert!(r.is_infinite() && r > 0.0); // KEEP
    assert!(ratio(0.0, 0.0).is_nan()); // KEEP
}
