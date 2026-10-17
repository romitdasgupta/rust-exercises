# Chapter 4 — Ownership and Moves

[All chapters](../../README.md) · [Previous chapter](../chapter03/README.md) · [Next chapter](../chapter05/README.md)

Original guided practice for [Programming Rust, 3rd edition, chapter 4](https://learning.oreilly.com/library/view/programming-rust-3rd/9781098176228/ch04.html). Read the corresponding section before each lesson. These exercises use original scenarios and contracts.

## Work through the chapter

For each lesson: record predictions before running code; implement only the TODO bodies; run its filtered tests; complete the compiler or transfer lab; explain the result in your own words; add one edge case. Passing tests does not replace the design and explanation work. Some requirements (such as avoiding clones or using a particular construct) need your own code review.

| Done | Lesson |
| --- | --- |
| [ ] | [01 · Ownership scopes](l01_scope.rs) |
| [ ] | [02 · Moves across assignments and fields](l02_move_paths.rs) |
| [ ] | [03 · Moves and control flow](l03_control_flow.rs) |
| [ ] | [04 · Moving from indexed collections](l04_indexed.rs) |
| [ ] | [05 · Leaving a replacement behind](l05_replace.rs) |
| [ ] | [06 · Copy and explicit cloning](l06_copy_clone.rs) |
| [ ] | [07 · Rc and Arc](l07_shared.rs) |
| [ ] | [08 · Capstone: owned dispatch queue](l08_capstone.rs) |

```sh
cargo test --test chapter04 l01_
cargo test --test chapter04 -- --list
cargo test --test chapter04
cargo test --release --test chapter04
cargo run --example ch04_scratch
```

The starters compile; tests fail intentionally at `todo!()` until implemented. Work in lesson order and keep tests intact. Every runnable task uses the standard library and needs no network. Compiler labs belong in [the scratch example](../../examples/ch04_scratch.rs), not in the exercise suite. Restore it to compiling code after each intentional error. Any package/tooling lab below is separate from the standard-library tests.

Use [progressive hints](HINTS.md) only after a first attempt. For review, share your predictions, implementation, and any diagnostic you cannot explain.

## Coverage map

| Book section or topic | Practice |
| --- | --- |
| Ownership | 01: trace scope and drop; lab allocation diagrams |
| Moves; more operations that move | 02: assignment, arguments, returns, tuple fields |
| Moves and control flow | 03: branch ownership and loop handoffs |
| Moves and indexed content | 04: pop/remove/swap_remove; 05: replace/take |
| Copy types: the exception to moves | 06: copying values and explicit clone; derive lab |
| Rc and Arc: shared ownership | 07: pointer sharing/counts; 08: immutable registry capstone and Arc thread lab |

## Mastery check

- [ ] Draw stack/heap ownership before and after a move, clone, and drop.
- [ ] Explain which operations invalidate the source binding and why a moved binding can be reassigned.
- [ ] Extract an owned collection element without leaving uninitialized storage.
- [ ] Explain why Copy cannot be derived for String and why cloning Rc differs from cloning its value.
- [ ] Distinguish Rc and Arc, shared ownership and mutable access, and reference cycles.
- [ ] Redo the final capstone after a break without looking at your implementation; invent a changed requirement and explain the tradeoff.

Use the book as the authority for language details. Manual labs deliberately include behavior that executable tests cannot establish, such as which program the borrow checker rejects or what an external crate's users can access.
