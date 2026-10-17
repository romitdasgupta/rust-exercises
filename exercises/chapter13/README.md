# Chapter 13 — Utility Traits

[All chapters](../../README.md) · [Previous chapter](../chapter12/README.md) · [Next chapter](../chapter14/README.md)

Original guided exercises for [*Programming Rust*, 3rd edition, chapter 13](https://learning.oreilly.com/library/view/programming-rust-3rd/9781098176228/ch13.html).
Read the corresponding book sections alongside these exercises. These are new problems, not reproductions of the book’s examples.

This chapter has **8 lessons, 22 implementation tasks, and 22 automated checks**, plus prediction, compiler/manual, explanation, and transfer work.
The starter functions intentionally contain `todo!()`: they compile, and tests fail until you implement the contracts. Keep the supplied tests intact. Type signatures/trait scaffolds are provided so each lesson compiles independently; reconstruct their bounds in the compiler labs to practice design rather than only filling method bodies.

## Work through the chapter

For each lesson: predict before running; implement one task; run its filter; complete the lab in [the scratch example](../../examples/ch13_scratch.rs); explain the ownership/trait/API choices; add a transfer case.
Read [Hint 1 before Hint 2](HINTS.md) only if stuck. Passing tests alone does not demonstrate mastery.

| Done | Lesson | Check |
| --- | --- | --- |
| [ ] | [01 · Destruction and resource lifetimes](l01_drop.rs) | `cargo test --test chapter13 l01_` |
| [ ] | [02 · Sized, Clone, and Copy](l02_sized_clone.rs) | `cargo test --test chapter13 l02_` |
| [ ] | [03 · Deref, DerefMut, and Default](l03_deref_default.rs) | `cargo test --test chapter13 l03_` |
| [ ] | [04 · AsRef and AsMut views](l04_views.rs) | `cargo test --test chapter13 l04_` |
| [ ] | [05 · Borrow and borrowed lookup](l05_borrow.rs) | `cargo test --test chapter13 l05_` |
| [ ] | [06 · Infallible and fallible conversions](l06_conversions.rs) | `cargo test --test chapter13 l06_` |
| [ ] | [07 · ToOwned and clone-on-write](l07_cow.rs) | `cargo test --test chapter13 l07_` |
| [ ] | [08 · Capstone: validated names with cheap views](l08_capstone.rs) | `cargo test --test chapter13 l08_` |

Run commands from the repository root:

```sh
cargo test --test chapter13 --no-run
cargo test --test chapter13 -- --list
cargo run --example ch13_scratch
cargo test --test chapter13
cargo test --release --test chapter13
```

Some labs deliberately fail to compile or panic. Keep them in scratch and restore a compiling example afterward. Do not put intentional compilation failures in the test modules. Tests are deterministic and require only the standard library; any separate dependency lab is described below.

## Coverage against the chapter

Every main topic is assigned to a coded lesson or concrete manual/compiler lab. A lesson reference includes its prompts and lab as well as its tests.

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
