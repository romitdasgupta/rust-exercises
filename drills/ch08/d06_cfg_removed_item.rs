//! DRILL 08-06 · cfg removes items
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch08`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: does helper exist in this build at all?

#[cfg(feature = "fast")]
fn helper() -> u32 {
    42
}

fn main() {
    let answer = helper();
    assert_eq!(answer, 42); // KEEP
}
