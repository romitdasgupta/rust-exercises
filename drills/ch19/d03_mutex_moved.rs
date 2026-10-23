//! DRILL 19-03 · A Mutex moved into the first worker
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch19`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: how many owners can a plain Mutex have?

use std::sync::Mutex;
use std::thread;

fn main() {
    let total = Mutex::new(0);
    let a = thread::spawn(move || *total.lock().unwrap() += 1);
    let b = thread::spawn(move || *total.lock().unwrap() += 1);
    a.join().unwrap(); // KEEP
    b.join().unwrap(); // KEEP
}
