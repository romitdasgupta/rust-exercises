//! DRILL 11-05 · impl Trait is one concrete type
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch11`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: how many types can hide behind one impl Iterator?

fn numbers(descending: bool) -> impl Iterator<Item = u32> {
    if descending {
        (1..=3).rev()
    } else {
        1..=3
    }
}

fn main() {
    assert_eq!(numbers(true).collect::<Vec<_>>(), [3, 2, 1]); // KEEP
    assert_eq!(numbers(false).collect::<Vec<_>>(), [1, 2, 3]); // KEEP
}
