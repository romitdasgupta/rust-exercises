//! LESSON 07 — Capstone: pinned future adapter
//! Run: cargo test --test chapter21 l07_
//!
//! PREDICT before running:
//! 1. Does Pin<Box<F>> require F: Unpin?
//! 2. Can a wrapper move a pinned Box handle while preserving the future address?
//!
//! IMPLEMENT the tasks; keep the tests.
//! EXPLAIN: Explain how storing Pin<Box<F>> lets a wrapper project safely without handwritten unsafe code. Relate this to future construction and first poll.
//! COMPILER LAB (examples/ch21_scratch.rs): Build an async future that holds a reference across await; try requiring Unpin. Then store it with Box::pin and use the adapter.

use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
struct WithTag<F: Future> {
    inner: Pin<Box<F>>,
    tag: Option<String>,
}
/// A. Pin an arbitrary future on the heap; no Unpin bound on F.
fn tagged<F: Future>(future: F, tag: String) -> WithTag<F> {
    todo!("A: implement the contract")
}
impl<F: Future> Future for WithTag<F> {
    type Output = (String, F::Output);
    /// B. Poll the inner future in place. Preserve tag across Pending; take it
    /// only on Ready, returning (tag, output). Polling after Ready may panic.
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        todo!("B: implement the contract")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn ready<F: std::future::Future>(future: F) -> F::Output {
        let mut f = std::pin::pin!(future);
        let mut cx = std::task::Context::from_waker(std::task::Waker::noop());
        match f.as_mut().poll(&mut cx) {
            std::task::Poll::Ready(x) => x,
            std::task::Poll::Pending => panic!("expected immediately ready future"),
        }
    }
    #[test]
    fn adapter_accepts_a_non_unpin_async_future() {
        let f = async {
            let s = String::from("borrowed");
            let r = &s;
            std::future::ready(()).await;
            r.len()
        };
        assert_eq!(ready(tagged(f, "task".into())), ("task".into(), 8));
    }
    #[test]
    fn pending_preserves_tag_and_inner_address() {
        let mut f = tagged(std::future::pending::<()>(), "waiting".into());
        let address = &*f.inner as *const _;
        assert!(
            Pin::new(&mut f)
                .poll(&mut Context::from_waker(std::task::Waker::noop()))
                .is_pending()
        );
        assert_eq!(f.tag.as_deref(), Some("waiting"));
        let moved = f;
        assert_eq!(&*moved.inner as *const _, address);
    }
}
