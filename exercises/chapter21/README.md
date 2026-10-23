# Chapter 21 — Futures

[All chapters](../../README.md) · [Previous chapter](../chapter20/README.md) · [Next chapter](../chapter22/README.md)

Original exercises accompanying [Programming Rust, 3rd edition, chapter 21](https://learning.oreilly.com/library/view/programming-rust-3rd/9781098176228/ch21.html). Read the corresponding section, then practice without copying the book's examples.

## Work through the chapter

Every lesson uses the same loop. It is built on *retrieval practice*: you keep what you predict, struggle with, and explain, not what you reread.

1. **Read** the book section for the lesson.
2. **Predict.** Open the lesson and answer its `p_predictions` test: replace each `guess()` with a value. Don't run anything yet.
3. **Implement** the `todo!()` bodies from their contracts and run `cargo test --test chapter21 l01_`. A failed prediction is feedback: write in [NOTES.md](NOTES.md) why you were wrong.
4. **Check technique.** Tests drive futures by hand with counting wakers, so they can check that `Pending` always arranges a wake, that `Ready` does not, that the newest waker is the one woken, that wakes happen after a lock is released, and that polling does not allocate. A solution that busy-loops, wakes a stale waker, or clones instead of moving fails with a message. Then run `cargo clippy --test chapter21` and fix warnings in your code.
5. **Drill the compiler.** `cargo drill ch21` shows small programs; predict whether each compiles (and which error), then fix it.
6. **Explain** in [NOTES.md](NOTES.md), add one edge-case test, then compare with *Check your explanation* in [HINTS.md](HINTS.md).

After the numbered lessons, do the **review** lesson (r01: earlier chapters, from memory) and the **code review** lesson (c01: fix a teammate's working-but-wrong code). Run `cargo progress` to see where you are and which chapters are due for spaced review.

The starter compiles; unfinished TODOs intentionally fail when reached. Keep the supplied tests. Each lesson is independent, including the capstone, so later unfinished tasks do not block an earlier lesson. Test names start with the letters of the tasks they exercise (`a_`, `a_b_`, ...), so you can filter by task, for example `cargo test --test chapter21 l01_future_trait::tests::a_`.

| Done | Lesson | Coding tasks |
| --- | --- | --- |
| [ ] | [l01 · Implement Future](l01_future_trait.rs) | 2 |
| [ ] | [l02 · Pending and wakeups](l02_wake.rs) | 3 |
| [ ] | [l03 · A completion cell](l03_completion.rs) | 3 |
| [ ] | [l04 · A parker block_on](l04_executor.rs) | 2 |
| [ ] | [l05 · Readiness registration model](l05_reactor.rs) | 2 |
| [ ] | [l06 · Cooperative work budgets](l06_cooperation.rs) | 2 |
| [ ] | [l07 · Capstone: pinned future adapter](l07_capstone.rs) | 2 |
| [ ] | [R01 · Review: Chapters 16, 19, and 20 from memory](r01_review.rs) | 3 |
| [ ] | [C01 · Code review: a teammate's pull request](c01_code_review.rs) | 3 |
| [ ] | [Compiler drills](../../drills/ch21/) — `cargo drill ch21` | 10 |

```sh
cargo test --test chapter21 l01_
cargo test --test chapter21 -- --list
# After completing all lessons:
cargo test --test chapter21
cargo test --release --test chapter21
cargo clippy --test chapter21
cargo drill ch21
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

Use the repository's pinned Rust 1.99.0 toolchain and run commands from the repository root (see [setup instructions](../../README.md#fresh-clone-setup)). The dependency-free starter target can be checked without running TODOs using `cargo test --offline --test chapter21 --no-run`. No complete reference solution is shipped. Extension projects are manual practice and are outside the core compilation checks.
