//! DRILL 19-02 · Arc does not make RefCell thread-safe
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch19`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: which trait lets two threads share `&T`?

use std::cell::RefCell;
use std::sync::Arc;
use std::thread;

fn main() {
    let log = Arc::new(RefCell::new(Vec::new()));
    let writer = Arc::clone(&log);
    thread::spawn(move || writer.borrow_mut().push("worker"))
        .join()
        .unwrap();
    let entries = log.borrow().clone();
    assert_eq!(entries, ["worker"]); // KEEP
}
