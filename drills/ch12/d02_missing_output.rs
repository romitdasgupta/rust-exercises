//! DRILL 12-02 · Add needs an Output
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch12`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: what type does a + b produce, and who says so?

use std::ops::Add;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Meters(f64);

impl Add for Meters {
    fn add(self, rhs: Meters) -> Meters {
        Meters(self.0 + rhs.0)
    }
}

fn main() {
    assert_eq!(Meters(1.5) + Meters(2.0), Meters(3.5)); // KEEP
}
