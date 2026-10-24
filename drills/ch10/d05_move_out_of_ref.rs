//! DRILL 10-05 · Moving a payload out of a reference
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch10`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: who owns the String inside `*slot`?

fn name_or_default(slot: &Option<String>) -> String {
    match *slot {
        Some(name) => name,
        None => String::from("anonymous"),
    }
}

fn main() {
    let slot = Some(String::from("ana"));
    assert_eq!(name_or_default(&slot), "ana"); // KEEP
    assert!(slot.is_some()); // KEEP
}
