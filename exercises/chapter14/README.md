# Chapter 14 — Closures

[All chapters](../../README.md) · [Previous chapter](../chapter13/README.md) · [Next chapter](../chapter15/README.md)

Original guided exercises for [*Programming Rust*, 3rd edition, chapter 14](https://learning.oreilly.com/library/view/programming-rust-3rd/9781098176228/ch14.html).
Read the corresponding book sections alongside these exercises. These are new problems, not reproductions of the book’s examples.

This chapter has **7 lessons and 16 implementation tasks**, a review lesson, a code-review lesson, and 10 compiler drills, with **39 automated checks**, plus prediction, compiler/manual, explanation, and transfer work.
The starter functions intentionally contain `todo!()`: they compile, and tests fail until you implement the contracts. Keep the supplied tests intact. Type signatures/trait scaffolds are provided so each lesson compiles independently; reconstruct their bounds in the compiler labs to practice design rather than only filling method bodies.

## Work through the chapter

Every lesson uses the same loop. It is built on *retrieval practice*: you keep what you predict, struggle with, and explain, not what you reread.

1. **Read** the book section for the lesson.
2. **Predict.** Open the lesson and answer its `p_predictions` test: replace each `guess()` with a value. Don't run anything yet.
3. **Implement** the `todo!()` bodies from their contracts and run `cargo test --test chapter14 l01_`. A failed prediction is feedback: write in [NOTES.md](NOTES.md) why you were wrong.
4. **Check technique.** Some tests count heap allocations, calls, or captured bytes, so a solution that clones captures or calls a callback too often fails. Then run `cargo clippy --test chapter14` and fix warnings in your code.
5. **Drill the compiler.** `cargo drill ch14` shows small programs; predict whether each compiles (and which error), then fix it.
6. **Explain** in [NOTES.md](NOTES.md), complete the lesson's lab in [the scratch example](../../examples/ch14_scratch.rs), add one edge-case test, then compare with *Check your explanation* in [HINTS.md](HINTS.md).

After the numbered lessons, do the **review** lesson (r01: earlier chapters, from memory) and the **code review** lesson (c01: fix a teammate's working-but-wrong code). Run `cargo progress` to see where you are and which chapters are due for spaced review.

| Done | Lesson | Check |
| --- | --- | --- |
| [ ] | [01 · Borrowed captures and sorting](l01_borrowed.rs) | `cargo test --test chapter14 l01_` |
| [ ] | [02 · Mutation, move, and consuming captures](l02_capture_modes.rs) | `cargo test --test chapter14 l02_` |
| [ ] | [03 · Choosing callable bounds](l03_call_traits.rs) | `cargo test --test chapter14 l03_` |
| [ ] | [04 · Returning borrowed and erased closures](l04_returning.rs) | `cargo test --test chapter14 l04_` |
| [ ] | [05 · Copy and Clone for captures](l05_cloning.rs) | `cargo test --test chapter14 l05_` |
| [ ] | [06 · Callback registration and invocation](l06_callbacks.rs) | `cargo test --test chapter14 l06_` |
| [ ] | [07 · Capstone: configurable text processing](l07_capstone.rs) | `cargo test --test chapter14 l07_` |
| [ ] | [R01 · Review: Chapters 10–13 from memory](r01_review.rs) | `cargo test --test chapter14 r01_` |
| [ ] | [C01 · Code review: a teammate's pull request](c01_code_review.rs) | `cargo test --test chapter14 c01_` |
| [ ] | [Compiler drills](../../drills/ch14/) | `cargo drill ch14` |

Run commands from the repository root:

```sh
cargo test --test chapter14 l01_
cargo test --test chapter14 -- --list
cargo test --test chapter14
cargo test --release --test chapter14
cargo clippy --test chapter14
cargo drill ch14
cargo run --example ch14_scratch
```

The starters compile; tests fail intentionally at `todo!()` and `guess()` until you fill them in. Keep the supplied tests intact. Use [progressive hints](HINTS.md) only after a first attempt; passing tests alone does not demonstrate mastery.

Some labs deliberately fail to compile or panic. Keep them in scratch and restore a compiling example afterward. Do not put intentional compilation failures in the test modules. Tests are deterministic and require only the standard library; any separate dependency lab is described below.

## Coverage against the chapter

Every main topic is assigned to a coded lesson or concrete manual/compiler lab. A lesson reference includes its prompts and lab as well as its tests.

| Book topic | Practice |
| --- | --- |
| Capturing variables; borrowed captures; move captures | 01–02 |
| Function and closure types; performance; safety | 03 + compiler labs |
| FnOnce; closures that consume captures; FnMut | 02–03 |
| Copy and Clone for closures | 05 |
| Callbacks; using closures effectively | 06–07 |
| Returning closures and lifetime design | 04 |
| Reading closure diagnostics (captures, call traits, closure types) | [drills](../../drills/ch14/) d01–d10 |

## Mastery check

- [ ] Determine whether a closure captures each variable by shared borrow, mutable borrow, or move.
- [ ] Choose the weakest useful Fn/FnMut/FnOnce bound.
- [ ] Explain why move does not imply FnOnce-only.
- [ ] Return closures with clear lifetimes and distinguish generic from boxed dispatch.
- [ ] Register and invoke callbacks without invalidating captured references.
- [ ] Redo the capstone from its contract after a few days without consulting your earlier implementation.
- [ ] Present your predictions, failed compiler experiments, and one new edge case for review.
