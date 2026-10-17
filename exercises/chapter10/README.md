# Chapter 10 — Enums and Patterns

[All chapters](../../README.md) · [Previous chapter](../chapter09/README.md) · [Next chapter](../chapter11/README.md)

Original guided practice for [Programming Rust, 3rd edition, chapter 10](https://learning.oreilly.com/library/view/programming-rust-3rd/9781098176228/ch10.html). Read the corresponding section before each lesson. These exercises use original scenarios and contracts.

## Work through the chapter

For each lesson: record predictions before running code; implement only the TODO bodies; run its filtered tests; complete the compiler or transfer lab; explain the result in your own words; add one edge case. Passing tests does not replace the design and explanation work. Some requirements (such as avoiding clones or using a particular construct) need your own code review.

| Done | Lesson |
| --- | --- |
| [ ] | [01 · Variants and payloads](l01_variants.rs) |
| [ ] | [02 · Recursive data with enums](l02_recursive.rs) |
| [ ] | [03 · Generic enums](l03_generic.rs) |
| [ ] | [04 · Literal, tuple and struct patterns](l04_structural.rs) |
| [ ] | [05 · Matching references and mutating payloads](l05_reference_patterns.rs) |
| [ ] | [06 · Array and slice patterns](l06_slice_patterns.rs) |
| [ ] | [07 · Guards, alternatives and @ bindings](l07_guards_or_at.rs) |
| [ ] | [08 · Pattern contexts and public compatibility](l08_contexts_public.rs) |
| [ ] | [09 · Capstone: generic ordered tree](l09_capstone.rs) |

```sh
cargo test --test chapter10 l01_
cargo test --test chapter10 -- --list
cargo test --test chapter10
cargo test --release --test chapter10
cargo run --example ch10_scratch
```

The starters compile; tests fail intentionally at `todo!()` until implemented. Work in lesson order and keep tests intact. Every runnable task uses the standard library and needs no network. Compiler labs belong in [the scratch example](../../examples/ch10_scratch.rs), not in the exercise suite. Restore it to compiling code after each intentional error. Any package/tooling lab below is separate from the standard-library tests.

Use [progressive hints](HINTS.md) only after a first attempt. For review, share your predictions, implementation, and any diagnostic you cannot explain.

## Coverage map

| Book section or topic | Practice |
| --- | --- |
| Enums; enums with data; enums in memory | 01: unit/data variants and layout lab |
| Rich structures using enums | 02: recursive expression tree |
| Generic enums | 03: type-parameterized alternatives |
| Literals, variables, wildcards; tuple/struct patterns | 04: structural destructuring and shadowing lab |
| Matching references; reference patterns | 05: borrowed/mutable match ergonomics and Rust 2024 lab |
| Array and slice patterns | 06: fixed-size and variable-length decomposition |
| Match guards; multiple possibilities; @ bindings | 07: guarded alternatives and whole-value bindings |
| Where patterns are allowed; public enums/compatibility | 08: binding contexts and external non_exhaustive lab |
| Populating a binary tree; big picture | 09: generic tree capstone |

## Mastery check

- [ ] Represent mutually exclusive states with variants and choose payload ownership deliberately.
- [ ] Explain recursive indirection and avoid assumptions about default enum layout.
- [ ] Destructure owned, borrowed, and mutably borrowed variants and track moves.
- [ ] Use slice patterns, guards, or-patterns and @ without introducing accidental catch-all bindings.
- [ ] Explain exhaustiveness, the role of wildcard arms, and non_exhaustive public API evolution.
- [ ] Build and query a recursive binary tree using patterns and compare its worst-case shape.
- [ ] Redo the final capstone after a break without looking at your implementation; invent a changed requirement and explain the tradeoff.

Use the book as the authority for language details. Manual labs deliberately include behavior that executable tests cannot establish, such as which program the borrow checker rejects or what an external crate's users can access.
