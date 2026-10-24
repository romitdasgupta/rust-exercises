//! DRILL 17-06 · Returning a borrow of a temporary
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch17`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: who owns the lowercase String after the function returns?

fn normalize(input: &str) -> &str {
    input.trim().to_lowercase().as_str()
}

fn main() {
    assert_eq!(normalize("  Rust "), "rust"); // KEEP
}
