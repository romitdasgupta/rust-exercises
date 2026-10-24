//! DRILL 19-05 · Mutating through a non-mut Mutex
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch19`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: where does the permission to mutate come from?

use std::sync::Mutex;

fn main() {
    let counter = Mutex::new(0);
    *counter.lock().unwrap() += 1;
    *counter.lock().unwrap() += 1;
    assert_eq!(*counter.lock().unwrap(), 2); // KEEP
}
