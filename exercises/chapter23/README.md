# Chapter 23 — Unsafe Code

[All chapters](../../README.md) · [Previous chapter](../chapter22/README.md) · [Next chapter](../chapter24/README.md)

Original exercises accompanying [Programming Rust, 3rd edition, chapter 23](https://learning.oreilly.com/library/view/programming-rust-3rd/9781098176228/ch23.html). Read the corresponding section, then practice without copying the book's examples.

## Work through the chapter

Every lesson uses the same loop. It is built on *retrieval practice*: you keep what you predict, struggle with, and explain, not what you reread.

1. **Read** the book section for the lesson.
2. **Predict.** Open the lesson and answer its `p_predictions` test: replace each `guess()` with a value. Don't run anything yet.
3. **Implement** the `todo!()` bodies from their contracts and run `cargo test --test chapter23 l01_`. A failed prediction is feedback: write in [NOTES.md](NOTES.md) why you were wrong.
4. **Check technique.** Some tests count heap allocations, compare pointers, or count drops, so a solution that copies instead of moving fails. Then run `cargo clippy --test chapter23` and fix warnings in your code.
5. **Drill the compiler.** `cargo drill ch23` shows small programs; predict whether each compiles (and which error), then fix it.
6. **Explain** in [NOTES.md](NOTES.md), add one edge-case test, then compare with *Check your explanation* in [HINTS.md](HINTS.md).

After the numbered lessons, do the **review** lesson (r01: earlier chapters, from memory) and the **code review** lesson (c01: fix a teammate's working-but-wrong code). Run `cargo progress` to see where you are and which chapters are due for spaced review.

Write a `// SAFETY:` comment for every unsafe block before you run it. A passing test does not show that an unsafe block is sound; your written argument is part of the exercise. Each lesson is independent, including the capstone, so later unfinished tasks do not block an earlier lesson.

| Done | Lesson | Coding tasks |
| --- | --- | --- |
| [ ] | [l01 · A safe invariant boundary](l01_ascii.rs) | 2 |
| [ ] | [l02 · Raw pointer reads with contracts](l02_pointers.rs) | 2 |
| [ ] | [l03 · Raw slice reconstruction](l03_raw_slices.rs) | 2 |
| [ ] | [l04 · A flagged borrow](l04_tagged.rs) | 2 |
| [ ] | [l05 · Layout, unsafe traits, unions](l05_representations.rs) | 3 |
| [ ] | [l06 · Moving values and initialization](l06_moves.rs) | 2 |
| [ ] | [l07 · A fixed-capacity byte gap buffer](l07_gap_buffer.rs) | 3 |
| [ ] | [l08 · Capstone: initialized-prefix storage](l08_capstone.rs) | 3 |
| [ ] | [R01 · Review: Chapters 17, 21, and 22 from memory](r01_review.rs) | 3 |
| [ ] | [C01 · Code review: a teammate's unsafe pull request](c01_code_review.rs) | 3 |
| [ ] | [Compiler drills](../../drills/ch23/) — `cargo drill ch23` | 10 |

```sh
cargo test --test chapter23 l01_
cargo test --test chapter23 -- --list
cargo test --test chapter23
cargo test --release --test chapter23
cargo clippy --test chapter23
cargo drill ch23
cargo run --example ch23_scratch
```

The starters compile; tests fail intentionally at `todo!()` and `guess()` until you fill them in. Keep the supplied tests intact. Every runnable task uses the standard library and needs no network. Free-form compiler experiments belong in [the scratch example](../../examples/ch23_scratch.rs); restore it to compiling code afterward. Dependency or platform labs have their own instructions and are outside the normal test target.

Use [progressive hints](HINTS.md) only after a first attempt.

## Coverage map

The section labels below follow the book's contents. A mapping to a lab means a concrete manual project, not automated coverage of an external library.

| Book sections | Practice |
| --- | --- |
| Unsafe from What?; unsafe Blocks; Marking Unsafe Code; Undefined Behavior | l01 invariant boundary and LABS.md audit; c01 unnecessary unsafe; drills d01–d05 |
| Example: An Efficient ASCII String Type | l01 original Ascii wrapper |
| Unsafe Functions; Raw Pointers; Dereferencing Raw Pointers Safely; Null Pointers | l02 raw read and l03 checked raw slice split |
| Example: RefWithFlag | l04 provenance-preserving tagged borrow |
| Unsafe Traits; Type Sizes and Alignments | l05 representation contracts/layout; drills d04, d07, d09 |
| Pointer Arithmetic; Moving Into and Out of Memory | l03 split and l06 ownership moves |
| Example: GapBuffer | l07 fixed-capacity byte gap buffer; LABS.md generic extension |
| Panic Safety in Unsafe Code | l06, l08 drop accounting; c01 panic between read and set_len; LABS.md panic audit |
| Unions | l05 bit views |
| Cross-section application | l08 initialized-prefix container capstone |

## Extension labs

- [LABS.md](LABS.md)

## Mastery check

1. Which safety guarantees remain the programmer’s responsibility inside unsafe blocks?
2. How do an unsafe function contract and a safe abstraction invariant differ?
3. What makes raw pointer reads valid: provenance, alignment, bounds, initialization, lifetime, and aliasing?
4. When must raw moves change ownership bookkeeping to prevent a double drop?
5. How do size, alignment, padding, and valid bit patterns constrain memory reinterpretation?
6. How does a gap buffer maintain disjoint initialized regions?
7. What must stay true if a user-provided operation panics?
8. What do tests and Miri establish, and which universal safety claims still need a written proof?

Redo the capstone from its contract in a later session. Explain one design tradeoff and create a case the supplied tests do not exercise.

## Validation scope

Use the repository's pinned Rust 1.99.0 toolchain and run commands from the repository root (see [setup instructions](../../README.md#fresh-clone-setup)). The dependency-free starter target can be checked without running TODOs using `cargo test --offline --test chapter23 --no-run`. No complete reference solution is shipped. Extension projects are manual practice and are outside the core compilation checks.
