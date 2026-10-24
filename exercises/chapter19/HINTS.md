# Chapter 19 hints

[Chapter guide](README.md) · [All hints](../../HINTS.md)

Try hint 1 before opening hint 2.

## l01 · Fork, join, and worker failures

<details><summary>Hint 1</summary>

Collect join handles before joining, so all workers can make progress.

</details>

<details><summary>Hint 2</summary>

The join result is Result<Result<u32, &str>, panic payload>. Map only the outer error, then return the inner result.

</details>

<details><summary>Check your explanation (after writing your own)</summary>

A `JoinHandle<T>`'s `join` returns `thread::Result<T>`: the outer `Err` carries a panic payload (`Box<dyn Any + Send>`), and the inner `T` is whatever the closure returned, so a worker that returns `Result<u32, E>` produces `Result<Result<u32, E>, payload>`. Mapping only the outer error keeps the worker's own `Ok`/`Err` intact and turns a panic into an application value. A `move` closure takes ownership of each captured value, so the String travels to the new thread and is dropped there; `spawn` demands `'static` because the caller might return before the worker finishes. Dropping a handle detaches the thread instead of cancelling it, so you join every handle you need results from, in the order you spawned them, to report results in input order.

</details>

## l02 · Borrowed parallel work

<details><summary>Hint 1</summary>

Split before spawning; the compiler can prove the halves do not overlap.

</details>

<details><summary>Hint 2</summary>

thread::scope allows non-static borrows. Use scoped handles for returned sums, and mutate each half directly in separate closures.

</details>

<details><summary>Check your explanation (after writing your own)</summary>

`thread::scope` guarantees that every thread spawned inside it is joined before `scope` returns, so those threads may borrow locals that outlive the scope; no `'static` bound and no `Arc` are needed. The borrow checker still applies across threads: two closures may share `&[i32]`, but two `&mut` borrows must not overlap, which `split_at_mut` proves by producing two disjoint halves. Because the joins happen at the end of the scope, any borrow taken by a worker has ended once `scope` returns and the data is usable again. Joining a scoped handle yourself is only needed to get its return value earlier.

</details>

## l03 · Ownership and channel closure

<details><summary>Hint 1</summary>

An iterator over a receiver ends only after all sender handles have been dropped and buffered messages consumed.

</details>

<details><summary>Hint 2</summary>

Use an explicit channel::<String>. In drain_now match TryRecvError variants without retrying Empty.

</details>

<details><summary>Check your explanation (after writing your own)</summary>

Messages from one sender arrive in the order they were sent (FIFO), but messages from several senders interleave in whatever order the scheduler produced, so a test must not assume a global order unless the values carry an index. `send` on an unbounded `channel` never blocks; it only means the value was queued, not processed. A `sync_channel(n)` blocks the sender when n messages are waiting, which bounds memory and slows a fast producer to the consumer's pace (backpressure); capacity 0 is a rendezvous. `recv` and the receiver's iterator report disconnection only after every sender is dropped and the buffer is empty, so a forgotten sender clone keeps a consumer waiting forever.

</details>

## l04 · Send and Sync contracts

<details><summary>Hint 1</summary>

Send is about moving a value across threads; Sync is about sharing references across threads.

</details>

<details><summary>Hint 2</summary>

Arc cloning changes ownership counts, not Vec contents. Clone before spawning and join both workers before returning the original.

</details>

<details><summary>Check your explanation (after writing your own)</summary>

`Send` means a value may be moved to another thread; `Sync` means `&T` may be shared, and `T: Sync` exactly when `&T: Send`. They are auto traits: the compiler implements them for a type when all of its fields have them, so `Rc` (non-atomic count) and `RefCell` (non-atomic borrow flag) opt out, and anything containing them loses the trait too. `Arc<T>` is `Send` and `Sync` only when `T: Send + Sync`, because every clone gives another thread `&T`. `Mutex<T>` adds `Sync` (it serializes access), but it still requires `T: Send`, because the thread that locks it can move the value out; a `Mutex<Rc<_>>` would let two threads touch one non-atomic count.

</details>

## l05 · Mutex invariants and poisoning

<details><summary>Hint 1</summary>

Keep a guard only while modifying the invariant. PoisonError can return the guard for deliberate recovery.

</details>

<details><summary>Hint 2</summary>

Use into_inner on the poison error, overwrite the entire protected state, then clear_poison. Joining workers establishes completion before reading.

</details>

<details><summary>Check your explanation (after writing your own)</summary>

