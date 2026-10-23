//! DRILL 06-09 · Initialized on every path?
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch06`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: does every path assign label before it is read?

fn describe(n: i32) -> String {
    let label;
    if n > 0 {
        label = "positive";
    } else if n < 0 {
        label = "negative";
    }
    format!("{n} is {label}")
}

fn main() {
    assert_eq!(describe(4), "4 is positive"); // KEEP
    assert_eq!(describe(0), "0 is zero"); // KEEP
}
