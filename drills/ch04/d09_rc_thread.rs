//! DRILL 04-09 · Rc across threads
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch04`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: why is Rc's reference count unsafe to share between threads?

use std::rc::Rc;
use std::thread;

fn main() {
    let shared = Rc::new(vec![1, 2, 3]);
    let worker = Rc::clone(&shared);
    let sum = thread::spawn(move || worker.iter().sum::<i32>()).join().unwrap();
    assert_eq!(sum + shared.len() as i32, 9); // KEEP
}
