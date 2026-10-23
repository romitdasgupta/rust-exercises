//! DRILL 19-08 · Condvar::wait takes the guard
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch19`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: what does `wait` do with the guard you pass it?

use std::sync::{Condvar, Mutex};

fn main() {
    let ready = Mutex::new(true);
    let changed = Condvar::new();
    let guard = ready.lock().unwrap();
    while !*guard {
        changed.wait(guard).unwrap();
    }
    assert!(*guard); // KEEP
}
