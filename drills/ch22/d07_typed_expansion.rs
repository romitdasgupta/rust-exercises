//! DRILL 22-07 · The expansion is still type-checked
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch22`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: what is the element type of the vec! this expands to?

macro_rules! row {
    ($($x:expr),*) => {
        vec![$($x),*]
    };
}

fn main() {
    let r = row!(1, "two", 3.5); // KEEP
    assert_eq!(r, ["1", "two", "3.5"]); // KEEP
}
