//! DRILL 10-01 · A new variant
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch10`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: what does the compiler check about every match on this enum?

enum Status {
    Active,
    Suspended,
    Archived,
}

fn label(status: &Status) -> &'static str {
    match status {
        Status::Active => "active",
        Status::Suspended => "suspended",
    }
}

fn main() {
    assert_eq!(label(&Status::Archived), "archived"); // KEEP
}
