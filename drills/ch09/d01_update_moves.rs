//! DRILL 09-01 · Update syntax moves fields
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch09`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: which fields of `base` did `..base` move?

struct Config {
    name: String,
    path: String,
    retries: u8,
}

fn main() {
    let base = Config {
        name: String::from("base"),
        path: String::from("/srv"),
        retries: 3,
    };
    let custom = Config {
        name: String::from("custom"),
        ..base
    };
    assert_eq!(custom.path, "/srv"); // KEEP
    assert_eq!(base.path, "/srv"); // KEEP
}