The invariant is "the counter equals the number of increments that have completed", and it holds whenever the mutex is unlocked. A `MutexGuard` is the only way to reach the data, and the lock is released when the guard is dropped, so keep guards short and never hold one across slow work or a join. If a thread panics while holding the guard, the mutex is poisoned: later `lock` calls return `Err`, because the invariant might be half-updated, but the data is still there (`into_inner` on the error gives it back) and poisoning proves nothing about corruption. Deadlock comes from two threads taking two locks in opposite orders, or one thread locking a std `Mutex` it already holds (it is not reentrant); a single global acquisition order prevents the first.

</details>

## l06 · RwLock and condition predicates

<details><summary>Hint 1</summary>

The mutex-protected Option is the state; the Condvar only helps wait efficiently. Two tests publish before anyone waits, so a solution that treats the notification as the data fails them; another starts the consumer first, so the wake path is exercised too.

</details>

<details><summary>Hint 2</summary>

while guard.is_none() { guard = cv.wait(guard).unwrap(); }. After the loop, take the value. For the snapshot, `read()` (not `write()`) lets other readers in while you clone.

</details>

<details><summary>Check your explanation (after writing your own)</summary>

A `Condvar` is a wake-up mechanism, not storage: a `notify` with no waiter is lost, so the real state lives in the mutex-protected value and the waiter checks a predicate. `wait` atomically releases the mutex and blocks, then reacquires it before returning, which lets the producer take the lock and change the state. Waits may return spuriously, or after another consumer already took the value, so the check is a `while` loop (or `wait_while`), never an `if`. An `RwLock` lets many readers in at once but a writer needs exclusive access; you cannot upgrade a read guard, so drop it and take a write lock, then re-check.

</details>

## l07 · Atomic counters and local state

<details><summary>Hint 1</summary>

try_update (called fetch_update on older toolchains) accepts a checked update function and returns the old value on success.

</details>

<details><summary>Hint 2</summary>

Use Cell inside thread_local!, with initialization inside each fresh worker thread. TLS is not shared mutation of one counter.

</details>

<details><summary>Check your explanation (after writing your own)</summary>

Atomicity means a read-modify-write such as `fetch_add` happens as one indivisible step, so no increment is lost and every reserved ID is unique. Ordering is a separate question: `Relaxed` orders nothing except that one variable, so it is right for counters and ID allocation but cannot publish other data; that needs `Release` on the store and `Acquire` on the load, or a lock. Atomic arithmetic wraps on overflow (even in debug builds), which is why `reserve` uses a checked update loop (`fetch_update`/`try_update`). A `thread_local!` gives each thread its own independent value, so nothing is shared at all. A `static mut` counter would need an invariant nobody can enforce ("only one thread touches this"), which is why reading or writing it is `unsafe`.

</details>

## l08 · Capstone: bounded parallel pipeline

<details><summary>Hint 1</summary>

A bounded sender can block until the receiver drains. Drop the original sender before collecting to EOF.

</details>

<details><summary>Hint 2</summary>

Spawn workers, consume rx inside the scope, then join workers. Tag values with input indices to reconstruct order without guessing a schedule.

</details>

<details><summary>Check your explanation (after writing your own)</summary>

Each producer owns a clone of the `SyncSender`; the original is dropped right after spawning, so the receiver's iterator ends exactly when the last producer finishes and drops its clone. With capacity 2, a producer blocks once two values wait, so the collecting loop must run while the producers are alive; joining first would deadlock as soon as the buffer filled. The output order across producers is unspecified, so each value carries its input index and sorting by that index *is* the contract: it restores a meaning the data already had. Sorting output to make a flaky test pass, without an index that defines the order, would hide a bug instead. In `collect_results`, returning at the first `Err` drops the receiver, and producers see that as a failed `send`, which is why they must handle send errors instead of unwrapping.

</details>

## R01 · Spaced review of Chapters 14–18

<details><summary>Hint 1</summary>

