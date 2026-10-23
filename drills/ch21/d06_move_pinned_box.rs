//! DRILL 21-06 · Moving a pinned box
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch21`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: does moving the Box move the future inside it?

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, Waker};

fn main() {
    let mut cx = Context::from_waker(Waker::noop());
    let job: Pin<Box<dyn Future<Output = u8>>> = Box::pin(async { 7 });
    let address = &*job as *const dyn Future<Output = u8> as *const u8;
    let mut moved = job;
    let after = &*moved as *const dyn Future<Output = u8> as *const u8;
    assert_eq!(address, after); // KEEP
    assert_eq!(moved.as_mut().poll(&mut cx), Poll::Ready(7)); // KEEP
}
