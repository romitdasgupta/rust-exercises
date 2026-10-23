//! DRILL 06-02 · The semicolon that changes a type
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch06`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: what is the type of a block whose last line ends in a semicolon?

fn doubled(n: i32) -> i32 {
    let twice = n * 2;
    twice;
}

fn main() {
    assert_eq!(doubled(21), 42); // KEEP
}
