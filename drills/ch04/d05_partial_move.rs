//! DRILL 04-05 · Partial moves
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch04`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: after moving one field, which parts are still usable?

fn main() {
    let pair = (String::from("left"), String::from("right"));
    let left = pair.0;
    assert_eq!(pair.1, "right"); // KEEP
    assert_eq!(left, "left"); // KEEP
}
