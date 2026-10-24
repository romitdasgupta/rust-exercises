//! DRILL 12-08 · PartialOrd builds on PartialEq
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch12`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: what must be true before < can be defined?

use std::cmp::Ordering;

struct Version(u32, u32);

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some((self.0, self.1).cmp(&(other.0, other.1)))
    }
}

fn main() {
    assert!(Version(1, 2) < Version(1, 10)); // KEEP
}
