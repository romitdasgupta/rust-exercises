//! DRILL 23-06 · Raw pointers across threads
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch23`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: what does the compiler know about who else uses this pointer?

use std::thread;

fn main() {
    let data = Box::new(7);
    let p: *const i32 = &*data;
    let seen = thread::spawn(move || unsafe { *p }).join().unwrap();
    assert_eq!(seen, 7); // KEEP
}
