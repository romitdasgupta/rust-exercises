# Chapter 5 — References

[All chapters](../../README.md) · [Previous chapter](../chapter04/README.md) · [Next chapter](../chapter06/README.md)

Original guided practice for [Programming Rust, 3rd edition, chapter 5](https://learning.oreilly.com/library/view/programming-rust-3rd/9781098176228/ch05.html). Read the corresponding section before each lesson. These exercises use original scenarios and contracts.

## Work through the chapter

Every lesson uses the same loop. It is built on *retrieval practice*: you keep what you predict, struggle with, and explain, not what you reread.

1. **Read** the book section for the lesson.
2. **Predict.** Open the lesson and answer its `p_predictions` test: replace each `guess()` with a value. Don't run anything yet.
3. **Implement** the `todo!()` bodies from their contracts and run `cargo test --test chapter05 l01_`. A failed prediction is feedback: write in [NOTES.md](NOTES.md) why you were wrong.
4. **Check technique.** Some tests count heap allocations or compare pointers, so a clone-everything solution fails. Then run `cargo clippy --test chapter05` and fix warnings in your code.
5. **Drill the compiler.** `cargo drill ch05` shows small programs; predict whether each compiles (and which error), then fix it.
6. **Explain** in [NOTES.md](NOTES.md), add one edge-case test, then compare with *Check your explanation* in [HINTS.md](HINTS.md).

After the numbered lessons, do the **review** lesson (r01: earlier chapters, from memory) and the **code review** lesson (c01: fix a teammate's working-but-wrong code). Run `cargo progress` to see where you are and which chapters are due for spaced review.

| Done | Lesson |
| --- | --- |
| [ ] | [01 · Borrowing and dereferencing](l01_basics.rs) |
| [ ] | [02 · Reference bindings and identity](l02_identity.rs) |
| [ ] | [03 · Optional references and fat pointers](l03_optional_fat.rs) |
| [ ] | [04 · Lifetimes through functions](l04_lifetime_flow.rs) |
| [ ] | [05 · Structs that borrow](l05_borrowed_structs.rs) |
| [ ] | [06 · Independent lifetimes and elision](l06_distinct_lifetimes.rs) |
| [ ] | [07 · Sharing versus mutation](l07_exclusive.rs) |
| [ ] | [08 · Disjoint mutable borrows](l08_disjoint.rs) |
| [ ] | [09 · Capstone: borrowed roster queries](l09_capstone.rs) |
| [ ] | [R01 · Review: Chapters 3 and 4 from memory](r01_review.rs) |
| [ ] | [C01 · Code review: a teammate's pull request](c01_code_review.rs) |
| [ ] | [Compiler drills](../../drills/ch05/) — `cargo drill ch05` |

```sh
cargo test --test chapter05 l01_
cargo test --test chapter05 -- --list
cargo test --test chapter05
cargo test --release --test chapter05
cargo clippy --test chapter05
cargo drill ch05
cargo run --example ch05_scratch
```

The starters compile; tests fail intentionally at `todo!()` and `guess()` until you fill them in. Work in lesson order and keep tests intact. Every runnable task uses the standard library and needs no network. Compiler labs belong in [the scratch example](../../examples/ch05_scratch.rs), not in the exercise suite. Restore it to compiling code after each intentional error. Any package/tooling lab below is separate from the standard-library tests.

Use [progressive hints](HINTS.md) only after a first attempt. For review, share your predictions, implementation, and any diagnostic you cannot explain.

## Coverage map

| Book section or topic | Practice |
| --- | --- |
| Reference basics; implicit and explicit references | 01: borrowing, dereferencing, implicit method receivers |
| Assigning references; references to references; comparing references | 02: rebinding, nested borrows, content versus identity |
| References are never null; arbitrary expressions; fat pointers | 03: Option, temporary borrows, slice metadata lab |
| Borrowing a local variable; receiving and passing references | 04: lifetime flow and invalid-local-return lab |
| Returning references; structs containing references | 05: borrowed views and owner lifetime lab |
| Distinct lifetime parameters; omitting lifetime parameters | 06: independent lifetimes and elision compiler lab |
| Sharing versus mutation | 07: exclusive access and last-use lab; 08: disjoint slices |
| Taking arms against a sea of objects | 09: capstone index-based selection and ownership design lab |

## Mastery check

- [ ] Distinguish changing a reference from changing its referent.
- [ ] Explain automatic dereferencing, value equality versus pointer identity, and the metadata in slice references.
- [ ] Reject references that can outlive their owner and annotate relationships rather than extending lifetimes.
- [ ] Use distinct lifetimes for independently borrowed fields and explain the elision rules.
- [ ] Resolve borrow conflicts with last-use boundaries or disjoint access, without unnecessary cloning.
- [ ] Redo the final capstone after a break without looking at your implementation; invent a changed requirement and explain the tradeoff.

Use the book as the authority for language details. Manual labs deliberately include behavior that executable tests cannot establish, such as which program the borrow checker rejects or what an external crate's users can access.
