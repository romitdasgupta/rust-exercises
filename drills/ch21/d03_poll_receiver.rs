//! DRILL 21-03 · poll with the wrong receiver
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch21`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: what signature does the trait promise callers?

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, Waker};

struct Countdown(u8);

impl Future for Countdown {
    type Output = &'static str;
    fn poll(&mut self, cx: &mut Context<'_>) -> Poll<&'static str> {
        if self.0 == 0 {
            return Poll::Ready("liftoff");
        }
        self.0 -= 1;
        cx.waker().wake_by_ref();
        Poll::Pending
    }
}

fn main() {
    let mut cx = Context::from_waker(Waker::noop());
    let mut f = Countdown(1);
    assert!(Pin::new(&mut f).poll(&mut cx).is_pending()); // KEEP
    assert_eq!(Pin::new(&mut f).poll(&mut cx), Poll::Ready("liftoff")); // KEEP
}
