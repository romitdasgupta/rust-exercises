//! DRILL 16-09 · contains on a Vec<String>
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch16`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: what type does Vec<T>::contains take, and does &str coerce to it?

fn main() {
    let names: Vec<String> = vec!["ada".into(), "grace".into()];
    let found = names.contains("grace");
    assert!(found); // KEEP
}
