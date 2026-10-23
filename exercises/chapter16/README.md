# Chapter 16 — Collections

[All chapters](../../README.md) · [Previous chapter](../chapter15/README.md) · [Next chapter](../chapter17/README.md)

Original guided exercises for [*Programming Rust*, 3rd edition, chapter 16](https://learning.oreilly.com/library/view/programming-rust-3rd/9781098176228/ch16.html).
Read the corresponding book sections alongside these exercises. These are new problems, not reproductions of the book’s examples.

This chapter has **9 lessons and 24 implementation tasks**, plus a review lesson, a code-review lesson, 10 compiler drills, and prediction, compiler/manual, explanation, and transfer work.
The starter functions intentionally contain `todo!()`: they compile, and tests fail until you implement the contracts. Keep the supplied tests intact. Type signatures/trait scaffolds are provided so each lesson compiles independently; reconstruct their bounds in the compiler labs to practice design rather than only filling method bodies.

## Work through the chapter

Every lesson uses the same loop. It is built on *retrieval practice*: you keep what you predict, struggle with, and explain, not what you reread.

1. **Read** the book section for the lesson.
2. **Predict.** Open the lesson and answer its `p_predictions` test: replace each `guess()` with a value. Don't run anything yet.
3. **Implement** the `todo!()` bodies from their contracts and run `cargo test --test chapter16 l01_`. A failed prediction is feedback: write in [NOTES.md](NOTES.md) why you were wrong.
4. **Check technique.** Some tests count heap allocations or compare buffer pointers, so a solution that copies into a new collection where the contract says *in place* fails. Then run `cargo clippy --test chapter16` and fix warnings in your code.
5. **Drill the compiler.** `cargo drill ch16` shows small programs; predict whether each compiles (and which error), then fix it.
6. **Explain** in [NOTES.md](NOTES.md), add one edge case of your own, complete the lesson's lab in [the scratch example](../../examples/ch16_scratch.rs), then compare with *Check your explanation* in [HINTS.md](HINTS.md).

After the numbered lessons, do the **review** lesson (r01: earlier chapters, from memory) and the **code review** lesson (c01: fix a teammate's working-but-wrong code). Run `cargo progress` to see where you are and which chapters are due for spaced review.

| Done | Lesson | Check |
| --- | --- | --- |
| [ ] | [01 · Vector access and structural editing](l01_vectors.rs) | `cargo test --test chapter16 l01_` |
| [ ] | [02 · Growth, removal, and borrow validity](l02_validity.rs) | `cargo test --test chapter16 l02_` |
| [ ] | [03 · Sorting, binary search, and slices](l03_sort_search.rs) | `cargo test --test chapter16 l03_` |
| [ ] | [04 · Queues and ring buffers](l04_deques.rs) | `cargo test --test chapter16 l04_` |
| [ ] | [05 · Priority queues and heap ordering](l05_heaps.rs) | `cargo test --test chapter16 l05_` |
| [ ] | [06 · Hash maps, tree maps, and entries](l06_maps.rs) | `cargo test --test chapter16 l06_` |
| [ ] | [07 · Set algebra and equal-but-distinct values](l07_sets.rs) | `cargo test --test chapter16 l07_` |
| [ ] | [08 · Hashing and custom builders](l08_hashing.rs) | `cargo test --test chapter16 l08_` |
| [ ] | [09 · Capstone: a task scheduler and audit](l09_capstone.rs) | `cargo test --test chapter16 l09_` |
| [ ] | [R01 · Review: Chapters 7, 10, 13–15 from memory](r01_review.rs) | `cargo test --test chapter16 r01_` |
| [ ] | [C01 · Code review: a teammate's pull request](c01_code_review.rs) | `cargo test --test chapter16 c01_` |
| [ ] | [Compiler drills](../../drills/ch16/) | `cargo drill ch16` |

Run commands from the repository root:

```sh
cargo test --test chapter16 l01_
cargo test --test chapter16 -- --list
cargo test --test chapter16
cargo test --release --test chapter16
cargo clippy --test chapter16
cargo drill ch16
cargo run --example ch16_scratch
```

Some labs deliberately fail to compile or panic. Keep them in scratch and restore a compiling example afterward. Do not put intentional compilation failures in the test modules. Tests are deterministic and require only the standard library; any separate dependency lab is described below.

## Coverage against the chapter

Every main topic is assigned to a coded lesson or concrete manual/compiler lab. A lesson reference includes its prompts and lab as well as its tests.

| Book topic | Practice |
| --- | --- |
| Collection overview and representation choices | 01 + mastery |
| Vec access; iteration; length/capacity; growth/shrinking; splitting/swapping/filling | 01–02 |
| Sorting/searching; slice comparisons | 03 |
| Random elements | random-lab/README.md (isolated rand package) |
| Preventing invalidation errors | 02 compiler lab |
| VecDeque | 04 |
| BinaryHeap | 05 |
| HashMap/BTreeMap; entries; map iteration | 06 |
| HashSet/BTreeSet; set iteration; equal but distinct values; set operations | 07 |
| Hashing; custom hash algorithms | 08 |
| Beyond standard collections | 09 design lab |
| Collection design integration | 09 capstone |

## Mastery check

- [ ] Choose Vec, VecDeque, BinaryHeap, hash, or tree collections by required operations.
- [ ] Predict which operations preserve order, invalidate references, or may reallocate.
- [ ] Use Entry to update maps and produce deterministic output without assuming hash iteration order.
- [ ] Keep Eq and Hash consistent, including equal keys with different payloads.
- [ ] Distinguish safe deterministic random tests from brittle hard-coded random sequences.
- [ ] Redo the capstone from its contract after a few days without consulting your earlier implementation.
- [ ] Present your predictions, failed compiler experiments, and one new edge case for review.

## Dependency lab

Complete [random-lab](random-lab/README.md) after the corresponding core lesson. It has its own manifest and commands, so it is not run by `cargo test --test chapter16`. Fetching its pinned dependency may require network access; all core checks remain offline.
