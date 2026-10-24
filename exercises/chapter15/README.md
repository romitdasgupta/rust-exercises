# Chapter 15 — Iterators

[All chapters](../../README.md) · [Previous chapter](../chapter14/README.md) · [Next chapter](../chapter16/README.md)

Original guided exercises for [*Programming Rust*, 3rd edition, chapter 15](https://learning.oreilly.com/library/view/programming-rust-3rd/9781098176228/ch15.html).
Read the corresponding book sections alongside these exercises. These are new problems, not reproductions of the book’s examples.

This chapter has **11 lessons and 34 implementation tasks**, a review lesson, a code-review lesson, and 10 compiler drills, with **60 automated checks**, plus prediction, compiler/manual, explanation, and transfer work.
The starter functions intentionally contain `todo!()`: they compile, and tests fail until you implement the contracts. Keep the supplied tests intact. Type signatures/trait scaffolds are provided so each lesson compiles independently; reconstruct their bounds in the compiler labs to practice design rather than only filling method bodies.

## Work through the chapter

Every lesson uses the same loop. It is built on *retrieval practice*: you keep what you predict, struggle with, and explain, not what you reread.

1. **Read** the book section for the lesson.
2. **Predict.** Open the lesson and answer its `p_predictions` test: replace each `guess()` with a value. Don't run anything yet.
3. **Implement** the `todo!()` bodies from their contracts and run `cargo test --test chapter15 l01_`. A failed prediction is feedback: write in [NOTES.md](NOTES.md) why you were wrong.
4. **Check technique.** Some tests count heap allocations or compare pointers, so a solution that collects intermediate vectors or clones items fails. Then run `cargo clippy --test chapter15` and fix warnings in your code.
5. **Drill the compiler.** `cargo drill ch15` shows small programs; predict whether each compiles (and which error), then fix it.
6. **Explain** in [NOTES.md](NOTES.md), complete the lesson's lab in [the scratch example](../../examples/ch15_scratch.rs), add one edge-case test, then compare with *Check your explanation* in [HINTS.md](HINTS.md).

After the numbered lessons, do the **review** lesson (r01: earlier chapters, from memory) and the **code review** lesson (c01: fix a teammate's working-but-wrong code). Run `cargo progress` to see where you are and which chapters are due for spaced review.

| Done | Lesson | Check |
| --- | --- | --- |
| [ ] | [01 · Borrowed, mutable, owned, and draining sources](l01_sources.rs) | `cargo test --test chapter15 l01_` |
| [ ] | [02 · Generating finite sequences](l02_generated.rs) | `cargo test --test chapter15 l02_` |
| [ ] | [03 · Mapping, filtering, and flattening](l03_mapping.rs) | `cargo test --test chapter15 l03_` |
| [ ] | [04 · Stopping, peeking, and resuming](l04_boundaries.rs) | `cargo test --test chapter15 l04_` |
| [ ] | [05 · Reversal, pairing, inspection, and repetition](l05_composition.rs) | `cargo test --test chapter15 l05_` |
| [ ] | [06 · Accumulators, extrema, and sequence comparisons](l06_aggregation.rs) | `cargo test --test chapter15 l06_` |
| [ ] | [07 · Searching and iterator positions](l07_search.rs) | `cargo test --test chapter15 l07_` |
| [ ] | [08 · Folds, short circuits, and side effects](l08_folds.rs) | `cargo test --test chapter15 l08_` |
| [ ] | [09 · Collections, extension, and partition](l09_collecting.rs) | `cargo test --test chapter15 l09_` |
| [ ] | [10 · Implementing a double-ended iterator](l10_custom.rs) | `cargo test --test chapter15 l10_` |
| [ ] | [11 · Capstone: a lazy measurement report](l11_capstone.rs) | `cargo test --test chapter15 l11_` |
| [ ] | [R01 · Review: Chapters 10–14 from memory](r01_review.rs) | `cargo test --test chapter15 r01_` |
| [ ] | [C01 · Code review: a teammate's pull request](c01_code_review.rs) | `cargo test --test chapter15 c01_` |
| [ ] | [Compiler drills](../../drills/ch15/) | `cargo drill ch15` |

Run commands from the repository root:

```sh
cargo test --test chapter15 l01_
cargo test --test chapter15 -- --list
cargo test --test chapter15
cargo test --release --test chapter15
cargo clippy --test chapter15
cargo drill ch15
cargo run --example ch15_scratch
```

The starters compile; tests fail intentionally at `todo!()` and `guess()` until you fill them in. Keep the supplied tests intact. Use [progressive hints](HINTS.md) only after a first attempt; passing tests alone does not demonstrate mastery.

Some labs deliberately fail to compile or panic. Keep them in scratch and restore a compiling example afterward. Do not put intentional compilation failures in the test modules. Tests are deterministic and require only the standard library; any separate dependency lab is described below.

## Coverage against the chapter

Every main topic is assigned to a coded lesson or concrete manual/compiler lab. A lesson reference includes its prompts and lab as well as its tests.

| Book topic | Practice |
| --- | --- |
| Iterator and IntoIterator; iter and iter_mut; ownership; drain; sources | 01 |
| from_fn; successors | 02 |
| map; filter; filter_map; flat_map; flatten | 03 |
| take; take_while; skip; skip_while; peekable; fuse; by_ref | 04 |
| Reversible iterators; rev; inspect; chain; enumerate; zip; cloned; copied; cycle | 05 |
| count; sum; product; extrema and key/comparator extrema; sequence comparison; any/all | 06 |
| position/rposition; ExactSizeIterator; nth/nth_back; last; find/rfind/find_map | 07 |
| fold/rfold; try_fold/try_rfold; for_each/try_for_each | 08 |
| collect/FromIterator; Extend; partition | 09 |
| Implementing iterators | 10 |
| Combining lazy adapters and consumers | 11 capstone |
| Reading iterator diagnostics (laziness, item types, missing traits) | [drills](../../drills/ch15/) d01–d10 |

## Mastery check

- [ ] Explain laziness and the precise ownership of Item for iter, iter_mut, and into_iter.
- [ ] Choose adapters and consumers while predicting short-circuit and consumption behavior.
- [ ] Implement Iterator, DoubleEndedIterator, ExactSizeIterator, and FusedIterator consistently.
- [ ] Use by_ref and peekable without accidentally consuming delimiters.
- [ ] Explain why iterator pipelines need not allocate intermediate collections.
- [ ] Redo the capstone from its contract after a few days without consulting your earlier implementation.
- [ ] Present your predictions, failed compiler experiments, and one new edge case for review.
