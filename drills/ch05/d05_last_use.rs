//! DRILL 05-05 · A borrow ends at its last use
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch05`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: does a borrow last until the closing brace?

fn main() {
    let mut log = String::from("start");
    let view = &log;
    let length = view.len();
    log.push_str(" end");
    assert_eq!((length, log.as_str()), (5, "start end")); // KEEP
}
