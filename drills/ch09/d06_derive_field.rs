//! DRILL 09-06 · Derives need every field
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch09`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: what does a derived `==` call on each field?

struct Celsius(f64);

#[derive(PartialEq)]
struct Reading {
    sensor: u8,
    temperature: Celsius,
}

fn main() {
    let a = Reading { sensor: 1, temperature: Celsius(20.5) };
    let b = Reading { sensor: 1, temperature: Celsius(20.5) };
    assert!(a == b); // KEEP
}
