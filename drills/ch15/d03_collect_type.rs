//! DRILL 15-03 · collect without a destination
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch15`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: how many collection types could `collect` build here?

fn main() {
    let doubled = (1..4).map(|x| x * 2).collect();
    assert_eq!(doubled, vec![2, 4, 6]); // KEEP
}
