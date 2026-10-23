//! REVIEW 01 — Spaced retrieval: Chapters 16, 19, and 20 inside Chapter 21
//! Run: cargo test --test chapter21 r01_
//!
//! Do this WITHOUT reopening Chapters 16–20 or their hints. Struggling to recall
//! is what makes the knowledge stick; look things up only after a real attempt,
//! and note in NOTES.md anything you had to look up.
//!
//! Each task mixes an earlier idea (boxed futures, threads, queues) with this
//! chapter's hand-written futures and wakers.

use std::{
    collections::VecDeque,
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll, Waker},
    thread,
};

/// A. (Ch20 boxed futures + Ch21 poll) Run every future concurrently. Each poll
/// of `JoinAll` polls every future that has not finished yet; it is Ready with
/// all outputs, in input order, once every future has finished. Never poll a
/// future again after it returned Ready.
struct JoinAll<T> {
    futures: Vec<Pin<Box<dyn Future<Output = T>>>>,
    outputs: Vec<Option<T>>,
}
// The outputs are never pinned, so moving a JoinAll is always fine.
impl<T> Unpin for JoinAll<T> {}

fn join_all<T>(futures: Vec<Pin<Box<dyn Future<Output = T>>>>) -> JoinAll<T> {
    todo!("A: one output slot per future")
}
impl<T> Future for JoinAll<T> {
    type Output = Vec<T>;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Vec<T>> {
        todo!("A: poll each unfinished future once, then check whether all are done")
    }
}

/// B. (Ch19 threads + Ch21 wakers) Run `job` on a new thread. The returned
/// future resolves to its result. The thread stores the result and takes the
/// registered waker under the lock, then wakes it after unlocking. The thread
/// may be detached: it always finishes.
struct Shared<T> {
    result: Option<T>,
    waiter: Option<Waker>,
}
struct Computation<T>(Arc<Mutex<Shared<T>>>);

fn spawn_compute<T, F>(job: F) -> Computation<T>
where
    T: Send + 'static,
    F: FnOnce() -> T + Send + 'static,
{
    todo!("B: share the state with the worker thread")
}
impl<T> Future for Computation<T> {
    type Output = T;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<T> {
        todo!("B: take the result, or register the latest waker")
    }
}

