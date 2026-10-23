# Chapter 21 hints

[Chapter guide](README.md) · [All hints](../../HINTS.md)

Try hint 1 before opening hint 2.

## l01 · Implement Future

<details><summary>Hint 1</summary>

Pin restricts moving the pointee; the smart-pointer handle itself can move.

</details>

<details><summary>Hint 2</summary>

get_mut is available because Once<T> is Unpin when T is Unpin. Take from Option rather than cloning the output.

</details>

<details><summary>Check your explanation (after writing your own)</summary>

`Output` is the type the future produces when it completes; `poll` returns `Poll<Output>`, either `Ready(value)` once or `Pending`. `poll` takes `self: Pin<&mut Self>` so the caller promises the future will not move again once polling has started, which is what lets compiler-generated futures hold references into their own state. `Once<T>` holds no such references, so it is `Unpin` whenever `T` is; that unlocks `Pin::get_mut`, giving an ordinary `&mut Self` and letting `Option::take` move the value out without cloning. A pinned `Box` keeps its contents at one heap address, so moving the `Pin<Box<_>>` handle is fine. Polling after `Ready` is a contract violation, and panicking with a clear message is the conventional response.

</details>

## l02 · Pending and wakeups

<details><summary>Hint 1</summary>

A waker is a scheduling notification; result storage belongs elsewhere.

</details>

<details><summary>Hint 2</summary>

Use cx.waker().wake_by_ref on the pending path only. Atomic fetch_add returns an old value that you can discard.

</details>

<details><summary>Check your explanation (after writing your own)</summary>

A `Waker` only says "poll this task again"; it carries no data, and waking does not run `poll` itself, it asks the executor to schedule a poll later. Executors may *coalesce* wakes: several wakes before the next poll lead to one poll, so a wake is not a counter of events and the future must re-check its own state when polled. A future that returns `Pending` must have arranged a wake (registered the waker somewhere, or woken itself), or it will never be polled again. Self-waking is right only when the poll made bounded progress, as `Countdown` does; self-waking without progress turns the task into a busy loop that burns CPU and starves others.

</details>

## l03 · A completion cell

<details><summary>Hint 1</summary>

The state and waiter registration must be inspected atomically under one mutex. Never wait on that mutex for future completion.

</details>

<details><summary>Hint 2</summary>

Store a clone of the latest waker. Move the waker out while locked, then invoke it only after the lock guard is dropped.

</details>

<details><summary>Check your explanation (after writing your own)</summary>

Completion first: `finish` locks, stores the value, finds no waiter, unlocks; the first `poll` locks, finds the value, and returns `Ready` without registering anything. Poll first: `poll` locks, finds no value, stores a clone of the current waker, unlocks, and returns `Pending`; later `finish` locks, stores the value, takes the waker, unlocks, and wakes, and the next poll finds the value. Because the "is there a value?" check and the waker registration happen under the same lock, `finish` cannot slip in between them, so no wake is lost. Each poll replaces the stored waker, since the executor may have moved the task. Waking after unlocking means a task woken on another thread does not immediately block on the lock you still hold.

</details>

## l04 · A parker block_on

<details><summary>Hint 1</summary>

thread::unpark stores a token if the thread has not parked yet. Always poll again after a park return.

</details>

<details><summary>Hint 2</summary>

Use pin! on the stack. A poll_fn closure can capture a yielded bool and an Option<T> that is taken only on the second poll.

</details>

<details><summary>Check your explanation (after writing your own)</summary>

`thread::unpark` sets a token for the thread; `park` consumes a token if one is present and returns immediately, otherwise it blocks. So a wake that arrives after the poll returned `Pending` but before `park` is not lost. `park` may also return spuriously with no wake at all, which is why `block_on` simply polls again in a loop rather than assuming the future is ready. This executor runs exactly one future on the calling thread; a real runtime adds a task queue for spawned tasks, an I/O reactor, timers, work stealing across threads, cooperative budgeting, and cancellation.

</details>

