//! DRILL 15-02 · Looping over a vector by value
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch15`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: what does `for name in names` do to `names`?

fn main() {
    let names = vec![String::from("ada"), String::from("grace")];
    let mut total = 0;
    for name in names {
        total += name.len();
    }
    assert_eq!((total, names.len()), (8, 2)); // KEEP
}
