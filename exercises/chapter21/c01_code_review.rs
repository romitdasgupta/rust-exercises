//! CODE REVIEW 01 — A teammate's pull request
//! Run: cargo test --test chapter21 c01_
//! Then: cargo clippy --test chapter21
//!
//! This is the kind of code you will be asked to review at work. It compiles,
//! and some behavior tests pass. Your job:
//! 1. Before running anything, read the code and write a review in NOTES.md:
//!    list every bug, every lost or misdirected wakeup, every wake made while a
//!    lock is held, and every allocation made per poll.
//! 2. Run the tests. Compare what they found with your review.
//! 3. Fix it. Keep the types and signatures; the tests depend on them.
//! 4. Run clippy and resolve its warnings for this file.

use std::{
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll, Waker},
};

struct Shared {
    value: Option<String>,
    waiter: Option<Waker>,
}

/// A. Resolves to the published message, moving it out of the shared state.
struct Message(Arc<Mutex<Shared>>);

impl Future for Message {
    type Output = String;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<String> {
        let mut shared = self.0.lock().unwrap();
        if let Some(value) = &shared.value {
            return Poll::Ready(value.clone());
        }
        if shared.waiter.is_none() {
            shared.waiter = Some(cx.waker().clone());
        }
        Poll::Pending
    }
}

/// B. Store the message and wake the waiting task, if any. Called once.
fn publish(shared: &Arc<Mutex<Shared>>, message: String) {
    let mut guard = shared.lock().unwrap();
    guard.value = Some(message);
    if let Some(waker) = &guard.waiter {
        waker.wake_by_ref();
    }
}

/// C. Sum `items`, adding at most `budget` (> 0) items per poll so that other
/// tasks get a turn. The total fits in u64.
struct ChunkedSum {
    items: Vec<u64>,
    next: usize,
    total: u64,
    budget: usize,
}

impl Future for ChunkedSum {
    type Output = u64;
    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<u64> {
        let remaining: Vec<u64> = self.items[self.next..].to_vec();
        for item in remaining.iter().take(self.budget) {
            self.total += item;
            self.next += 1;
        }
        if self.next == self.items.len() {
            Poll::Ready(self.total)
        } else {
            Poll::Pending
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use practice::alloc::count;
    use std::{
        sync::atomic::{AtomicUsize, Ordering},
        task::Wake,
    };

    struct Counter(AtomicUsize);
    impl Wake for Counter {
        fn wake(self: Arc<Self>) {
            self.0.fetch_add(1, Ordering::Relaxed);
        }
        fn wake_by_ref(self: &Arc<Self>) {
            self.0.fetch_add(1, Ordering::Relaxed);
        }
    }
    fn counted() -> (Arc<Counter>, Waker) {
        let c = Arc::new(Counter(AtomicUsize::new(0)));
        let w = Waker::from(Arc::clone(&c));
        (c, w)
    }
    fn shared() -> Arc<Mutex<Shared>> {
        Arc::new(Mutex::new(Shared {
            value: None,
            waiter: None,
        }))
    }

    #[test]
    fn a_published_message_is_ready() {
        let state = shared();
        publish(&state, "hello".into());
        let mut msg = Message(Arc::clone(&state));
        let out = Pin::new(&mut msg).poll(&mut Context::from_waker(Waker::noop()));
        assert_eq!(out, Poll::Ready("hello".to_string()));
    }

    #[test]
    fn a_latest_waker_is_woken() {
        let state = shared();
        let mut msg = Message(Arc::clone(&state));
        let (first, w1) = counted();
        let (second, w2) = counted();
        assert!(
            Pin::new(&mut msg)
                .poll(&mut Context::from_waker(&w1))
                .is_pending()
        );
        // The executor moved the task and now polls it with a different waker.
        assert!(
            Pin::new(&mut msg)
                .poll(&mut Context::from_waker(&w2))
                .is_pending()
        );
        publish(&state, "news".into());
        assert_eq!(
            (
                first.0.load(Ordering::Relaxed),
                second.0.load(Ordering::Relaxed)
            ),
            (0, 1),
            "the task now listens on the second waker"
        );
    }

    #[test]
    fn a_message_is_moved_not_cloned() {
        let state = shared();
        let message = String::from("a large payload");
        let p = message.as_ptr();
        publish(&state, message);
        let mut msg = Message(Arc::clone(&state));
        match Pin::new(&mut msg).poll(&mut Context::from_waker(Waker::noop())) {
            Poll::Ready(s) => assert_eq!(s.as_ptr(), p, "take the String out instead of cloning"),
            Poll::Pending => panic!("message was published"),
        }
    }

    #[test]
    fn b_wake_happens_after_unlocking() {
        struct Probe(Arc<Mutex<Shared>>, Mutex<Vec<bool>>);
        impl Wake for Probe {
            fn wake(self: Arc<Self>) {
                self.wake_by_ref();
            }
            fn wake_by_ref(self: &Arc<Self>) {
                let free = self.0.try_lock().is_ok();
                self.1.lock().unwrap().push(free);
            }
        }
        let state = shared();
        let probe = Arc::new(Probe(Arc::clone(&state), Mutex::new(Vec::new())));
        let waker = Waker::from(Arc::clone(&probe));
        let mut msg = Message(Arc::clone(&state));
        assert!(
            Pin::new(&mut msg)
                .poll(&mut Context::from_waker(&waker))
                .is_pending()
        );
        publish(&state, "x".into());
        assert_eq!(
            *probe.1.lock().unwrap(),
            [true],
            "a woken task on another thread would immediately block on this lock"
        );
    }

    #[test]
    fn c_sum_is_correct() {
        let mut f = ChunkedSum {
            items: (1..=10).collect(),
            next: 0,
            total: 0,
            budget: 3,
        };
        let mut cx = Context::from_waker(Waker::noop());
        let total = loop {
            if let Poll::Ready(total) = Pin::new(&mut f).poll(&mut cx) {
                break total;
            }
        };
        assert_eq!(total, 55);
    }

    #[test]
    fn c_pending_requests_a_repoll() {
        let mut f = ChunkedSum {
            items: vec![1, 2, 3],
            next: 0,
            total: 0,
            budget: 2,
        };
        let (c, w) = counted();
        assert!(
            Pin::new(&mut f)
                .poll(&mut Context::from_waker(&w))
                .is_pending()
        );
        assert_eq!(
            c.0.load(Ordering::Relaxed),
            1,
            "Pending with no wake: a real executor would never poll this task again"
        );
    }

    #[test]
    fn c_polls_do_not_allocate() {
        let mut f = ChunkedSum {
            items: (0..1000).collect(),
            next: 0,
            total: 0,
            budget: 10,
        };
        let mut cx = Context::from_waker(Waker::noop());
        let (_, allocations) = count(|| {
            for _ in 0..50 {
                let _ = Pin::new(&mut f).poll(&mut cx);
            }
        });
        assert_eq!(allocations, 0, "50 polls cost {allocations} allocations");
    }
}
