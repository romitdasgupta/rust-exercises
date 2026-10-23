//! DRILL 21-04 · A Future without an Output
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch21`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: what does the trait require besides poll?

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, Waker};

struct Answer;

impl Future for Answer {
    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<u32> {
        Poll::Ready(42)
    }
}

fn main() {
    let mut cx = Context::from_waker(Waker::noop());
    assert_eq!(Pin::new(&mut Answer).poll(&mut cx), Poll::Ready(42)); // KEEP
}
