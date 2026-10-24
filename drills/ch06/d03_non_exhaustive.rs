//! DRILL 06-03 · Matching on an integer
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch06`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: which u8 values does this match not cover?

fn level(code: u8) -> &'static str {
    match code {
        0 => "off",
        1..=9 => "low",
        10..=99 => "high",
    }
}

fn main() {
    assert_eq!(level(5), "low"); // KEEP
    assert_eq!(level(200), "invalid"); // KEEP
}
