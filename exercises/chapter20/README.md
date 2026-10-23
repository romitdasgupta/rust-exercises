# Chapter 20 — Asynchronous Programming

[All chapters](../../README.md) · [Previous chapter](../chapter19/README.md) · [Next chapter](../chapter21/README.md)

Original exercises accompanying [Programming Rust, 3rd edition, chapter 20](https://learning.oreilly.com/library/view/programming-rust-3rd/9781098176228/ch20.html). Read the corresponding section, then practice without copying the book's examples.

## Work through the chapter

Every lesson uses the same loop. It is built on *retrieval practice*: you keep what you predict, struggle with, and explain, not what you reread.

1. **Read** the book section for the lesson.
2. **Predict.** Open the lesson and answer its `p_predictions` test: replace each `guess()` with a value. Don't run anything yet.
3. **Implement** the `todo!()` bodies from their contracts and run `cargo test --test chapter20 l01_`. A failed prediction is feedback: write in [NOTES.md](NOTES.md) why you were wrong.
4. **Check technique.** Futures here are driven by tiny test executors, so tests can count polls, log the order in which work starts, probe a lock with `try_lock` while a future is suspended, and count heap allocations. A solution that runs work concurrently when the contract says sequentially, holds a lock across `.await`, or boxes needlessly fails with a message. Then run `cargo clippy --test chapter20` and fix warnings in your code.
5. **Drill the compiler.** `cargo drill ch20` shows small programs; predict whether each compiles (and which error), then fix it.
6. **Explain** in [NOTES.md](NOTES.md), add one edge-case test, then compare with *Check your explanation* in [HINTS.md](HINTS.md).

After the numbered lessons, do the **review** lesson (r01: earlier chapters, from memory) and the **code review** lesson (c01: fix a teammate's working-but-wrong code). Run `cargo progress` to see where you are and which chapters are due for spaced review.

The starter compiles; unfinished TODOs intentionally fail when reached. Keep the supplied tests. Each lesson is independent, including the capstone, so later unfinished tasks do not block an earlier lesson. Test names start with the task letter (`a_`, `b_`, ...), so `cargo test --test chapter20 l06_racing::tests::a_` runs one task's tests.

| Done | Lesson | Coding tasks |
| --- | --- | --- |
| [ ] | [l01 · Lazy futures and await](l01_lazy.rs) | 2 |
| [ ] | [l02 · Borrowed and owned futures](l02_lifetimes.rs) | 2 |
| [ ] | [l03 · Async behavior in traits](l03_traits.rs) | 2 |
| [ ] | [l04 · Lending async callbacks](l04_async_closures.rs) | 2 |
| [ ] | [l05 · Cancellation by dropping](l05_cancellation.rs) | 2 |
| [ ] | [l06 · Polling a race](l06_racing.rs) | 2 |
| [ ] | [l07 · Capstone: asynchronous graph traversal](l07_capstone.rs) | 2 |
| [ ] | [R01 · Review: Chapters 13 and 17–19 from memory](r01_review.rs) | 3 |
| [ ] | [C01 · Code review: a teammate's pull request](c01_code_review.rs) | 3 |
| [ ] | [Compiler drills](../../drills/ch20/) — `cargo drill ch20` | 10 |

```sh
cargo test --test chapter20 l01_
cargo test --test chapter20 -- --list
# After completing all lessons:
cargo test --test chapter20
cargo test --release --test chapter20
cargo clippy --test chapter20
cargo drill ch20
cargo run --example ch20_scratch
```

Use [the scratch example](../../examples/ch20_scratch.rs) for experiments and restore it to a compiling state afterward. Core lessons use only the standard library. Dependency or platform labs have their own instructions and are outside the normal test target.

## Coverage map

The section labels below follow the book's contents. A mapping to a lab means a concrete manual project, not automated coverage of an external library.

| Book sections | Practice |
| --- | --- |
| What Asynchronous Code Looks Like; Futures; Asynchronous Function; Future Types; async Blocks | l01 lazy computation and sequencing |
| Future Lifetimes; Tasks and ’static; Tasks and Send | l02 borrowing/ownership and LABS.md spawn bounds |
| Spawning Asynchronous Tasks; Listing a Web Page’s Outgoing Links; Spidering the Web Concurrently; Extracting the Traversal’s Results | l07 in-memory graph capstone; LABS.md Tokio task-based local crawler |
| Waiting Versus Blocking; Asynchronous Versions of Blocking Tools | LABS.md cooperative scheduling and spawn_blocking experiment |
| Asynchronous Closures; AsyncFn; Asynchronous Functions in Traits | l03 trait dispatch and l04 lending async callbacks |
| Cancellation; Cancel Safety | l05 drop-based cancellation and LABS.md byte-read state experiment |
| Racing Futures with tokio::select! | l06 poll-level race model; LABS.md real select! bias/cancellation project |
| Shutdown; Aborting Tasks; Shutdown Signals; Cancellation Tokens; Conclusion | LABS.md Tokio/tokio-util shutdown project and capstone review |

## Extension labs

- [LABS.md](LABS.md)

## Mastery check

1. Why is calling an async function different from executing its body?
2. What are the borrowing, Send, and static requirements of a spawned task?
3. Which work can safely block an executor thread, and what belongs in spawn_blocking?
4. How do AsyncFn, AsyncFnMut, and AsyncFnOnce express lending asynchronous callbacks?
5. What state is lost when a future is dropped in a select! branch?
6. How do abort, cooperative signals, and cancellation tokens differ?
7. What evidence demonstrates that a concurrent traversal actually bounds its work and collects all task errors?

Redo the capstone from its contract in a later session. Explain one design tradeoff and create a case the supplied tests do not exercise.

## Validation scope

Use the repository's pinned Rust 1.99.0 toolchain and run commands from the repository root (see [setup instructions](../../README.md#fresh-clone-setup)). The dependency-free starter target can be checked without running TODOs using `cargo test --offline --test chapter20 --no-run`. No complete reference solution is shipped. Extension projects are manual practice and are outside the core compilation checks.
