# Chapter 8 — Crates and Modules

[All chapters](../../README.md) · [Previous chapter](../chapter07/README.md) · [Next chapter](../chapter09/README.md)

Original guided practice for [Programming Rust, 3rd edition, chapter 8](https://learning.oreilly.com/library/view/programming-rust-3rd/9781098176228/ch08.html). Read the corresponding section before each lesson. These exercises use original scenarios and contracts.

## Work through the chapter

Every lesson uses the same loop. It is built on *retrieval practice*: you keep what you predict, struggle with, and explain, not what you reread.

1. **Read** the book section for the lesson.
2. **Predict.** Open the lesson and answer its `p_predictions` test: replace each `guess()` with a value. Don't run anything yet.
3. **Implement** the `todo!()` bodies from their contracts and run `cargo test --test chapter08 l01_`. A failed prediction is feedback: write in [NOTES.md](NOTES.md) why you were wrong.
4. **Check technique.** Some tests count heap allocations or compare pointers, so a solution that copies everything fails. Then run `cargo clippy --test chapter08` and fix warnings in your code.
5. **Drill the compiler.** `cargo drill ch08` shows small programs; predict whether each compiles (and which error), then fix it.
6. **Explain** in [NOTES.md](NOTES.md), add one edge-case test, then compare with *Check your explanation* in [HINTS.md](HINTS.md).

After the numbered lessons, do the **review** lesson (r01: earlier chapters, from memory) and the **code review** lesson (c01: fix a teammate's working-but-wrong code). Run `cargo progress` to see where you are and which chapters are due for spaced review.

| Done | Lesson |
| --- | --- |
| [ ] | [01 · Crates, editions and profiles](l01_crates_profiles.rs) |
| [ ] | [02 · Nested modules and private implementation](l02_modules.rs) |
| [ ] | [03 · Paths, imports and reexports](l03_paths_exports.rs) |
| [ ] | [04 · Constants and statics](l04_const_static.rs) |
| [ ] | [05 · Attributes, cfg and included files](l05_attributes.rs) |
| [ ] | [06 · Tests and executable documentation](l06_tests_docs.rs) |
| [ ] | [07 · Dependency requirements and lockfiles](l07_dependencies.rs) |
| [ ] | [08 · Capstone: library facade and workspace](l08_capstone.rs) |
| [ ] | [R01 · Review: Chapters 4, 6 and 7 from memory](r01_review.rs) |
| [ ] | [C01 · Code review: a teammate's pull request](c01_code_review.rs) |
| [ ] | [Compiler drills](../../drills/ch08/) — `cargo drill ch08` |

```sh
cargo test --test chapter08 l01_
cargo test --test chapter08 -- --list
cargo test --test chapter08
cargo test --release --test chapter08
cargo clippy --test chapter08
cargo drill ch08
cargo run --example ch08_scratch
```

The starters compile; tests fail intentionally at `todo!()` and `guess()` until you fill them in. Work in lesson order and keep tests intact. Every runnable task uses the standard library and needs no network. Compiler labs belong in [the scratch example](../../examples/ch08_scratch.rs), not in the exercise suite. Restore it to compiling code after each intentional error. Any package/tooling lab below is separate from the standard-library tests.

Use [progressive hints](HINTS.md) only after a first attempt.

## Coverage map

| Book section or topic | Practice |
| --- | --- |
| Crates; editions; Rust versions; build profiles | 01: profile task and Cargo metadata/toolchain lab |
| Modules; nested modules; modules in separate files | 02: module API and file split lab |
| Paths/imports; standard prelude; pub use; public struct fields | 03: reexports and privacy labs |
| Constants and statics | 04: compile-time values and stable static identity |
| Turning a program into a library; src/bin | 08: library-shaped capstone plus external consumer lab |
| Attributes; conditional compilation; build environment; including files | 05: cfg, env! and include_str; build-script lab |
| Tests/documentation; integration tests; doc-tests | 06: tested API and executable documentation lab |
| Dependencies; versions; Cargo.lock; build scripts | 07: dependency/tooling package lab |
| Publishing; workspaces; more nice things | 08: local package/workspace audit lab; cargo fmt/check/clippy |

## Mastery check

- [ ] Explain crate versus module versus package, and make a multi-target package with a library.
- [ ] Control visibility with private fields and public reexports; prove it from an external consumer crate.
- [ ] Split modules into files without changing their public paths.
- [ ] Distinguish const from static and compile-time cfg from a runtime if.
- [ ] Write unit, integration, and doc tests; explain lockfiles, dependency requirements, build scripts, and workspaces.
- [ ] Inspect a local package archive without publishing anything.
- [ ] Redo the final capstone after a break without looking at your implementation; invent a changed requirement and explain the tradeoff.

Use the book as the authority for language details. Manual labs deliberately include behavior that executable tests cannot establish, such as which program the borrow checker rejects or what an external crate's users can access.
