# Chapter 11 — Traits and Generics

[All chapters](../../README.md) · [Previous chapter](../chapter10/README.md) · [Next chapter](../chapter12/README.md)

Original guided exercises for [*Programming Rust*, 3rd edition, chapter 11](https://learning.oreilly.com/library/view/programming-rust-3rd/9781098176228/ch11.html).
Read the corresponding book sections alongside these exercises. These are new problems, not reproductions of the book’s examples.

This chapter has **7 lessons, 19 implementation tasks, and 19 automated checks**, plus prediction, compiler/manual, explanation, and transfer work.
The starter functions intentionally contain `todo!()`: they compile, and tests fail until you implement the contracts. Keep the supplied tests intact. Type signatures/trait scaffolds are provided so each lesson compiles independently; reconstruct their bounds in the compiler labs to practice design rather than only filling method bodies.

## Work through the chapter

For each lesson: predict before running; implement one task; run its filter; complete the lab in [the scratch example](../../examples/ch11_scratch.rs); explain the ownership/trait/API choices; add a transfer case.
Read [Hint 1 before Hint 2](HINTS.md) only if stuck. Passing tests alone does not demonstrate mastery.

| Done | Lesson | Check |
| --- | --- | --- |
| [ ] | [01 · Traits, implementations, and defaults](l01_interfaces.rs) | `cargo test --test chapter11 l01_` |
| [ ] | [02 · Static and dynamic dispatch](l02_dispatch.rs) | `cargo test --test chapter11 l02_` |
| [ ] | [03 · Self, supertraits, and associated items](l03_self.rs) | `cargo test --test chapter11 l03_` |
| [ ] | [04 · Extension traits and qualified calls](l04_extensions.rs) | `cargo test --test chapter11 l04_` |
| [ ] | [05 · Associated types and generic traits](l05_relationships.rs) | `cargo test --test chapter11 l05_` |
| [ ] | [06 · Opaque results, lifetimes, and precise capture](l06_bounds.rs) | `cargo test --test chapter11 l06_` |
| [ ] | [07 · Capstone: a generic validation pipeline](l07_capstone.rs) | `cargo test --test chapter11 l07_` |

Run commands from the repository root:

```sh
cargo test --test chapter11 --no-run
cargo test --test chapter11 -- --list
cargo run --example ch11_scratch
cargo test --test chapter11
cargo test --release --test chapter11
```

Some labs deliberately fail to compile or panic. Keep them in scratch and restore a compiling example afterward. Do not put intentional compilation failures in the test modules. Tests are deterministic and require only the standard library; any separate dependency lab is described below.

## Coverage against the chapter

Every main topic is assigned to a coded lesson or concrete manual/compiler lab. A lesson reference includes its prompts and lab as well as its tests.

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
