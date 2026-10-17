# Thread extension projects

## Rayon fork-join and pixel grid (optional dependency)

Create an isolated temporary Cargo project; add the current Rayon crate using its official documentation. Define your own 160×90 grid where a pixel is a deterministic escape-count computation, without copying the book's Mandelbrot program. Write sequential, scoped-thread chunked, `par_iter_mut().enumerate()`, and `par_chunks_mut(width)` implementations. Acceptance: all produce exactly the same byte buffer for empty, tiny, and normal grids; coordinate-to-index calculations are checked; no shared mutable lock per pixel. Benchmark only after equality checks, in release mode. Compare thread-pool work stealing with spawning one OS thread per row, and document overhead/partitioning rather than asserting a speedup. Try parallel reduce and fallible try_for_each; explain why floating-point reduction order can affect results.

## Multiconsumer worker queue and backpressure

Implement `Arc<Mutex<Receiver<Job>>>` shared among three workers. Hold the receiver guard only for receiving one job, release it before computation, and return `(job_id, result)` over a separate channel. Close all senders, drain results while workers run, then join. Repeat with sync_channel capacities 0, 1, and 4; handle disconnected receivers as cancellation. Acceptance: each job appears once, all workers terminate on closure, output comparisons use IDs, and no sleeps/assumed worker order. Explain how holding the guard through processing accidentally serializes the pool, and where bounded capacity limits queued memory. Add a generic `IntoIterator<Item=T>` producer where T: Send and compare borrowing through a scope with owning the iterator in an unscoped worker.

## Waiting, lock order, and shutdown

Use a slot `(Mutex<Option<Job>>, Condvar)` with one consumer. Have the consumer send a "ready to inspect predicate" signal, then wait in a predicate loop; producer publishes under the lock and notifies. Use a second completion channel with recv_timeout to make lab failure bounded; join after completion. Test both publication-before-wait and waiter-before-publication. Add a closed flag so shutdown cannot strand a waiter. Acceptance: no notification is used as stored state, no lock guard crosses a join, and shutdown wakes all consumers. Write a lock-order table for a two-account transfer; repair conflicting acquisition orders on paper before running anything. Explain poisoning recovery for a transfer interrupted midway, rather than simply clearing poison.

## Atomics, globals, and local state

Build a per-run statistics struct with AtomicUsize counts and explicit Arc ownership. Keep task completion synchronized through join; explain why Relaxed is enough for counts observed after join but cannot publish unrelated non-atomic data. Compare a mutex counter, TLS accumulation followed by merge, and atomic fetch_add. Use a local OnceLock to initialize immutable configuration once; do not introduce mutable global configuration. Acceptance: each run starts from zero, concurrent counts equal sequential counts, and no unsafe Send/Sync implementation or static mut is used. Sketch (without executing a broken program) the missing happens-before edge in a flag-plus-data publication scheme.

References: [std::thread](https://doc.rust-lang.org/std/thread/), [std::sync](https://doc.rust-lang.org/std/sync/), and [Rayon](https://docs.rs/rayon/latest/rayon/).
