//! DRILL 22-10 · Leaves need the trait the expansion uses
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch22`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: which trait does `{}` require of every argument?

macro_rules! show {
    ($($x:expr),*) => {
        vec![$(format!("{}", $x)),*]
    };
}

fn main() {
    let s = show!(1, vec![2, 3]); // KEEP
    assert_eq!(s, ["1", "[2, 3]"]); // KEEP
}
