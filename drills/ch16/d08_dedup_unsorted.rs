//! DRILL 16-08 · dedup on unsorted data
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch16`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: which duplicates does dedup look for?

fn main() {
    let mut tags = vec!["red", "blue", "red", "blue"];
    tags.dedup();
    assert_eq!(tags.len(), 2); // KEEP
}
