# Chapter 15 — Iterators

[All chapters](../../README.md) · [Previous chapter](../chapter14/README.md) · [Next chapter](../chapter16/README.md)

Original guided exercises for [*Programming Rust*, 3rd edition, chapter 15](https://learning.oreilly.com/library/view/programming-rust-3rd/9781098176228/ch15.html).
Read the corresponding book sections alongside these exercises. These are new problems, not reproductions of the book’s examples.

This chapter has **11 lessons, 34 implementation tasks, and 35 automated checks**, plus prediction, compiler/manual, explanation, and transfer work.
The starter functions intentionally contain `todo!()`: they compile, and tests fail until you implement the contracts. Keep the supplied tests intact. Type signatures/trait scaffolds are provided so each lesson compiles independently; reconstruct their bounds in the compiler labs to practice design rather than only filling method bodies.

## Work through the chapter

For each lesson: predict before running; implement one task; run its filter; complete the lab in [the scratch example](../../examples/ch15_scratch.rs); explain the ownership/trait/API choices; add a transfer case.
Read [Hint 1 before Hint 2](HINTS.md) only if stuck. Passing tests alone does not demonstrate mastery.

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

Run commands from the repository root:

```sh
cargo test --test chapter15 --no-run
cargo test --test chapter15 -- --list
cargo run --example ch15_scratch
cargo test --test chapter15
cargo test --release --test chapter15
```

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

## Mastery check

- [ ] Explain laziness and the precise ownership of Item for iter, iter_mut, and into_iter.
- [ ] Choose adapters and consumers while predicting short-circuit and consumption behavior.
- [ ] Implement Iterator, DoubleEndedIterator, ExactSizeIterator, and FusedIterator consistently.
- [ ] Use by_ref and peekable without accidentally consuming delimiters.
- [ ] Explain why iterator pipelines need not allocate intermediate collections.
- [ ] Redo the capstone from its contract after a few days without consulting your earlier implementation.
- [ ] Present your predictions, failed compiler experiments, and one new edge case for review.
