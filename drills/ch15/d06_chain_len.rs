//! DRILL 15-06 · The length of a chain
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch15`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: can a chain of two exact-size iterators always report its length?

fn main() {
    let ids = (0..3).chain(10..12);
    let n = ids.len();
    assert_eq!(n, 5); // KEEP
}
