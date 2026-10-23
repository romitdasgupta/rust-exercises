//! DRILL 12-03 · Eq for a struct holding a float
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch12`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: is f64 equality reflexive?

#[derive(Debug, PartialEq, Eq)]
struct Reading {
    sensor: u8,
    celsius: f64,
}

fn main() {
    let a = Reading { sensor: 1, celsius: 20.5 };
    assert_eq!(a, Reading { sensor: 1, celsius: 20.5 }); // KEEP
}