## l05 · Readiness registration model

<details><summary>Hint 1</summary>

The ready flag stores the event, not the waker. This toy event has one waiter and is never reset.

</details>

<details><summary>Hint 2</summary>

Never leave a window between checking ready and installing the waker. Actual I/O also retries the operation and handles WouldBlock.

</details>

<details><summary>Check your explanation (after writing your own)</summary>

Readiness means "an operation is now likely to make progress", not "your data has arrived": a readable socket may hold half a message, and readiness can be stale by the time you try, so the future always performs the real I/O and handles `WouldBlock` by re-registering. EOF is a successful read of 0 bytes, not an error. The ready flag is the stored event, so a signal that arrives before anyone polls is remembered. Checking the flag and installing the waker must happen under the same lock that the signaler uses to set the flag; otherwise the signal could land after the check but before registration, and the wake would go to nobody.

</details>

## l06 · Cooperative work budgets

<details><summary>Hint 1</summary>

A Future does not automatically make CPU work cooperative. Count elements processed in one poll.

</details>

<details><summary>Hint 2</summary>

Compute an end index bounded by both budget and slice length, and widen each element to u64 before adding. Wake only when work remains after consuming the budget.

</details>

<details><summary>Check your explanation (after writing your own)</summary>

A future that runs a long loop inside one `poll` monopolizes its executor thread: runtimes cannot preempt code in the middle of `poll`, so other tasks on that thread wait. `BudgetSum` bounds the work per poll to `budget` elements and, if more remain, returns `Pending`. Here the self-wake is appropriate because this poll *did* make progress and the future is immediately ready to make more; it is asking to go to the back of the queue, not waiting for an external event. Poll counts (`ceil(len / budget)`, at least one) show the structure, not speed; timing depends on the executor and the machine.

</details>

## l07 · Capstone: pinned future adapter

<details><summary>Hint 1</summary>

Pin<Box<F>> is itself a movable handle. Moving the wrapper does not move F inside its heap allocation.

</details>

<details><summary>Hint 2</summary>

get_mut on the wrapper is safe because its fields can move; then call inner.as_mut().poll(cx). Do not take the tag until completion.

</details>

<details><summary>Check your explanation (after writing your own)</summary>

`WithTag` stores the inner future as `Pin<Box<F>>`: the future lives at a fixed heap address that never changes, however often the wrapper moves. `Pin<Box<F>>` is `Unpin` whatever `F` is, and `Option<String>` is `Unpin`, so the whole wrapper is `Unpin` and `Pin::get_mut` gives safe `&mut` access to its fields; `inner.as_mut()` then re-creates the `Pin<&mut F>` needed to poll the inner future in place. No `unsafe` projection is needed because the only pinned thing is behind its own pinned pointer. Construction (`tagged`) moves `F` into the box before any poll, which is fine because a future only starts holding self-references after its first poll; from then on it never moves. The tag is taken only on `Ready`, so it survives any number of `Pending` polls.

</details>

## R01 · Spaced review of Chapters 16, 19, and 20

<details><summary>Hint 1</summary>

A: one `Option<T>` per future records which ones are done; each poll skips the finished ones. B: share `Arc<Mutex<Shared<T>>>` between the future and the worker, like l03's completion cell, with a thread as the producer. C: a `VecDeque` keeps arrival order; search it by id before pushing.

</details>
<details><summary>Hint 2</summary>

A: after polling, if every slot is `Some`, take them all into a `Vec`. B: the worker computes, then in a block stores the result and `take()`s the waiter, and wakes after the block; `poll` takes the result or replaces the waiter. C: `iter_mut().find(|(i, _)| *i == id)` and `Waker::clone_from` update in place; `pop_front()?` then `wake()` consumes the waker.

</details>

## C01 · Code review

<details><summary>Hint 1</summary>

There are two bugs: one wakes a stale waker, one returns `Pending` without arranging any wake. Then look at what `Message` does with the String, what `publish` is holding when it wakes, and what `ChunkedSum` allocates on every poll.

