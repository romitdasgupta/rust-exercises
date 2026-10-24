//! DRILL 23-02 · Making pointers is safe
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch23`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: is computing a far-away address undefined behavior by itself?

fn main() {
    let values = [10, 20, 30];
    let first = values.as_ptr();
    let far = first.wrapping_add(1_000);
    assert!(!far.is_null()); // KEEP
    assert_ne!(first, far); // KEEP
}
