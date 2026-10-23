//! DRILL 22-04 · Grouping of expression fragments
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch22`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: is `$x * 2` pasted as text, as in a C macro?

macro_rules! double {
    ($x:expr) => {
        $x * 2
    };
}

fn main() {
    assert_eq!(double!(1 + 2), 6); // KEEP
}
