//! DRILL 21-10 · Mutating through Pin<&mut Self>
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch21`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: which binding needs to be mutable?

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, Waker};

struct Retries(u8);

impl Future for Retries {
    type Output = u8;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<u8> {
        if self.0 == 3 {
            return Poll::Ready(3);
        }
        self.0 += 1;
        cx.waker().wake_by_ref();
        Poll::Pending
    }
}

fn main() {
    let mut cx = Context::from_waker(Waker::noop());
    let mut f = Retries(0);
    let polls = (1..=10).find(|_| Pin::new(&mut f).poll(&mut cx).is_ready());
    assert_eq!(polls, Some(4)); // KEEP
}
