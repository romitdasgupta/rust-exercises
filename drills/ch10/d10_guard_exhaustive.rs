//! DRILL 10-10 · Guards and exhaustiveness
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch10`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: does the compiler evaluate guards when checking coverage?

fn sign(n: i32) -> &'static str {
    match n {
        x if x < 0 => "negative",
        0 => "zero",
        x if x > 0 => "positive",
    }
}

fn main() {
    assert_eq!(sign(-3), "negative"); // KEEP
    assert_eq!(sign(0), "zero"); // KEEP
    assert_eq!(sign(9), "positive"); // KEEP
}
