# Chapter 23 — Unsafe Code

[All chapters](../../README.md) · [Previous chapter](../chapter22/README.md) · [Next chapter](../chapter24/README.md)

Original exercises accompanying [Programming Rust, 3rd edition, chapter 23](https://learning.oreilly.com/library/view/programming-rust-3rd/9781098176228/ch23.html). Read the corresponding section, then practice without copying the book's examples.

## Working route

Predict on paper first. Implement one lesson's TODOs, run its filtered tests, do its scratch experiment, and explain the result in your own words. Add one edge case of your own. Open [HINTS.md](HINTS.md) only after an attempt. Passing tests is one part of mastery; the explanation and labs matter too.

The starter compiles; unfinished TODOs intentionally fail when reached. Keep the supplied tests. Each lesson is independent, including the capstone, so later unfinished tasks do not block an earlier lesson.

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

```sh
cargo test --test chapter23 l01_
cargo test --test chapter23 -- --list
# After completing all lessons:
cargo test --test chapter23
cargo test --release --test chapter23
cargo run --example ch23_scratch
```

Use [the scratch example](../../examples/ch23_scratch.rs) for experiments and restore it to a compiling state afterward. Core lessons use only the standard library. Dependency or platform labs have their own instructions and are outside the normal test target.

## Coverage map

The section labels below follow the book's contents. A mapping to a lab means a concrete manual project, not automated coverage of an external library.

| Book sections | Practice |
| --- | --- |
| Unsafe from What?; unsafe Blocks; Marking Unsafe Code; Undefined Behavior | l01 invariant boundary and LABS.md audit |
| Example: An Efficient ASCII String Type | l01 original Ascii wrapper |
| Unsafe Functions; Raw Pointers; Dereferencing Raw Pointers Safely; Null Pointers | l02 raw read and l03 checked raw slice split |
| Example: RefWithFlag | l04 provenance-preserving tagged borrow |
| Unsafe Traits; Type Sizes and Alignments | l05 representation contracts/layout |
| Pointer Arithmetic; Moving Into and Out of Memory | l03 split and l06 ownership moves |
| Example: GapBuffer | l07 fixed-capacity byte gap buffer; LABS.md generic extension |
| Panic Safety in Unsafe Code | l06, l08 drop accounting and LABS.md panic audit |
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

The dependency-free starter target compiles on the installed Rust 1.99 toolchain. Temporary complete reference implementations pass the chapter tests in debug and release; reference answers are not saved in this exercise directory. External-runtime, procedural-macro, native-library, platform-specific, and Miri extension projects are assigned practice, and were not executed as part of this validation.
