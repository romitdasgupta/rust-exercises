//! DRILL 08-07 · What cfg hides is not type-checked
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch08`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: does the compiler type-check code that cfg removed?

#[cfg(any())]
fn broken() -> u8 {
    "this is not a number"
}

fn main() {
    let enabled = cfg!(any());
    assert!(!enabled); // KEEP
}
