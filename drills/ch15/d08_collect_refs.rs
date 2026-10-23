//! DRILL 15-08 · Collecting borrowed items into owned ones
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch15`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: what item type does `iter()` produce over a Vec<String>?

fn main() {
    let tags = vec![String::from("rust"), String::from("iter")];
    let copy: Vec<String> = tags.iter().collect();
    assert_eq!(copy, tags); // KEEP
}
