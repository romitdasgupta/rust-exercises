# Chapter 19 — Threads

[All chapters](../../README.md) · [Previous chapter](../chapter18/README.md) · [Next chapter](../chapter20/README.md)

Original exercises accompanying [Programming Rust, 3rd edition, chapter 19](https://learning.oreilly.com/library/view/programming-rust-3rd/9781098176228/ch19.html). Read the corresponding section, then practice without copying the book's examples.

## Working route

Predict on paper first. Implement one lesson's TODOs, run its filtered tests, do its scratch experiment, and explain the result in your own words. Add one edge case of your own. Open [HINTS.md](HINTS.md) only after an attempt. Passing tests is one part of mastery; the explanation and labs matter too.

The starter compiles; unfinished TODOs intentionally fail when reached. Keep the supplied tests. Each lesson is independent, including the capstone, so later unfinished tasks do not block an earlier lesson.

| Done | Lesson | Coding tasks |
| --- | --- | --- |
| [ ] | [l01 · Fork, join, and worker failures](l01_fork_join.rs) | 2 |
| [ ] | [l02 · Borrowed parallel work](l02_scoped.rs) | 2 |
| [ ] | [l03 · Ownership and channel closure](l03_channels.rs) | 2 |
| [ ] | [l04 · Send and Sync contracts](l04_send_sync.rs) | 2 |
| [ ] | [l05 · Mutex invariants and poisoning](l05_mutexes.rs) | 2 |
| [ ] | [l06 · RwLock and condition predicates](l06_coordination.rs) | 3 |
| [ ] | [l07 · Atomic counters and local state](l07_atomics_tls.rs) | 2 |
| [ ] | [l08 · Capstone: bounded parallel pipeline](l08_capstone.rs) | 2 |

```sh
cargo test --test chapter19 l01_
cargo test --test chapter19 -- --list
# After completing all lessons:
cargo test --test chapter19
cargo test --release --test chapter19
cargo run --example ch19_scratch
```

Use [the scratch example](../../examples/ch19_scratch.rs) for experiments and restore it to a compiling state afterward. Core lessons use only the standard library. Dependency or platform labs have their own instructions and are outside the normal test target.

## Coverage map

The section labels below follow the book's contents. A mapping to a lab means a concrete manual project, not automated coverage of an external library.

| Book sections | Practice |
| --- | --- |
| Fork-Join Parallelism; spawn and join; Error Handling Across Threads | l01 join ownership and panic payloads |
| Sharing Immutable Data Across Threads; Rayon; Revisiting the Mandelbrot Set | l02 scoped slices; LABS.md Rayon image-grid project |
| Channels; Sending Values; Receiving Values; Running the Pipeline | l03 channel ownership and l08 bounded pipeline |
| Channel Features and Performance; Piping Almost Any Iterator to a Channel; Beyond Pipelines | l03, l08; LABS.md bounded multistage experiment |
| Thread Safety: Send and Sync | l04 trait-bound/compiler experiments |
| Shared Mutable State; What Is a Mutex?; Mutex<T>; mut and Mutex | l05 shared counter and poisoning recovery |
| Why Mutexes Are Not Always a Good Idea; Deadlock; Poisoned Mutexes; Multiconsumer Channels Using Mutexes | l05; LABS.md lock-order and worker-queue project |
| Read/Write Locks (RwLock<T>); Condition Variables (Condvar) | l06 snapshots and predicate waits |
| Atomics; Global Variables; Thread-Local Variables | l07; LABS.md per-run state and OnceLock |
| What Hacking Concurrent Code in Rust Is Like | l08 capstone plus scheduling-independent review |

## Extension labs

- [LABS.md](LABS.md)

## Mastery check

1. When can scoped threads borrow data that spawn cannot?
2. How do join errors differ from a worker Result error?
3. Which values must be Send, and when does Arc<T> require Sync?
4. How do channel closure, bounded backpressure, and a forgotten sender affect termination?
5. What invariant is a mutex protecting; why does a Condvar wait in a loop?
6. Which guarantees does relaxed atomic arithmetic provide, and which does it not provide?
7. How can you design concurrent tests around completion signals rather than sleeps?

Redo the capstone from its contract in a later session. Explain one design tradeoff and create a case the supplied tests do not exercise.

## Validation scope

Use the repository's pinned Rust 1.99.0 toolchain and run commands from the repository root (see [setup instructions](../../README.md#fresh-clone-setup)). The dependency-free starter target can be checked without running TODOs using `cargo test --offline --test chapter19 --no-run`. No complete reference solution is shipped. Extension projects are manual practice and are outside the core compilation checks.
