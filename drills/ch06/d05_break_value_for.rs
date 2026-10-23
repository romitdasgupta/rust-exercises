//! DRILL 06-05 · break with a value
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch06`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: which kind of loop can produce a value, and what leaves the function?

fn first_big(readings: &[i32]) -> i32 {
    for &r in readings {
        if r > 10 {
            break r;
        }
    }
    0
}

fn main() {
    assert_eq!(first_big(&[3, 8, 12, 5]), 12); // KEEP
    assert_eq!(first_big(&[1, 2]), 0); // KEEP
}
