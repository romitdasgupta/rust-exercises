//! DRILL 19-09 · Reading after a worker panicked
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch19`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: does a panic while locked destroy the data?

use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let items = Arc::new(Mutex::new(vec![1]));
    let worker = Arc::clone(&items);
    let crashed = thread::spawn(move || {
        let mut guard = worker.lock().unwrap();
        guard.push(2);
        panic!("worker failed after pushing");
    })
    .join();
    assert!(crashed.is_err()); // KEEP
    let items = items.lock().unwrap();
    assert_eq!(*items, [1, 2]); // KEEP
}
