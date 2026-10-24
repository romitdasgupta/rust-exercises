# Chapter 12 — Operator Overloading

[All chapters](../../README.md) · [Previous chapter](../chapter11/README.md) · [Next chapter](../chapter13/README.md)

Original guided exercises for [*Programming Rust*, 3rd edition, chapter 12](https://learning.oreilly.com/library/view/programming-rust-3rd/9781098176228/ch12.html). Read the corresponding section before each lesson. These exercises use original scenarios and contracts. Type signatures and trait scaffolds are provided so each lesson compiles; the compiler labs ask you to rebuild their bounds yourself.

## Work through the chapter

Every lesson uses the same loop. It is built on *retrieval practice*: you keep what you predict, struggle with, and explain, not what you reread.

1. **Read** the book section for the lesson.
2. **Predict.** Open the lesson and answer its `p_predictions` test: replace each `guess()` with a value. Don't run anything yet.
3. **Implement** the `todo!()` bodies from their contracts and run `cargo test --test chapter12 l01_`. A failed prediction is feedback: write in [NOTES.md](NOTES.md) why you were wrong.
4. **Check technique.** Some tests count heap allocations or compare pointers, so a solution that copies where it could borrow fails. Then run `cargo clippy --test chapter12` and fix warnings in your code.
5. **Drill the compiler.** `cargo drill ch12` shows small programs; predict whether each compiles (and which error), then fix it.
6. **Explain** in [NOTES.md](NOTES.md), add one edge-case test, then compare with *Check your explanation* in [HINTS.md](HINTS.md).

After the numbered lessons, do the **review** lesson (r01: earlier chapters, from memory) and the **code review** lesson (c01: fix a teammate's working-but-wrong code). Run `cargo progress` to see where you are and which chapters are due for spaced review.

| Done | Lesson |
| --- | --- |
| [ ] | [01 · Unary, binary, and assignment operators](l01_arithmetic.rs) |
| [ ] | [02 · Equivalence and NaN](l02_equality.rs) |
| [ ] | [03 · Partial and total comparisons](l03_ordering.rs) |
| [ ] | [04 · Index and IndexMut](l04_indexing.rs) |
| [ ] | [05 · Capstone: a small bit-set API](l05_capstone.rs) |
| [ ] | [R01 · Review: Chapters 5, 10, and 11 from memory](r01_review.rs) |
| [ ] | [C01 · Code review: a teammate's pull request](c01_code_review.rs) |
| [ ] | [Compiler drills](../../drills/ch12/) — `cargo drill ch12` |

```sh
cargo test --test chapter12 l01_
cargo test --test chapter12 -- --list
cargo test --test chapter12
cargo test --release --test chapter12
cargo clippy --test chapter12
cargo drill ch12
cargo run --example ch12_scratch
```

The starters compile; tests fail intentionally at `todo!()` and `guess()` until you fill them in. Keep the supplied tests intact. Every runnable task uses the standard library and needs no network. Each lesson's compiler lab belongs in [the scratch example](../../examples/ch12_scratch.rs); some labs deliberately fail to compile or panic, so restore it to compiling code afterward. Never put intentional compilation failures in the test modules.

Use [progressive hints](HINTS.md) only after a first attempt.

## Coverage against the chapter

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
