# Chapter 12 — Operator Overloading

[All chapters](../../README.md) · [Previous chapter](../chapter11/README.md) · [Next chapter](../chapter13/README.md)

Original guided exercises for [*Programming Rust*, 3rd edition, chapter 12](https://learning.oreilly.com/library/view/programming-rust-3rd/9781098176228/ch12.html).
Read the corresponding book sections alongside these exercises. These are new problems, not reproductions of the book’s examples.

This chapter has **5 lessons, 16 implementation tasks, and 18 automated checks**, plus prediction, compiler/manual, explanation, and transfer work.
The starter functions intentionally contain `todo!()`: they compile, and tests fail until you implement the contracts. Keep the supplied tests intact. Type signatures/trait scaffolds are provided so each lesson compiles independently; reconstruct their bounds in the compiler labs to practice design rather than only filling method bodies.

## Work through the chapter

For each lesson: predict before running; implement one task; run its filter; complete the lab in [the scratch example](../../examples/ch12_scratch.rs); explain the ownership/trait/API choices; add a transfer case.
Read [Hint 1 before Hint 2](HINTS.md) only if stuck. Passing tests alone does not demonstrate mastery.

| Done | Lesson | Check |
| --- | --- | --- |
| [ ] | [01 · Unary, binary, and assignment operators](l01_arithmetic.rs) | `cargo test --test chapter12 l01_` |
| [ ] | [02 · Equivalence and NaN](l02_equality.rs) | `cargo test --test chapter12 l02_` |
| [ ] | [03 · Partial and total comparisons](l03_ordering.rs) | `cargo test --test chapter12 l03_` |
| [ ] | [04 · Index and IndexMut](l04_indexing.rs) | `cargo test --test chapter12 l04_` |
| [ ] | [05 · Capstone: a small bit-set API](l05_capstone.rs) | `cargo test --test chapter12 l05_` |

Run commands from the repository root:

```sh
cargo test --test chapter12 --no-run
cargo test --test chapter12 -- --list
cargo run --example ch12_scratch
cargo test --test chapter12
cargo test --release --test chapter12
```

Some labs deliberately fail to compile or panic. Keep them in scratch and restore a compiling example afterward. Do not put intentional compilation failures in the test modules. Tests are deterministic and require only the standard library; any separate dependency lab is described below.

## Coverage against the chapter

Every main topic is assigned to a coded lesson or concrete manual/compiler lab. A lesson reference includes its prompts and lab as well as its tests.

| Book topic | Practice |
| --- | --- |
| Arithmetic and bitwise operators; unary; binary; compound assignment | 01 + compiler lab |
| Equivalence comparisons; Eq and NaN | 02 |
| Ordered comparisons; partial and total order | 03 |
| Index and IndexMut | 04 |
| Other operators; design of a consistent operator API | 05 + compiler lab |

## Mastery check

- [ ] Explain operand ownership and why Add<Rhs> has an associated Output.
- [ ] Keep PartialEq, Eq, PartialOrd, and Ord consistent, including hashing implications.
- [ ] Explain why floats cannot promise Eq or a natural total order.
- [ ] Implement indexing that returns a reference and assignment that mutates in place.
- [ ] Name operators that Rust does not allow user code to overload.
- [ ] Redo the capstone from its contract after a few days without consulting your earlier implementation.
- [ ] Present your predictions, failed compiler experiments, and one new edge case for review.
