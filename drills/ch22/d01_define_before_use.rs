//! DRILL 22-01 · Define before use
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch22`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: where does a macro_rules! name start being visible?

fn main() {
    let total = twice!(21);
    assert_eq!(total, 42); // KEEP
}

macro_rules! twice {
    ($x:expr) => {
        $x * 2
    };
}
