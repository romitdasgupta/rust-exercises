//! DRILL 16-02 · Updating a map through an index
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch16`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: which trait would `counts["a"] += 1` need, and does HashMap implement it?

use std::collections::HashMap;

fn main() {
    let mut counts = HashMap::from([("a", 1)]);
    counts["a"] += 1;
    assert_eq!(counts["a"], 2); // KEEP
}
