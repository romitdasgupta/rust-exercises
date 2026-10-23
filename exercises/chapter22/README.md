# Chapter 22 — Macros

[All chapters](../../README.md) · [Previous chapter](../chapter21/README.md) · [Next chapter](../chapter23/README.md)

Original exercises accompanying [Programming Rust, 3rd edition, chapter 22](https://learning.oreilly.com/library/view/programming-rust-3rd/9781098176228/ch22.html). Read the corresponding section, then practice without copying the book's examples.

## Work through the chapter

Every lesson uses the same loop. It is built on *retrieval practice*: you keep what you predict, struggle with, and explain, not what you reread.

1. **Read** the book section for the lesson.
2. **Predict.** Open the lesson and answer its `p_predictions` test: replace each `guess()` with a value. Don't run anything yet.
3. **Implement** the `todo!()` bodies from their contracts and run `cargo test --test chapter22 l01_`. A failed prediction is feedback: write in [NOTES.md](NOTES.md) why you were wrong.
4. **Check technique.** Some tests count heap allocations, evaluations, or threads, so a macro that copies, re-evaluates, or collects eagerly fails. Then run `cargo clippy --test chapter22` and fix warnings in your code.
5. **Drill the compiler.** `cargo drill ch22` shows small programs; predict whether each compiles (and which error), then fix it.
6. **Explain** in [NOTES.md](NOTES.md), add one edge-case test, then compare with *Check your explanation* in [HINTS.md](HINTS.md).

After the numbered lessons, do the **review** lesson (r01: earlier chapters, from memory) and the **code review** lesson (c01: fix a teammate's working-but-wrong code). Run `cargo progress` to see where you are and which chapters are due for spaced review.

Each lesson is independent, including the capstone, so later unfinished tasks do not block an earlier lesson.

| Done | Lesson | Coding tasks |
| --- | --- | --- |
| [ ] | [l01 · Expansion and evaluation once](l01_expansion.rs) | 2 |
| [ ] | [l02 · Repetition and separators](l02_repetition.rs) | 2 |
| [ ] | [l03 · Type and identifier fragments](l03_fragments.rs) | 3 |
| [ ] | [l04 · Recursive structured literals](l04_recursive.rs) | 4 |
| [ ] | [l05 · Traits handle macro leaf types](l05_traits.rs) | 2 |
| [ ] | [l06 · Exports and hygiene](l06_exports.rs) | 2 |
| [ ] | [l07 · Capstone: structured events](l07_capstone.rs) | 2 |
| [ ] | [R01 · Review: Chapters 15, 19, and 21 from memory](r01_review.rs) | 3 |
| [ ] | [C01 · Code review: a teammate's macro pull request](c01_code_review.rs) | 3 |
| [ ] | [Compiler drills](../../drills/ch22/) — `cargo drill ch22` | 10 |

```sh
cargo test --test chapter22 l01_
cargo test --test chapter22 -- --list
cargo test --test chapter22
cargo test --release --test chapter22
cargo clippy --test chapter22
cargo drill ch22
cargo run --example ch22_scratch
```

The starters compile; tests fail intentionally at `todo!()` and `guess()` until you fill them in. Keep the supplied tests intact. Every runnable task uses the standard library and needs no network. Free-form compiler experiments belong in [the scratch example](../../examples/ch22_scratch.rs); restore it to compiling code afterward. Dependency or platform labs have their own instructions and are outside the normal test target.

Use [progressive hints](HINTS.md) only after a first attempt.

## Coverage map

The section labels below follow the book's contents. A mapping to a lab means a concrete manual project, not automated coverage of an external library.

| Book sections | Practice |
| --- | --- |
| Macro Basics; Basics of Macro Expansion; Unintended Consequences | l01 expression expansion and evaluation once; c01 double evaluation; drills d03–d04 |
| Repetition | l02 repeated key/value pairs and trailing commas |
| Built-In Macros; Debugging Macros | l01 location macro; LABS.md expansion/diagnostics |
| Building the json! Macro; Fragment Types | l03 fragment contracts and l04 recursive data literals |
| Recursion in Macros; Using Traits with Macros | l04 and l05 trait-directed leaves |
| Scoping and Hygiene; Importing and Exporting Macros | l06 exported helper paths and local names; c01 `$crate` review; drills d02, d06, d08–d09 |
| Beyond macro_rules! | LABS.md isolated procedural derive/attribute project |
| Cross-section application | l07 event-construction capstone |

## Extension labs

- [LABS.md](LABS.md)

## Mastery check

1. How does macro expansion differ from calling a function, especially for evaluation count?
2. Which fragment types fit expressions, types, identifiers, token trees, and patterns?
3. Where can repetition be empty, and how do trailing commas affect matching?
4. How do recursive macro rules and traits divide syntax handling from type behavior?
5. Why do exported macros use $crate rather than relying on a caller import?
6. How does macro hygiene handle local variables and referenced items?
7. When does a procedural macro justify a separate proc-macro crate?

Redo the capstone from its contract in a later session. Explain one design tradeoff and create a case the supplied tests do not exercise.

## Validation scope

Use the repository's pinned Rust 1.99.0 toolchain and run commands from the repository root (see [setup instructions](../../README.md#fresh-clone-setup)). The dependency-free starter target can be checked without running TODOs using `cargo test --offline --test chapter22 --no-run`. No complete reference solution is shipped. Extension projects are manual practice and are outside the core compilation checks.
