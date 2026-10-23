//! DRILL 03-01 · Calling a method on an untyped literal
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch03`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: which abs method should the compiler pick?

fn main() {
    let distance = (-17).abs();
    assert_eq!(distance, 17_i32); // KEEP
}