A: `Cursor<Vec<u8>>` implements `BufRead`, and `lines()` yields `io::Result<String>`, so `?` inside the worker closure turns the first bad line into that source's error. B: workers borrow the chunks and a shared `Mutex<BTreeMap>` through `thread::scope`. C: a `Box<dyn FnOnce() -> T + Send>` is itself `FnOnce() -> T + Send + 'static`.

</details>
<details><summary>Hint 2</summary>

A: spawn every worker first, collect the handles, then join them in order. B: count into a local `BTreeMap<&str, usize>` with `entry(..).or_default()`, then lock once and add each count into the shared map, allocating a String only for new keys if you like. C: `jobs.into_iter().map(thread::spawn).collect::<Vec<_>>()` starts them all; joining in order gives input order.

</details>

## C01 · Code review

<details><summary>Hint 1</summary>

There are two bugs: one reverses the results, one leaves stale cache entries behind. Then look at how many threads `map_parallel` starts, what `join().unwrap()` does when `f` panics, and what `refresh` holds while it calls `fetch`.

</details>
<details><summary>Hint 2</summary>

Split the items into `workers.max(1)` chunks with `chunks(len.div_ceil(workers))`, give each chunk one scoped worker that returns a `Vec<R>`, and join the handles in order with `map_err(|_| "worker panicked")?`. In `refresh`, build the new map first (calling `fetch` with no lock held), then take the write lock only to swap it in.

</details>
<details><summary>Check your review</summary>

`map_parallel` pops handles from the end, so results come back reversed. It also ignores `workers` and spawns one thread per item, which is unbounded for large inputs, and `join().unwrap()` turns a worker panic into a panic in the caller instead of the documented `Err`. `refresh` holds the write lock for the whole loop, so every reader waits behind slow fetches, and because it only inserts, keys that are no longer wanted stay in the cache. The fix computes everything without the lock and replaces the map in one short critical section, which also makes the update atomic from a reader's point of view.

</details>

## Drills

Run `cargo drill ch19`. The compiler output explains *what* happened; these notes explain *why*. Read one only after the drill's verdict.

<details><summary>d01 · Spawning a borrowing closure</summary>E0373. `thread::spawn` requires a `'static` closure, because the new thread may outlive `main`'s stack frame; the closure only borrows `words`. Add `move` so the vector moves into the thread, or use `thread::scope` to borrow it safely.</details>
<details><summary>d02 · Arc does not make RefCell thread-safe</summary>E0277: `RefCell<Vec<&str>>` is not `Sync`, so `Arc<RefCell<_>>` is not `Send`. Its borrow flag is a plain integer that two threads could update at once. Use `Arc<Mutex<Vec<&str>>>` and `lock()` instead of `borrow_mut()`.</details>
<details><summary>d03 · A Mutex moved into the first worker</summary>E0382: the first `move` closure takes ownership of the Mutex, so the second cannot use it. Share it with `Arc<Mutex<_>>` and move one `Arc` clone into each worker, or borrow it from both workers inside `thread::scope`.</details>
<details><summary>d04 · Two scoped workers, one mutable slice</summary>E0499: both closures capture `pixels` mutably, and the compiler does not reason about which index ranges they touch. `let (left, right) = pixels.split_at_mut(3);` produces two disjoint `&mut` slices, one per worker.</details>
<details><summary>d05 · Mutating through a non-mut Mutex</summary>Compiles and runs. `lock()` takes `&self`; the Mutex provides interior mutability, and the guard hands out `&mut` to one thread at a time. Each guard here is a temporary dropped at the end of its statement, so the next `lock()` does not deadlock.</details>
<details><summary>d06 · Returning a reference out of a guard</summary>E0515: the reference points into data reached through the guard, and the guard (and with it the lock) is dropped when the function returns. Return an owned value (`.clone()`), return the `MutexGuard` itself, or take a closure that runs while the lock is held.</details>
<details><summary>d07 · A Cell in a static</summary>E0277: every thread can reach a `static`, so its type must be `Sync`, and `Cell` is not. Use `AtomicU32` (`fetch_add(1, Relaxed) + 1`) for a process-wide counter, or `thread_local!` if each thread should count separately.</details>
<details><summary>d08 · Condvar::wait takes the guard</summary>E0382: `wait` consumes the guard (it must unlock the mutex while sleeping) and returns a new one when it wakes. Reassign it: make the binding `mut` and write `guard = changed.wait(guard).unwrap();`, or use `wait_while`.</details>
<details><summary>d09 · Reading after a worker panicked</summary>Panics at run time: the worker panicked while holding the guard, so the mutex is poisoned and `lock().unwrap()` panics on the `PoisonError`. The data is intact (it contains the pushed 2). Recover deliberately with `lock().unwrap_or_else(PoisonError::into_inner)` when the invariant can be re-checked.</details>
<details><summary>d10 · Sending after the receiver is gone</summary>Panics at run time: the worker received one value and returned, dropping the receiver, so the second `send` returns `Err(SendError(2))` and `unwrap` panics. A failed send is how a producer learns that the consumer stopped; handle it (`if tx.send(2).is_err() { /* stop producing */ }`).</details>