/// C. (Ch16 VecDeque + Ch21 wakers) A first-in, first-out list of waiting tasks.
#[derive(Default)]
struct WaitQueue {
    waiters: VecDeque<(u64, Waker)>,
}
impl WaitQueue {
    /// C1. Register task `id`. A new id joins the back of the queue. An id that
    /// is already queued keeps its place but stores the newest waker.
    fn register(&mut self, id: u64, waker: &Waker) {
        todo!("C1: find the id, or push to the back")
    }
    /// C2. Remove the longest-waiting task, wake it, and return its id.
    fn wake_one(&mut self) -> Option<u64> {
        todo!("C2: pop from the front")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use practice::guess;
    use std::{
        io::{BufRead, BufReader},
        sync::{
            atomic::{AtomicUsize, Ordering},
            mpsc,
        },
        task::Wake,
    };

    struct Counter(AtomicUsize);
    impl Wake for Counter {
        fn wake(self: Arc<Self>) {
            self.0.fetch_add(1, Ordering::Relaxed);
        }
    }
    fn counted() -> (Arc<Counter>, Waker) {
        let c = Arc::new(Counter(AtomicUsize::new(0)));
        let w = Waker::from(Arc::clone(&c));
        (c, w)
    }

    /// Returns Pending `n` times (waking itself each time), then Ready.
    fn yield_times(mut n: u32) -> impl Future<Output = ()> {
        std::future::poll_fn(move |cx| {
            if n == 0 {
                return Poll::Ready(());
            }
            n -= 1;
            cx.waker().wake_by_ref();
            Poll::Pending
        })
    }

    /// Polls with a no-op waker until Ready; returns the output and the poll count.
    fn drive<F: Future>(future: F) -> (F::Output, usize) {
        let mut f = std::pin::pin!(future);
        let mut cx = Context::from_waker(Waker::noop());
        for polls in 1..=1000 {
            if let Poll::Ready(x) = f.as_mut().poll(&mut cx) {
                return (x, polls);
            }
        }
        panic!("still pending after 1000 polls")
    }

    /// An executor whose waker sends on a channel; it blocks on `recv` while Pending.
    struct ChannelWake(Mutex<mpsc::Sender<()>>);
    impl Wake for ChannelWake {
        fn wake(self: Arc<Self>) {
            let _ = self.0.lock().unwrap().send(());
        }
    }
    fn wait_for<F: Future>(future: F) -> (F::Output, usize) {
        let (tx, rx) = mpsc::channel();
        let waker = Waker::from(Arc::new(ChannelWake(Mutex::new(tx))));
        let mut cx = Context::from_waker(&waker);
        let mut f = std::pin::pin!(future);
        let mut polls = 0;
        loop {
            polls += 1;
            if let Poll::Ready(x) = f.as_mut().poll(&mut cx) {
                return (x, polls);
            }
            rx.recv_timeout(std::time::Duration::from_secs(10))
                .expect("Pending, but nobody woke the task within 10 s");
        }
    }

    #[test]
    fn a_futures_run_concurrently_in_input_order() {
        let futures: Vec<Pin<Box<dyn Future<Output = &str>>>> = vec![
            Box::pin(async {
                yield_times(3).await;
                "slow"
            }),
            Box::pin(async { "instant" }),
            Box::pin(async {
                yield_times(1).await;
                "medium"
            }),
        ];
        let (out, polls) = drive(join_all(futures));
        assert_eq!(out, ["slow", "instant", "medium"]);
        assert_eq!(
            polls, 4,
            "concurrent: the slowest future decides the poll count"
        );
    }

    #[test]
    fn a_empty_join_is_ready_at_once() {
        let (out, polls) = drive(join_all(Vec::<Pin<Box<dyn Future<Output = u8>>>>::new()));
        assert!(out.is_empty());
        assert_eq!(polls, 1);
    }

    #[test]
    fn b_result_arrives_from_another_thread() {
        let caller = thread::current().id();
        let (worker, polls) = wait_for(spawn_compute(|| thread::current().id()));
        assert_ne!(worker, caller, "the job must run on a new thread");
        assert!(
            polls <= 3,
            "{polls} polls: register the waker and wait to be woken"
        );
    }

    #[test]
    fn b_waiting_task_is_woken_with_the_result() {
        let (go_tx, go_rx) = mpsc::channel::<()>();
        let mut f =
            spawn_compute(
                move || match go_rx.recv_timeout(std::time::Duration::from_secs(5)) {
                    Ok(()) => String::from("computed"),
                    Err(_) => String::from("the job blocked its caller: run it on a new thread"),
                },
            );
        let (c, w) = counted();
        assert!(
            Pin::new(&mut f)
                .poll(&mut Context::from_waker(&w))
                .is_pending()
        );
        go_tx.send(()).unwrap();
        let (out, _) = wait_for(f);
        assert_eq!(out, "computed");
        assert!(c.0.load(Ordering::Relaxed) <= 1);
    }

    #[test]
    fn c_wait_queue_is_first_in_first_out() {
        let mut q = WaitQueue::default();
        let (a, wa) = counted();
        let (b, wb) = counted();
        q.register(1, &wa);
        q.register(2, &wb);
        assert_eq!(q.wake_one(), Some(1));
        assert_eq!(a.0.load(Ordering::Relaxed), 1);
        assert_eq!(q.wake_one(), Some(2));
        assert_eq!(b.0.load(Ordering::Relaxed), 1);
        assert_eq!(q.wake_one(), None);
    }

    #[test]
    fn c_re_registering_keeps_the_place_and_the_newest_waker() {
        let mut q = WaitQueue::default();
        let (old, w_old) = counted();
        let (new, w_new) = counted();
        let (other, w_other) = counted();
        q.register(7, &w_old);
        q.register(8, &w_other);
        q.register(7, &w_new);
        assert_eq!(q.waiters.len(), 2);
        assert_eq!(
            Arc::strong_count(&old),
            2,
            "the replaced waker must be dropped"
        );
        assert_eq!(q.wake_one(), Some(7));
        assert_eq!(
            (old.0.load(Ordering::Relaxed), new.0.load(Ordering::Relaxed)),
            (0, 1)
        );
        assert_eq!(q.wake_one(), Some(8));
        assert_eq!(other.0.load(Ordering::Relaxed), 1);
    }

    /// P. Earlier-chapter recall: predict each value BEFORE running.
    #[test]
    fn p_predictions() {
        // P1 (Ch20). Two futures are created; only the second is polled.
        // What does it return, and what does the counter hold afterwards?
        let counter = AtomicUsize::new(0);
        let bump = || async { counter.fetch_add(1, Ordering::Relaxed) };
        let _first = bump();
        let (second, _) = drive(bump());
        let p1: (usize, usize) = guess();
        assert_eq!(
            p1,
            (second, counter.load(Ordering::Relaxed)),
            "P1: lazy futures"
        );

        // P2 (Ch19). Strong count seen inside a scoped thread that clones the Arc,
        // and after the scope ends.
        let shared = Arc::new(5);
        let inside = thread::scope(|s| {
            s.spawn(|| Arc::strong_count(&Arc::clone(&shared)))
                .join()
                .unwrap()
        });
        let p2: (usize, usize) = guess();
        assert_eq!(p2, (inside, Arc::strong_count(&shared)), "P2: Arc counts");

        // P3 (Ch18). Lines in "a\nb" and in "a\n".
        let p3: (usize, usize) = guess();
        assert_eq!(
            p3,
            (
                BufReader::new(&b"a\nb"[..]).lines().count(),
                BufReader::new(&b"a\n"[..]).lines().count()
            ),
            "P3: a trailing newline does not start a line"
        );

        // P4 (Ch16). A deque after push_back(2), push_front(3), then pop_back().
        let mut d = VecDeque::from([1]);
        d.push_back(2);
        d.push_front(3);
        let popped = d.pop_back();
        let p4: (Option<i32>, Vec<i32>) = guess();
        assert_eq!(p4, (popped, Vec::from(d)), "P4: VecDeque ends");
    }
}
