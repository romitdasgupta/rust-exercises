# Chapter 24 — Foreign Functions

[All chapters](../../README.md) · [Previous chapter](../chapter23/README.md)

Original exercises accompanying [Programming Rust, 3rd edition, chapter 24](https://learning.oreilly.com/library/view/programming-rust-3rd/9781098176228/ch24.html). Read the corresponding section, then practice without copying the book's examples.

## Working route

Predict on paper first. Implement one lesson's TODOs, run its filtered tests, do its scratch experiment, and explain the result in your own words. Add one edge case of your own. Open [HINTS.md](HINTS.md) only after an attempt. Passing tests is one part of mastery; the explanation and labs matter too.

The starter compiles; unfinished TODOs intentionally fail when reached. Keep the supplied tests. Each lesson is independent, including the capstone, so later unfinished tasks do not block an earlier lesson.

| Done | Lesson | Coding tasks |
| --- | --- | --- |
| [ ] | [l01 · C representations](l01_representations.rs) | 2 |
| [ ] | [l02 · C strings and ownership](l02_c_strings.rs) | 3 |
| [ ] | [l03 · ABI calls and out-parameters](l03_abi_status.rs) | 2 |
| [ ] | [l04 · Callbacks and userdata](l04_callbacks.rs) | 2 |
| [ ] | [l05 · Opaque handle ownership](l05_handles.rs) | 2 |
| [ ] | [l06 · Owner-bound foreign borrows](l06_borrows.rs) | 2 |
| [ ] | [l07 · Capstone: checked foreign transaction](l07_capstone.rs) | 2 |

```sh
cargo test --test chapter24 l01_
cargo test --test chapter24 -- --list
# After completing all lessons:
cargo test --test chapter24
cargo test --release --test chapter24
cargo run --example ch24_scratch
```

Use [the scratch example](../../examples/ch24_scratch.rs) for experiments and restore it to a compiling state afterward. Core lessons use only the standard library. Dependency or platform labs have their own instructions and are outside the normal test target.

## Coverage map

The section labels below follow the book's contents. A mapping to a lab means a concrete manual project, not automated coverage of an external library.

| Book sections | Practice |
| --- | --- |
| Finding Common Data Representations | l01 C ABI types/layout; l02 C strings; LABS.md actual C headers |
| Declaring Foreign Functions and Variables | l03 ABI function pointers and status/out-parameters; LABS.md extern/static declarations |
| Using Functions from Libraries | l04 callback/userdata boundary; LABS.md real C linker project |
| A Raw Interface to libgit2 | l05 typed opaque handle model; LABS.md actual libgit2 declarations/errors |
| A Safe Interface to libgit2 | l05 RAII owner and l06 lifetime-bound borrow; LABS.md real repository wrapper |
| Conclusion | l07 status/result capstone and complete boundary audit |

## Extension labs

- [LABS.md](LABS.md)

## Mastery check

1. How do repr(C), C primitive aliases, size_t, pointers, and ABI declarations agree across languages?
2. Why do CString and CStr have different ownership/lifetime roles?
3. When is a foreign output parameter initialized, and what may be read after failure?
4. Which allocator frees each foreign object, and how is cleanup made exception safe?
5. How do borrowed foreign objects retain their owner’s lifetime?
6. How do callbacks retain userdata lifetime and prevent unwinding through a C ABI boundary?
7. Which initialization/shutdown, threading, error-message, version, and linker obligations must a safe wrapper document?

Redo the capstone from its contract in a later session. Explain one design tradeoff and create a case the supplied tests do not exercise.

## Validation scope

Use the repository's pinned Rust 1.99.0 toolchain and run commands from the repository root (see [setup instructions](../../README.md#fresh-clone-setup)). The dependency-free starter target can be checked without running TODOs using `cargo test --offline --test chapter24 --no-run`. No complete reference solution is shipped. Extension projects are manual practice and are outside the core compilation checks.
