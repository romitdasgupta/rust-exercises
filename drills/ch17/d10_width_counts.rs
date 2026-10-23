//! DRILL 17-10 · Width counts what?
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch17`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: does `{:<5}` pad to 5 bytes, 5 scalars, or 5 screen columns?

fn main() {
    let cell = format!("{:<5}|", "é"); // KEEP
    let width = cell.len();
    assert_eq!(width, 6); // KEEP
}
