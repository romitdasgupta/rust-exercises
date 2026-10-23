//! DRILL 19-10 · Sending after the receiver is gone
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch19`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: who is left to receive the second message?

use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel::<u32>();
    let worker = thread::spawn(move || rx.recv().unwrap() * 2);
    tx.send(1).unwrap();
    let doubled = worker.join().unwrap(); // KEEP
    tx.send(2).unwrap();
    assert_eq!(doubled, 2); // KEEP
}
