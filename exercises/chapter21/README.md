# Chapter 21 — Futures

[All chapters](../../README.md) · [Previous chapter](../chapter20/README.md) · [Next chapter](../chapter22/README.md)

Original exercises accompanying [Programming Rust, 3rd edition, chapter 21](https://learning.oreilly.com/library/view/programming-rust-3rd/9781098176228/ch21.html). Read the corresponding section, then practice without copying the book's examples.

## Working route

Predict on paper first. Implement one lesson's TODOs, run its filtered tests, do its scratch experiment, and explain the result in your own words. Add one edge case of your own. Open [HINTS.md](HINTS.md) only after an attempt. Passing tests is one part of mastery; the explanation and labs matter too.

The starter compiles; unfinished TODOs intentionally fail when reached. Keep the supplied tests. Each lesson is independent, including the capstone, so later unfinished tasks do not block an earlier lesson.

| Done | Lesson | Coding tasks |
| --- | --- | --- |
| [ ] | [l01 · Implement Future](l01_future_trait.rs) | 2 |
| [ ] | [l02 · Pending and wakeups](l02_wake.rs) | 3 |
| [ ] | [l03 · A completion cell](l03_completion.rs) | 3 |
| [ ] | [l04 · A parker block_on](l04_executor.rs) | 2 |
| [ ] | [l05 · Readiness registration model](l05_reactor.rs) | 2 |
| [ ] | [l06 · Cooperative work budgets](l06_cooperation.rs) | 2 |
| [ ] | [l07 · Capstone: pinned future adapter](l07_capstone.rs) | 2 |

```sh
cargo test --test chapter21 l01_
cargo test --test chapter21 -- --list
# After completing all lessons:
cargo test --test chapter21
cargo test --release --test chapter21
cargo run --example ch21_scratch
```

Use [the scratch example](../../examples/ch21_scratch.rs) for experiments and restore it to a compiling state afterward. Core lessons use only the standard library. Dependency or platform labs have their own instructions and are outside the normal test target.

## Coverage map

The section labels below follow the book's contents. A mapping to a lab means a concrete manual project, not automated coverage of an external library.

| Book sections | Practice |
| --- | --- |
| The Future Trait; Implementing and Instantiating Futures | l01 one-shot future and l02 pending/waking |
| How Do We Know When to Poll?; Invoking Wakers: spawn_blocking | l02 wake semantics; l03 completion cell; LABS.md blocking adapter |
| Implementing block_on; The Role of Runtimes | l04 parker executor; LABS.md executor responsibilities |
| Implementing I/O Futures; Reactors; Implementing a Reactor | l05 readiness registration model; LABS.md actual loopback reactor |
| Cooperative Multitasking; Tokio’s Executor | l06 bounded work per poll; LABS.md real runtime observation |
| Pinning; The Two Life Stages of a Future; Pinned Pointers; The Unpin Trait | l07 pinned future capstone and compiler experiments |
| Conclusion | cross-lesson executor/reactor/pinning explanation |

## Extension labs

- [LABS.md](LABS.md)

## Mastery check

1. What promises accompany Poll::Pending, and why must futures not block in poll?
2. How do wakeups schedule another poll without carrying the result themselves?
3. Where must waker registration and completion state be synchronized to avoid missed wakeups?
4. How does thread::unpark avoid a wake-before-park race?
5. Which responsibilities belong to an executor, a reactor, and a future?
6. Why does pinning matter after polling begins, and which values can safely move through Unpin?
7. How can a future cooperate by bounding work per poll?

Redo the capstone from its contract in a later session. Explain one design tradeoff and create a case the supplied tests do not exercise.

## Validation scope

The dependency-free starter target compiles on the installed Rust 1.99 toolchain. Temporary complete reference implementations pass the chapter tests in debug and release; reference answers are not saved in this exercise directory. External-runtime, procedural-macro, native-library, platform-specific, and Miri extension projects are assigned practice, and were not executed as part of this validation.
