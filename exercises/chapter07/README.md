# Chapter 7 — Error Handling

[All chapters](../../README.md) · [Previous chapter](../chapter06/README.md) · [Next chapter](../chapter08/README.md)

Original guided practice for [Programming Rust, 3rd edition, chapter 7](https://learning.oreilly.com/library/view/programming-rust-3rd/9781098176228/ch07.html). Read the corresponding section before each lesson. These exercises use original scenarios and contracts.

## Work through the chapter

For each lesson: record predictions before running code; implement only the TODO bodies; run its filtered tests; complete the compiler or transfer lab; explain the result in your own words; add one edge case. Passing tests does not replace the design and explanation work. Some requirements (such as avoiding clones or using a particular construct) need your own code review.

| Done | Lesson |
| --- | --- |
| [ ] | [01 · Panics and invariants](l01_panic.rs) |
| [ ] | [02 · Catching Result errors and printing context](l02_catching.rs) |
| [ ] | [03 · Propagating failures with question mark](l03_propagation.rs) |
| [ ] | [04 · Result methods and aliases](l04_methods_aliases.rs) |
| [ ] | [05 · Multiple error types at a boundary](l05_multiple_errors.rs) |
| [ ] | [06 · Custom errors with a source chain](l06_custom.rs) |
| [ ] | [07 · Impossible errors and deliberate recovery](l07_invariants_ignoring.rs) |
| [ ] | [08 · Capstone: fallible batch report](l08_capstone.rs) |

```sh
cargo test --test chapter07 l01_
cargo test --test chapter07 -- --list
cargo test --test chapter07
cargo test --release --test chapter07
cargo run --example ch07_scratch
```

The starters compile; tests fail intentionally at `todo!()` until implemented. Work in lesson order and keep tests intact. Every runnable task uses the standard library and needs no network. Compiler labs belong in [the scratch example](../../examples/ch07_scratch.rs), not in the exercise suite. Restore it to compiling code after each intentional error. Any package/tooling lab below is separate from the standard-library tests.

Use [progressive hints](HINTS.md) only after a first attempt. For review, share your predictions, implementation, and any diagnostic you cannot explain.

## Coverage map

| Book section or topic | Practice |
| --- | --- |
| Panic; unwinding; aborting | 01: invariant panic and scoped unwind lab |
| Panic hook; other panic macros | 01: isolated-process panic-hook and macro labs |
| Result; catching errors; printing errors | 02: match-based recovery and contextual messages |
| Propagating errors; Result methods; type aliases | 03: ? propagation; 04: combinators and alias |
| Working with multiple error types; anyhow | 05: boxed errors and optional anyhow package lab |
| Defining custom error types | 06: typed errors, Display and source chain |
| Errors that cannot happen; ignoring errors | 07: explicit invariant and selective handling |
| Handling errors in main; why Results | 08: capstone report and standalone main exit-status lab |

## Mastery check

- [ ] Choose Result for expected failures and panic for a documented broken invariant.
- [ ] Explain unwinding versus aborting and why catch_unwind is not general error handling.
- [ ] Propagate errors with ? and preserve contextual information and source chains.
- [ ] Use typed errors, boxed trait-object errors, or anyhow according to the boundary and recovery needs.
- [ ] Explain what ignoring an error means and make main return a useful exit status.
- [ ] Redo the final capstone after a break without looking at your implementation; invent a changed requirement and explain the tradeoff.

Use the book as the authority for language details. Manual labs deliberately include behavior that executable tests cannot establish, such as which program the borrow checker rejects or what an external crate's users can access.
