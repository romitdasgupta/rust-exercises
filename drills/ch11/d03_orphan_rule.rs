//! DRILL 11-03 · The orphan rule
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch11`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: who owns Display, and who owns Vec?

use std::fmt;

impl fmt::Display for Vec<u32> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} items", self.len())
    }
}

fn main() {
    let ids = vec![4, 8];
    assert_eq!(format!("{}", ids), "2 items"); // KEEP
}
