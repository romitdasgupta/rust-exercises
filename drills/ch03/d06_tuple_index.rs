//! DRILL 03-06 · Indexing a tuple with a variable
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch03`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: if each field can have its own type, what type would reading[field] be?

fn main() {
    let reading = (21, "celsius");
    let field = 0;
    let value = reading[field];
    assert_eq!(value, 21); // KEEP
}
