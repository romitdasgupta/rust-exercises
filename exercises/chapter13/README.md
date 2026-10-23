# Chapter 13 — Utility Traits

[All chapters](../../README.md) · [Previous chapter](../chapter12/README.md) · [Next chapter](../chapter14/README.md)

Original guided exercises for [*Programming Rust*, 3rd edition, chapter 13](https://learning.oreilly.com/library/view/programming-rust-3rd/9781098176228/ch13.html). Read the corresponding section before each lesson. These exercises use original scenarios and contracts. Type signatures and trait scaffolds are provided so each lesson compiles; the compiler labs ask you to rebuild their bounds yourself.

## Work through the chapter

Every lesson uses the same loop. It is built on *retrieval practice*: you keep what you predict, struggle with, and explain, not what you reread.

1. **Read** the book section for the lesson.
2. **Predict.** Open the lesson and answer its `p_predictions` test: replace each `guess()` with a value. Don't run anything yet.
3. **Implement** the `todo!()` bodies from their contracts and run `cargo test --test chapter13 l01_`. A failed prediction is feedback: write in [NOTES.md](NOTES.md) why you were wrong.
4. **Check technique.** Some tests count heap allocations or compare pointers, so a solution that copies where it could borrow fails. Then run `cargo clippy --test chapter13` and fix warnings in your code.
5. **Drill the compiler.** `cargo drill ch13` shows small programs; predict whether each compiles (and which error), then fix it.
6. **Explain** in [NOTES.md](NOTES.md), add one edge-case test, then compare with *Check your explanation* in [HINTS.md](HINTS.md).

After the numbered lessons, do the **review** lesson (r01: earlier chapters, from memory) and the **code review** lesson (c01: fix a teammate's working-but-wrong code). Run `cargo progress` to see where you are and which chapters are due for spaced review.

| Done | Lesson |
| --- | --- |
| [ ] | [01 · Destruction and resource lifetimes](l01_drop.rs) |
| [ ] | [02 · Sized, Clone, and Copy](l02_sized_clone.rs) |
| [ ] | [03 · Deref, DerefMut, and Default](l03_deref_default.rs) |
| [ ] | [04 · AsRef and AsMut views](l04_views.rs) |
| [ ] | [05 · Borrow and borrowed lookup](l05_borrow.rs) |
| [ ] | [06 · Infallible and fallible conversions](l06_conversions.rs) |
| [ ] | [07 · ToOwned and clone-on-write](l07_cow.rs) |
| [ ] | [08 · Capstone: validated names with cheap views](l08_capstone.rs) |
| [ ] | [R01 · Review: Chapters 9, 11, and 12 from memory](r01_review.rs) |
| [ ] | [C01 · Code review: a teammate's pull request](c01_code_review.rs) |
| [ ] | [Compiler drills](../../drills/ch13/) — `cargo drill ch13` |

```sh
cargo test --test chapter13 l01_
cargo test --test chapter13 -- --list
cargo test --test chapter13
cargo test --release --test chapter13
cargo clippy --test chapter13
cargo drill ch13
cargo run --example ch13_scratch
```

The starters compile; tests fail intentionally at `todo!()` and `guess()` until you fill them in. Keep the supplied tests intact. Every runnable task uses the standard library and needs no network. Each lesson's compiler lab belongs in [the scratch example](../../examples/ch13_scratch.rs); some labs deliberately fail to compile or panic, so restore it to compiling code afterward. Never put intentional compilation failures in the test modules.

Use [progressive hints](HINTS.md) only after a first attempt.

## Coverage against the chapter

| Book topic | Practice |
| --- | --- |
| Drop | 01 |
| Sized; Clone; Copy | 02 + compiler labs |
| Deref; DerefMut; Default | 03 |
| AsRef; AsMut | 04 |
| Borrow; BorrowMut | 05 |
| From; Into; TryFrom; TryInto | 06 |
| ToOwned; Cow | 07 |
| Choosing utility traits in an API | 08 capstone |

## Mastery check

- [ ] Predict destruction order, including explicit drop, moves, and unwinding.
- [ ] Explain why Copy forbids Drop and how Clone can do real work.
- [ ] Use ?Sized when an API need not require statically sized values.
- [ ] Distinguish cheap reference views, identity-preserving borrows, and owned conversions.
- [ ] Explain where Cow avoids allocation and where it must allocate.
- [ ] Redo the capstone from its contract after a few days without consulting your earlier implementation.
- [ ] Present your predictions, failed compiler experiments, and one new edge case for review.
