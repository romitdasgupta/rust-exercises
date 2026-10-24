//! DRILL 06-01 · An if used as a value
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch06`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: what value does the expression have when the condition is false?

fn main() {
    let retries = 3;
    let delay = if retries > 2 { 100 };
    assert_eq!(delay * 2, 200); // KEEP
}
