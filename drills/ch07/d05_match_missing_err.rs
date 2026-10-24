//! DRILL 07-05 · Matching a Result
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch07`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: which variants of Result does this match handle?

fn main() {
    let parsed: Result<u8, std::num::ParseIntError> = "12".parse();
    let value = match parsed {
        Ok(n) => n,
    };
    assert_eq!(value, 12); // KEEP
}
