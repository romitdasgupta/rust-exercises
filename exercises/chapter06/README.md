# Chapter 6 — Expressions

[All chapters](../../README.md) · [Previous chapter](../chapter05/README.md) · [Next chapter](../chapter07/README.md)

Original guided practice for [Programming Rust, 3rd edition, chapter 6](https://learning.oreilly.com/library/view/programming-rust-3rd/9781098176228/ch06.html). Read the corresponding section before each lesson. These exercises use original scenarios and contracts.

## Work through the chapter

Every lesson uses the same loop. It is built on *retrieval practice*: you keep what you predict, struggle with, and explain, not what you reread.

1. **Read** the book section for the lesson.
2. **Predict.** Open the lesson and answer its `p_predictions` test: replace each `guess()` with a value. Don't run anything yet.
3. **Implement** the `todo!()` bodies from their contracts and run `cargo test --test chapter06 l01_`. A failed prediction is feedback: write in [NOTES.md](NOTES.md) why you were wrong.
4. **Check technique.** Some tests count heap allocations or compare pointers, so a solution that copies everything fails. Then run `cargo clippy --test chapter06` and fix warnings in your code.
5. **Drill the compiler.** `cargo drill ch06` shows small programs; predict whether each compiles (and which error), then fix it.
6. **Explain** in [NOTES.md](NOTES.md), add one edge-case test, then compare with *Check your explanation* in [HINTS.md](HINTS.md).

After the numbered lessons, do the **review** lesson (r01: earlier chapters, from memory) and the **code review** lesson (c01: fix a teammate's working-but-wrong code). Run `cargo progress` to see where you are and which chapters are due for spaced review.

| Done | Lesson |
| --- | --- |
| [ ] | [01 · Expression blocks and declarations](l01_blocks.rs) |
| [ ] | [02 · if and match as values](l02_branching.rs) |
| [ ] | [03 · if let and let else](l03_let_else.rs) |
| [ ] | [04 · for, while, break and continue](l04_loops.rs) |
| [ ] | [05 · Loop values and labeled blocks](l05_labels.rs) |
| [ ] | [06 · const and unsafe expression blocks](l06_const_unsafe.rs) |
| [ ] | [07 · Operators, calls, fields and assignment](l07_operators.rs) |
| [ ] | [08 · Casts and closure expressions](l08_casts_closures.rs) |
| [ ] | [09 · Capstone: command expression engine](l09_capstone.rs) |
| [ ] | [R01 · Review: Chapters 3–5 from memory](r01_review.rs) |
| [ ] | [C01 · Code review: a teammate's pull request](c01_code_review.rs) |
| [ ] | [Compiler drills](../../drills/ch06/) — `cargo drill ch06` |

```sh
cargo test --test chapter06 l01_
cargo test --test chapter06 -- --list
cargo test --test chapter06
cargo test --release --test chapter06
cargo clippy --test chapter06
cargo drill ch06
cargo run --example ch06_scratch
```

The starters compile; tests fail intentionally at `todo!()` and `guess()` until you fill them in. Work in lesson order and keep tests intact. Every runnable task uses the standard library and needs no network. Compiler labs belong in [the scratch example](../../examples/ch06_scratch.rs), not in the exercise suite. Restore it to compiling code after each intentional error. Any package/tooling lab below is separate from the standard-library tests.

Use [progressive hints](HINTS.md) only after a first attempt.

## Coverage map

| Book section or topic | Practice |
| --- | --- |
| An expression language; blocks; declarations | 01: expression-valued blocks, shadowing and item declarations lab |
| if and match; if let; let else | 02: branching; 03: refutable extraction and early exits |
| Loops; break and continue expressions; labeled blocks | 04: iteration; 05: loop values and labeled early exits |
| return expressions; why Rust has loop | 05: divergence and return versus break lab |
| const and unsafe blocks | 06: inline constants and locally justified unsafe lab |
| Function/method calls; fields/elements; reference operators | 07: precedence, receiver and indexing lab |
| Arithmetic, bitwise, comparison, logical operators; assignment | 07: operator behavior and short-circuiting |
| Type casts; closures; onward | 08: explicit casts and closure captures; 09: control-flow capstone |

## Mastery check

- [ ] Explain how semicolons and tail expressions change a block type.
- [ ] Choose match, if let, and let else according to exhaustiveness and early exit needs.
- [ ] Trace return, labeled break, continue, and loop values; explain the never type.
- [ ] Separate const evaluation from runtime and explain the proof obligation at an unsafe block.
- [ ] Explain operator precedence, short-circuiting, cast truncation, and closure capture.
- [ ] Redo the final capstone after a break without looking at your implementation; invent a changed requirement and explain the tradeoff.

Use the book as the authority for language details. Manual labs deliberately include behavior that executable tests cannot establish, such as which program the borrow checker rejects or what an external crate's users can access.
