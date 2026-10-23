//! DRILL 13-04 · Moving out through Deref
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch13`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: does deref give you the Vec, or a reference to it?

use std::ops::Deref;

struct Samples(Vec<i32>);

impl Deref for Samples {
    type Target = Vec<i32>;
    fn deref(&self) -> &Vec<i32> {
        &self.0
    }
}

fn main() {
    let samples = Samples(vec![3, 1, 2]);
    let inner: Vec<i32> = *samples;
    assert_eq!(inner.len(), 3); // KEEP
}
