# Chapter 17 — Strings and Text

[All chapters](../../README.md) · [Previous chapter](../chapter16/README.md) · [Next chapter](../chapter18/README.md)

Original guided exercises for [*Programming Rust*, 3rd edition, chapter 17](https://learning.oreilly.com/library/view/programming-rust-3rd/9781098176228/ch17.html).
Read the corresponding book sections alongside these exercises. These are new problems, not reproductions of the book’s examples.

This chapter has **9 lessons, 28 implementation tasks, and 29 automated checks**, plus prediction, compiler/manual, explanation, and transfer work.
The starter functions intentionally contain `todo!()`: they compile, and tests fail until you implement the contracts. Keep the supplied tests intact. Type signatures/trait scaffolds are provided so each lesson compiles independently; reconstruct their bounds in the compiler labs to practice design rather than only filling method bodies.

## Work through the chapter

For each lesson: predict before running; implement one task; run its filter; complete the lab in [the scratch example](../../examples/ch17_scratch.rs); explain the ownership/trait/API choices; add a transfer case.
Read [Hint 1 before Hint 2](HINTS.md) only if stuck. Passing tests alone does not demonstrate mastery.

| Done | Lesson | Check |
| --- | --- | --- |
| [ ] | [01 · Unicode scalar operations and encoding](l01_unicode.rs) | `cargo test --test chapter17 l01_` |
| [ ] | [02 · Constructing and editing owned text](l02_editing.rs) | `cargo test --test chapter17 l02_` |
| [ ] | [03 · Search patterns, byte positions, and replacement](l03_search.rs) | `cargo test --test chapter17 l03_` |
| [ ] | [04 · Text iteration, trimming, and casing](l04_iteration.rs) | `cargo test --test chapter17 l04_` |
| [ ] | [05 · FromStr and textual conversion](l05_parsing.rs) | `cargo test --test chapter17 l05_` |
| [ ] | [06 · The formatting language](l06_formatting.rs) | `cargo test --test chapter17 l06_` |
| [ ] | [07 · Display, fmt::Write, and format_args!](l07_custom_format.rs) | `cargo test --test chapter17 l07_` |
| [ ] | [08 · UTF-8, borrowed text views, and delayed allocation](l08_representations.rs) | `cargo test --test chapter17 l08_` |
| [ ] | [09 · Capstone: a Unicode-aware report parser](l09_capstone.rs) | `cargo test --test chapter17 l09_` |

Run commands from the repository root:

```sh
cargo test --test chapter17 --no-run
cargo test --test chapter17 -- --list
cargo run --example ch17_scratch
cargo test --test chapter17
cargo test --release --test chapter17
```

Some labs deliberately fail to compile or panic. Keep them in scratch and restore a compiling example afterward. Do not put intentional compilation failures in the test modules. Tests are deterministic and require only the standard library; any separate dependency lab is described below.

## Coverage against the chapter

Every main topic is assigned to a coded lesson or concrete manual/compiler lab. A lesson reference includes its prompts and lab as well as its tests.

| Book topic | Practice |
| --- | --- |
| Unicode background: ASCII/Latin-1/Unicode; UTF-8; text directionality | 01 + manual lab |
| char classification; digits; case conversion; integer conversions | 01 |
| String/str creation and inspection; appending/insertion; removing/replacing | 02 |
| Search/iteration conventions; search patterns; searching/replacing | 03 |
| Text iteration; trimming; string case conversion | 04 |
| Parsing values from strings; converting values to strings | 05 |
| Borrowing text-like types; accessing UTF-8; constructing from UTF-8; delaying allocation; generic collection operations | 08 |
| Formatting text/numbers/other types; debug and pointer formatting; argument names/indices; dynamic width/precision | 06 |
| Formatting custom types; using formatting in an API | 07 |
| Regular expressions; basic matching/captures/replacement; lazy regex construction | regex-lab/README.md (isolated package) |
| Integrated text-processing design | 09 capstone |

## Mastery check

- [ ] Distinguish bytes, Unicode scalar values, grapheme clusters, display columns, and logical text order.
- [ ] Choose byte-safe slicing and identify when searching returns byte positions.
- [ ] Handle case expansion and invalid UTF-8 without losing ownership accidentally.
- [ ] Implement FromStr and Display with clear contracts and formatter error propagation.
- [ ] Build a regex once with LazyLock and choose is_match, find, or captures by required information.
- [ ] Redo the capstone from its contract after a few days without consulting your earlier implementation.
- [ ] Present your predictions, failed compiler experiments, and one new edge case for review.

## Dependency lab

Complete [regex-lab](regex-lab/README.md) after the corresponding core lesson. It has its own manifest and commands, so it is not run by `cargo test --test chapter17`. Fetching its pinned dependency may require network access; all core checks remain offline.
