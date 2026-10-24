//! DRILL 15-04 · What filter's closure receives
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch15`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: what is the item type of `iter()`, and what does `filter` pass?

fn main() {
    let scores = vec![4, 9, 1, 7];
    let high: Vec<&i32> = scores.iter().filter(|s| s > 5).collect();
    assert_eq!(high, [&9, &7]); // KEEP
}
