//! DRILL 03-05 · An integer is not a condition
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch03`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: what type must an if condition have?

fn main() {
    let pending = 3;
    let mut checked = false;
    if pending {
        checked = true;
    }
    assert!(checked); // KEEP
}
