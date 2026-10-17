# Chapter 9 — Structs

[All chapters](../../README.md) · [Previous chapter](../chapter08/README.md) · [Next chapter](../chapter10/README.md)

Original guided practice for [Programming Rust, 3rd edition, chapter 9](https://learning.oreilly.com/library/view/programming-rust-3rd/9781098176228/ch09.html). Read the corresponding section before each lesson. These exercises use original scenarios and contracts.

## Work through the chapter

For each lesson: record predictions before running code; implement only the TODO bodies; run its filtered tests; complete the compiler or transfer lab; explain the result in your own words; add one edge case. Passing tests does not replace the design and explanation work. Some requirements (such as avoiding clones or using a particular construct) need your own code review.

| Done | Lesson |
| --- | --- |
| [ ] | [01 · Named structs and update syntax](l01_named.rs) |
| [ ] | [02 · Tuple structs, unit structs and layout](l02_kinds_layout.rs) |
| [ ] | [03 · Methods and associated items](l03_methods.rs) |
| [ ] | [04 · Smart-pointer self receivers](l04_smart_self.rs) |
| [ ] | [05 · Generic structs and impl blocks](l05_generics.rs) |
| [ ] | [06 · Lifetimes and const parameters](l06_lifetime_const.rs) |
| [ ] | [07 · Deriving common traits](l07_derive.rs) |
| [ ] | [08 · Cell and RefCell](l08_interior.rs) |
| [ ] | [09 · Capstone: validated inventory](l09_capstone.rs) |

```sh
cargo test --test chapter09 l01_
cargo test --test chapter09 -- --list
cargo test --test chapter09
cargo test --release --test chapter09
cargo run --example ch09_scratch
```

The starters compile; tests fail intentionally at `todo!()` until implemented. Work in lesson order and keep tests intact. Every runnable task uses the standard library and needs no network. Compiler labs belong in [the scratch example](../../examples/ch09_scratch.rs), not in the exercise suite. Restore it to compiling code after each intentional error. Any package/tooling lab below is separate from the standard-library tests.

Use [progressive hints](HINTS.md) only after a first attempt. For review, share your predictions, implementation, and any diagnostic you cannot explain.

## Coverage map

| Book section or topic | Practice |
| --- | --- |
| Defining structs | 01: named fields, shorthand, update syntax and partial moves |
| Other kinds: tuple and unit-like structs; struct layout | 02: distinct types and unspecified layout lab |
| Defining methods with impl; associated functions/constants | 03: self/&self/&mut self and validated construction |
| Passing Self as Box, Rc, Arc | 04: owned smart-pointer receiver methods |
| Generic structs | 05: reusable owned storage |
| Generic structs with lifetimes; constant parameters | 06: borrowed fixed-length view |
| Deriving common traits | 07: derived Clone/Debug/PartialEq/Eq/Default and Copy limitations |
| Interior mutability | 08: Cell/RefCell runtime access rules |
| Chapter integration | 09: capstone validated inventory with interior statistics |

## Mastery check

- [ ] Choose named, tuple, and unit structs and explain why tuple structs create distinct types.
- [ ] Keep invariants behind methods and choose owned, shared, or exclusive receivers deliberately.
- [ ] Use smart-pointer self receivers without accidentally cloning the underlying data.
- [ ] Combine type, lifetime, and const parameters and explain the role of each.
- [ ] State what derived traits do and which field traits they require.
- [ ] Use Cell and RefCell while explaining runtime borrow errors and the absence of arbitrary shared mutation.
- [ ] Redo the final capstone after a break without looking at your implementation; invent a changed requirement and explain the tradeoff.

Use the book as the authority for language details. Manual labs deliberately include behavior that executable tests cannot establish, such as which program the borrow checker rejects or what an external crate's users can access.
