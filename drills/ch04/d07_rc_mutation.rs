//! DRILL 04-07 · Shared ownership is shared, not mutable
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch04`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: what does Rc promise every other owner?

use std::rc::Rc;

fn main() {
    let config = Rc::new(String::from("zone=a"));
    let other = Rc::clone(&config);
    config.push_str(";debug");
    assert_eq!(*other, "zone=a"); // KEEP
}
