# Executors and real I/O

## Blocking work adapter

In an isolated scratch package adapt l03's completion cell into `spawn_blocking(FnOnce() -> T) -> impl Future<Output=Result<T, JobError>>`. Move the closure into an OS worker and catch its panic with catch_unwind. Store success or failure in the shared state and wake after unlocking. Add explicit producer-disconnected behavior rather than leaving the receiver pending forever. Acceptance: completion before polling, polling before completion, replacement waker, worker panic, and receiver dropped early are covered. Drive order with std channels; no sleeps. Keep a join/reaper policy so no worker is accidentally detached. Run it through l04's block_on and explain how this differs from Tokio's pooled spawn_blocking, including cancellation limitations.

## A small reactor using real readiness (optional Mio)

Create an isolated package with the current Mio release with the `os-poll` and `net` features for Poll and networking. Use a loopback listener on port 0 and a nonblocking accepted TcpStream. Build a token-to-waker registration table protected by a mutex, and a reactor thread owning Poll. A read future first attempts a nonblocking read, handles WouldBlock by registration, and arranges an interest update/wakeup for the reactor. Address the check/register race explicitly (for example register then retry the read before returning Pending). Reactor readiness wakes the registered task; the future retries I/O, since readiness can be stale/spurious. Drive a two-part payload with acknowledgement channels; no external network or sleeps. Acceptance: no wake is lost when a byte arrives around registration, EOF is distinguished from WouldBlock, dropped futures unregister their token, stale tokens cannot wake a replacement resource, and a shutdown signal joins the reactor. Bound integration tests with timeouts to diagnose broken code. Explain edge-triggered draining and why an event alone is not a complete message.

## Runtime roles and fairness

In a separate Tokio package run two budgeted CPU futures on a current-thread executor. Use shared logs to show they yield between chunks; compare with one future doing all work in one poll. Do not assert an implementation-specific task order. Inspect official documentation for Tokio's task queues, cooperative budget, I/O driver, and timer driver; distinguish public guarantees from implementation details. Acceptance: same final results, all handles joined, no blocking I/O in poll, and a diagram separating future, executor, reactor, timer, waker, and OS readiness.

## Pinning and future lifecycle audit

Write a compile-only matrix for T, Box<T>, Pin<&mut T>, and Pin<Box<T>> with T: Unpin and a type containing PhantomPinned. Demonstrate that moving a pinned pointer handle is allowed while moving a !Unpin pointee through a safe API is rejected. Do not bypass Pin with unchecked access to make a forbidden move. Compare a newly constructed future with one that has been polled to Pending and now stores internal borrows. Acceptance: record compiler diagnostics for unsafe moves, demonstrate a safe pinned adapter without unsafe projection, and explain why polling takes Pin<&mut Self> even for futures that happen to be Unpin.

References: [Future](https://doc.rust-lang.org/std/future/trait.Future.html), [Pin](https://doc.rust-lang.org/std/pin/), [Mio](https://docs.rs/mio/latest/mio/), and [Tokio runtime](https://docs.rs/tokio/latest/tokio/runtime/).
