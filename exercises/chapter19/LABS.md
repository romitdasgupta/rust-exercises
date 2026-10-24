# Thread extension projects

These projects cover what the core tests cannot: real thread pools, deadlock-prone designs, and timing observations. Each lab lives in its own temporary Cargo project outside this repository, so the root package never gains a dependency. Work through the steps in order; each checkpoint tells you what you should see before moving on.

## Lab 1 · Rayon fork-join and pixel grid (optional dependency)

**Setup.** Create an isolated project (`cargo new --lib grid-lab`) and add the current Rayon release, following its official documentation:

```toml
[dependencies]
rayon = "1"
```

**Steps**

1. Define your own 160×90 grid type that stores one `u8` per pixel in a `Vec<u8>` of length `width * height`. Write `index(x, y) -> usize` and `coords(i) -> (x, y)` and test them against each other for every corner and one interior pixel.
   - *Checkpoint:* you should now see the round-trip tests pass, including on a 1×1 and a 0×0 grid.
2. Write a deterministic escape-count function for one pixel (your own iteration rule, not the book's Mandelbrot program), with a fixed iteration cap so each pixel's value is a pure function of its coordinates.
3. Implement four renderers that fill the same buffer:
   1. sequential, with a plain loop;
   2. scoped threads, each worker taking a chunk of rows (`thread::scope` plus `chunks_mut`);
   3. Rayon `par_iter_mut().enumerate()`;
   4. Rayon `par_chunks_mut(width)`, one row per task.
   - *Checkpoint:* you should now see a test that compares all four buffers byte for byte pass for empty, tiny (e.g. 3×2), and normal (160×90) grids.
4. Only after the equality tests pass, benchmark the renderers in `--release`. Compare Rayon's work-stealing pool with spawning one OS thread per row. Record overhead and how the work was partitioned; do not assert a speedup.
   - *Checkpoint:* you should now have a short table of timings with a note on machine and core count.
5. Try a parallel `reduce` (for example, the total escape count) and a fallible `try_for_each` that stops on an invalid pixel. Then sum a floating-point quantity in parallel and sequentially.
   - *Checkpoint:* you should now see that the integer reduction always matches, while the float sum may differ in its last digits between runs or between parallel and sequential.

**Acceptance criteria**

- [ ] All four renderers produce exactly the same byte buffer for empty, tiny, and normal grids.
- [ ] Coordinate-to-index calculations are checked by tests.
- [ ] No shared mutable lock per pixel; each task owns a disjoint slice.
- [ ] Benchmarks run only after the equality checks, in release mode, and document overhead and partitioning instead of asserting a speedup.
- [ ] A written comparison of work stealing versus one OS thread per row.
- [ ] A `reduce` and a fallible `try_for_each` are implemented, with an explanation of why floating-point reduction order can change the result.

## Lab 2 · Multiconsumer worker queue and backpressure

**Setup.** Standard library only:

```toml
[dependencies]
# none: std::sync::mpsc, Arc, Mutex, thread
```

**Steps**

1. Define `struct Job { id: u32, input: u64 }`. Create a job channel and wrap its receiver as `Arc<Mutex<Receiver<Job>>>`; create a separate results channel for `(job_id, result)`.
2. Spawn three workers. Each loop takes the lock, receives *one* job, drops the guard, and only then computes and sends `(job_id, result)`.
   - *Checkpoint:* you should now see all three workers make progress (log the worker number with each result).
3. Send the jobs, then drop every job sender. Drain the results channel while the workers are still running, and join the workers afterwards.
   - *Checkpoint:* you should now see every worker exit on its own once the job channel is closed and empty.
4. Change the code so a worker holds the guard while it computes. Observe and explain how this accidentally serializes the pool, then restore the correct version.
5. Replace the job channel with `sync_channel` capacities 0, 1, and 4. Treat a send error as cancellation: drop the results receiver early and make the producer and workers stop cleanly.
   - *Checkpoint:* you should now see each capacity complete with the same set of results, and the early-drop run terminate without a panic.
6. Add a generic producer that accepts any `IntoIterator<Item = T>` with `T: Send`. Write one version that borrows the iterator's source inside `thread::scope`, and one that moves an owned iterator into an unscoped worker. Compare the bounds each one needs.

**Acceptance criteria**

- [ ] Each job appears exactly once in the results.
- [ ] All workers terminate when the job channel closes; every handle is joined.
- [ ] Output comparisons use job IDs (sort or collect into a map), never an assumed worker order.
- [ ] No sleeps are used for coordination.
- [ ] Capacities 0, 1, and 4 are tested, and a disconnected receiver is handled as cancellation.
- [ ] Written explanations: why holding the guard through processing serializes the pool, and where bounded capacity limits queued memory.
- [ ] The generic `IntoIterator` producer exists in a scoped (borrowing) and an unscoped (owning) form, with a comparison of the two.

## Lab 3 · Waiting, lock order, and shutdown

**Setup.** Standard library only:

```toml
[dependencies]
# none: Mutex, Condvar, mpsc, thread
```

**Steps**

1. Build a slot `(Mutex<Option<Job>>, Condvar)` with one consumer. Before it starts waiting, the consumer sends a "ready to inspect the predicate" message on a channel; then it waits in a predicate loop (`while slot.is_none()`). The producer publishes under the lock and then notifies.
2. Give the consumer a second, completion channel. The test waits on it with `recv_timeout`, so a broken version fails within a bounded time instead of hanging; join the consumer only after completion arrives.
   - *Checkpoint:* you should now see the test pass with the "ready" message received before publishing.
3. Add a second test where the producer publishes *before* the consumer starts waiting.
   - *Checkpoint:* you should now see both orders pass, which shows the notification is not being used as stored data.
4. Add a `closed` flag to the protected state. Shutdown sets it under the lock and calls `notify_all`; the predicate loop exits when the slot is filled *or* the slot is closed. Run several consumers.
   - *Checkpoint:* you should now see every consumer return after shutdown, none stranded.
5. On paper, write a lock-order table for a two-account transfer (`transfer(a, b)` and `transfer(b, a)` running at once). Find the conflicting acquisition orders and repair them (for example, always lock the lower account ID first) *before* running anything.
6. Explain how you would recover from poisoning if a transfer panicked midway: what invariant (for example, the total balance) you re-check or restore, rather than simply clearing the poison.

**Acceptance criteria**

- [ ] No notification is used as stored state; the predicate lives in the mutex.
- [ ] Both publication-before-wait and waiter-before-publication are tested.
- [ ] Failures are bounded by `recv_timeout`; no lock guard is held across a `join`.
- [ ] Shutdown wakes all consumers via the closed flag and `notify_all`.
- [ ] The lock-order table and its repair are written down before the code runs.
- [ ] A written poisoning-recovery plan for a half-finished transfer.

## Lab 4 · Atomics, globals, and local state

**Setup.** Standard library only:

```toml
[dependencies]
# none: std::sync::atomic, Arc, OnceLock, thread_local!
```

**Steps**

1. Define a per-run statistics struct with `AtomicUsize` counters (for example `processed` and `rejected`). Create one per run and share it with workers through an explicit `Arc`; there is no global.
2. Run the workers with `fetch_add(1, Relaxed)` and join them all before reading the counts.
   - *Checkpoint:* you should now see the concurrent counts equal a sequential run of the same input.
3. Write two alternatives with the same results: a `Mutex<usize>` counter, and `thread_local!` accumulation that each worker returns and the caller merges after join. Compare the three designs.
4. Initialize immutable configuration once with a local `OnceLock` (inside the run, not a mutable global), and read it from every worker.
   - *Checkpoint:* you should now see the initializer run exactly once per run, even when several workers ask for the config.
5. Explain why `Relaxed` is enough for counts read after `join` (join itself synchronizes), but cannot publish unrelated non-atomic data.
6. Sketch, without running a broken program, a flag-plus-data publication scheme with `Relaxed` on both sides. Mark the missing happens-before edge and show how `Release`/`Acquire` adds it.

**Acceptance criteria**

- [ ] Each run starts from zero.
- [ ] Concurrent counts equal sequential counts for all three designs.
- [ ] No `unsafe impl Send`/`Sync` and no `static mut`.
- [ ] Configuration is initialized once through a local `OnceLock`; there is no mutable global configuration.
- [ ] A written explanation of why `Relaxed` suffices after `join` and where it does not.
- [ ] A sketch of the missing happens-before edge in flag-plus-data publication.

References: [std::thread](https://doc.rust-lang.org/std/thread/), [std::sync](https://doc.rust-lang.org/std/sync/), and [Rayon](https://docs.rs/rayon/latest/rayon/).