</details>
<details><summary>Hint 2</summary>

In `poll`, `take()` the value and always store the current waker. In `publish`, set the value and `take()` the waiter inside a block, then call `wake()` after the guard is gone. In `ChunkedSum`, sum the slice `items[next..end]` directly and call `cx.waker().wake_by_ref()` before returning `Pending`.

</details>
<details><summary>Check your review</summary>

`Message::poll` registers a waker only the first time, so after the executor polls with a new waker the task is never woken (a lost wakeup that looks like a hang in production). It clones the message instead of moving it out, which doubles the allocation and leaves the payload in the shared state. `publish` calls `wake_by_ref` while holding the lock, so a task woken on another thread immediately blocks on that lock, and it leaves the waker registered. `ChunkedSum` returns `Pending` without waking, so a real executor never polls it again; it only "works" under a loop that polls blindly. It also copies the remaining items into a new `Vec` on every poll, which is quadratic work over the whole sum.

</details>

## Drills

Run `cargo drill ch21`. The compiler output explains *what* happened; these notes explain *why*. Read one only after the drill's verdict.

<details><summary>d01 · Polling without a pin</summary>E0599: `poll` takes `self: Pin<&mut Self>`, so it is not callable on a plain value or `&mut`. `Ready<T>` is `Unpin`, so `Pin::new(&mut job).poll(&mut cx)` works; for any future, `std::pin::pin!(job)` or `Box::pin(job)` does.</details>
<details><summary>d02 · Pin::new on an async block</summary>E0277: the async block is `!Unpin` (it holds a reference to its own `text` across an await), and `Pin::new` only accepts `Unpin` targets. Pin it where it will stay: `let mut job = pin!(async { .. });` or `Box::pin(async { .. })`, then `job.as_mut().poll(&mut cx)`.</details>
<details><summary>d03 · poll with the wrong receiver</summary>E0053: the trait's method takes `self: Pin<&mut Self>`, and an impl must match. Write `fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>)`; because `Countdown` is `Unpin`, `self.0` reads and writes still work through the pin.</details>
<details><summary>d04 · A Future without an Output</summary>E0046: `Future` has an associated type, and every impl must name it. Add `type Output = u32;` and the `Poll<u32>` return type then matches.</details>
<details><summary>d05 · Storing the context's waker</summary>E0308: `cx.waker()` returns `&Waker`, borrowed from the `Context` for this poll only. To keep it, store an owned clone: `Some(cx.waker().clone())` (or `clone_from` into an existing one).</details>
<details><summary>d06 · Moving a pinned box</summary>Compiles and runs. Moving a `Pin<Box<_>>` moves only the pointer; the future stays at the same heap address, which is exactly the guarantee pinning needs.</details>
<details><summary>d07 · Polling Ready twice</summary>Panics at run time: "`Ready` polled after completion". `Ready` hands out its value once; after a future returns `Ready`, the contract says not to poll it again, and std futures panic if you do. Stop polling after `Ready`.</details>
<details><summary>d08 · Waking while the state is borrowed</summary>Panics at run time: "RefCell already mutably borrowed". The "wake" runs the task, which borrows the state that `complete` still holds mutably. It is the single-threaded version of waking under a mutex. Take the callback out inside a block, let the borrow end, then call it.</details>
<details><summary>d09 · A Waker from a plain struct</summary>E0277: `Waker` implements `From<Arc<W>>` for `W: Wake`, not `From<W>`. Wakers are cloned and moved between threads, so they share the task through an `Arc`: `let flag = Arc::new(Flag(..)); let waker = Waker::from(Arc::clone(&flag));`.</details>
<details><summary>d10 · Mutating through Pin<&mut Self></summary>E0596: `self` is a `Pin<&mut Self>` binding, and mutating through it requires the binding itself to be `mut` (it calls `DerefMut` on the pin). Write `mut self: Pin<&mut Self>`; that works here because `Retries` is `Unpin`.</details>
