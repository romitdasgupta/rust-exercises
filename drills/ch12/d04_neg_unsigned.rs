//! DRILL 12-04 · Unary minus on an unsigned integer
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch12`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: which integer types implement Neg?

fn main() {
    let speed: u32 = 30;
    let reverse = -speed;
    assert_eq!(reverse + 30, 0); // KEEP
}
