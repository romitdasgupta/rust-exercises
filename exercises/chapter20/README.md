# Chapter 20 — Asynchronous Programming

[All chapters](../../README.md) · [Previous chapter](../chapter19/README.md) · [Next chapter](../chapter21/README.md)

Original exercises accompanying [Programming Rust, 3rd edition, chapter 20](https://learning.oreilly.com/library/view/programming-rust-3rd/9781098176228/ch20.html). Read the corresponding section, then practice without copying the book's examples.

## Working route

Predict on paper first. Implement one lesson's TODOs, run its filtered tests, do its scratch experiment, and explain the result in your own words. Add one edge case of your own. Open [HINTS.md](HINTS.md) only after an attempt. Passing tests is one part of mastery; the explanation and labs matter too.

The starter compiles; unfinished TODOs intentionally fail when reached. Keep the supplied tests. Each lesson is independent, including the capstone, so later unfinished tasks do not block an earlier lesson.

| Done | Lesson | Coding tasks |
| --- | --- | --- |
| [ ] | [l01 · Lazy futures and await](l01_lazy.rs) | 2 |
| [ ] | [l02 · Borrowed and owned futures](l02_lifetimes.rs) | 2 |
| [ ] | [l03 · Async behavior in traits](l03_traits.rs) | 2 |
| [ ] | [l04 · Lending async callbacks](l04_async_closures.rs) | 2 |
| [ ] | [l05 · Cancellation by dropping](l05_cancellation.rs) | 2 |
| [ ] | [l06 · Polling a race](l06_racing.rs) | 2 |
| [ ] | [l07 · Capstone: asynchronous graph traversal](l07_capstone.rs) | 2 |

```sh
cargo test --test chapter20 l01_
cargo test --test chapter20 -- --list
# After completing all lessons:
cargo test --test chapter20
cargo test --release --test chapter20
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
