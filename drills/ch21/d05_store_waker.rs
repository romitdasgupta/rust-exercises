//! DRILL 21-05 · Storing the context's waker
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch21`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: who owns the waker that `cx` lends you?

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, Waker};

struct Gate {
    open: bool,
    waiter: Option<Waker>,
}

impl Future for Gate {
    type Output = ();
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        if self.open {
            return Poll::Ready(());
        }
        self.waiter = Some(cx.waker());
        Poll::Pending
    }
}

fn main() {
    let mut cx = Context::from_waker(Waker::noop());
    let mut gate = Gate {
        open: false,
        waiter: None,
    };
    assert!(Pin::new(&mut gate).poll(&mut cx).is_pending()); // KEEP
    assert!(gate.waiter.is_some()); // KEEP
}
