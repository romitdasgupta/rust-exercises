//! DRILL 03-02 · No implicit widening
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch03`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: does Rust ever convert between numeric types for you?

fn main() {
    let small: u16 = 300;
    let big: u32 = small;
    assert_eq!(big, 300); // KEEP
}
