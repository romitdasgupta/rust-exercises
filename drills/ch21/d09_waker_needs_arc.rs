//! DRILL 21-09 · A Waker from a plain struct
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch21`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: how does a Waker keep its task alive?

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::task::{Wake, Waker};

struct Flag(AtomicBool);

impl Wake for Flag {
    fn wake(self: Arc<Self>) {
        self.0.store(true, Ordering::Relaxed);
    }
}

fn main() {
    let flag = Flag(AtomicBool::new(false));
    let waker = Waker::from(flag);
    waker.wake_by_ref(); // KEEP
    assert!(flag.0.load(Ordering::Relaxed)); // KEEP
}
