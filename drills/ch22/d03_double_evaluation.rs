//! DRILL 22-03 · Evaluating an argument twice
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch22`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: how many times does `$x` appear in the expansion?

macro_rules! square {
    ($x:expr) => {
        $x * $x
    };
}

fn main() {
    let mut calls = 0;
    let mut next = || {
        calls += 1;
        calls
    };
    let one = square!(next());
    assert_eq!(one, 1); // KEEP
    assert_eq!(calls, 1); // KEEP
}
