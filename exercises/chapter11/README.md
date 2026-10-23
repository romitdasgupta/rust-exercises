# Chapter 11 — Traits and Generics

[All chapters](../../README.md) · [Previous chapter](../chapter10/README.md) · [Next chapter](../chapter12/README.md)

Original guided exercises for [*Programming Rust*, 3rd edition, chapter 11](https://learning.oreilly.com/library/view/programming-rust-3rd/9781098176228/ch11.html). Read the corresponding section before each lesson. These exercises use original scenarios and contracts. Type signatures and trait scaffolds are provided so each lesson compiles; the compiler labs ask you to rebuild their bounds yourself.

## Work through the chapter

Every lesson uses the same loop. It is built on *retrieval practice*: you keep what you predict, struggle with, and explain, not what you reread.

1. **Read** the book section for the lesson.
2. **Predict.** Open the lesson and answer its `p_predictions` test: replace each `guess()` with a value. Don't run anything yet.
3. **Implement** the `todo!()` bodies from their contracts and run `cargo test --test chapter11 l01_`. A failed prediction is feedback: write in [NOTES.md](NOTES.md) why you were wrong.
4. **Check technique.** Some tests count heap allocations or compare pointers, so a solution that copies where it could borrow fails. Then run `cargo clippy --test chapter11` and fix warnings in your code.
5. **Drill the compiler.** `cargo drill ch11` shows small programs; predict whether each compiles (and which error), then fix it.
6. **Explain** in [NOTES.md](NOTES.md), add one edge-case test, then compare with *Check your explanation* in [HINTS.md](HINTS.md).

After the numbered lessons, do the **review** lesson (r01: earlier chapters, from memory) and the **code review** lesson (c01: fix a teammate's working-but-wrong code). Run `cargo progress` to see where you are and which chapters are due for spaced review.

| Done | Lesson |
| --- | --- |
| [ ] | [01 · Traits, implementations, and defaults](l01_interfaces.rs) |
| [ ] | [02 · Static and dynamic dispatch](l02_dispatch.rs) |
| [ ] | [03 · Self, supertraits, and associated items](l03_self.rs) |
| [ ] | [04 · Extension traits and qualified calls](l04_extensions.rs) |
| [ ] | [05 · Associated types and generic traits](l05_relationships.rs) |
| [ ] | [06 · Opaque results, lifetimes, and precise capture](l06_bounds.rs) |
| [ ] | [07 · Capstone: a generic validation pipeline](l07_capstone.rs) |
| [ ] | [R01 · Review: Chapters 7, 9, and 10 from memory](r01_review.rs) |
| [ ] | [C01 · Code review: a teammate's pull request](c01_code_review.rs) |
| [ ] | [Compiler drills](../../drills/ch11/) — `cargo drill ch11` |

```sh
cargo test --test chapter11 l01_
cargo test --test chapter11 -- --list
cargo test --test chapter11
cargo test --release --test chapter11
cargo clippy --test chapter11
cargo drill ch11
cargo run --example ch11_scratch
```

The starters compile; tests fail intentionally at `todo!()` and `guess()` until you fill them in. Keep the supplied tests intact. Every runnable task uses the standard library and needs no network. Each lesson's compiler lab belongs in [the scratch example](../../examples/ch11_scratch.rs); some labs deliberately fail to compile or panic, so restore it to compiling code afterward. Never put intentional compilation failures in the test modules.

Use [progressive hints](HINTS.md) only after a first attempt.

## Coverage against the chapter

| Book topic | Practice |
| --- | --- |
| Using traits; dyn dispatch; generic functions; choosing dispatch | 01–02 |
| Defining and implementing traits; default methods | 01 |
| Traits on external types; fully qualified calls | 04 |
| Self; subtraits; associated functions and constants | 03 |
| Relationships between types; associated types; generic traits | 05 |
| impl Trait; lifetime bounds; precise use bounds; discovering bounds | 06 + compiler labs |
| Traits as a foundation | 07 capstone |

## Mastery check

- [ ] Explain a trait object’s data/vtable pair and a generic function’s monomorphization.
- [ ] Design a dyn-compatible interface, then explain which methods would break compatibility.
- [ ] Choose associated types versus a generic trait parameter and justify each bound.
- [ ] Explain Self, supertraits, associated constants, extension traits, and coherence.
- [ ] Demonstrate how return-position impl Trait captures lifetimes in edition 2024.
- [ ] Redo the capstone from its contract after a few days without consulting your earlier implementation.
- [ ] Present your predictions, failed compiler experiments, and one new edge case for review.
