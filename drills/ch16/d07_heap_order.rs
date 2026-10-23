//! DRILL 16-07 · The heap's default order
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch16`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: is BinaryHeap a min-heap or a max-heap?

use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    let mut deadlines = BinaryHeap::from(vec![30, 10, 20]);
    let soonest = deadlines.pop().unwrap();
    assert_eq!(soonest, 10); // KEEP
}
