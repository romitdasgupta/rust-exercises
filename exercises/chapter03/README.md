# Programming Rust practice: Chapter 3 — Fundamental Types

[All chapters](../../README.md) · [Next chapter](../chapter04/README.md)

Original exercises accompanying [Chapter 3 of *Programming Rust*, 3rd edition](https://learning.oreilly.com/library/view/programming-rust-3rd/9781098176228/ch03.html).
The book is your reference; these exercises are a place to practice and explain its ideas.

## Work through the chapter

Every lesson uses the same loop. It is built on *retrieval practice*: you keep what you predict, struggle with, and explain, not what you reread.

1. **Read** the book section for the lesson.
2. **Predict.** Open the lesson and answer its `p_predictions` test: replace each `guess()` with a value. Don't run anything yet.
3. **Implement** the `todo!()` bodies from their contracts and run `cargo test --test chapter03 l01_`. A failed prediction is feedback: write in [NOTES.md](NOTES.md) why you were wrong.
4. **Check technique.** Some tests count heap allocations or compare pointers, so a copy-everything solution fails. Then run `cargo clippy --test chapter03` and fix warnings in your code.
5. **Drill the compiler.** `cargo drill ch03` shows small programs; predict whether each compiles (and which error), then fix it.
6. **Explain** in [NOTES.md](NOTES.md), add one edge-case test, then compare with *Check your explanation* in [HINTS.md](HINTS.md).

After the numbered lessons, do the **code review** lesson (c01: fix a teammate's working-but-wrong code). From Chapter 4 on, each chapter also opens with a review lesson that brings this chapter back from memory. Run `cargo progress` to see where you are and which chapters are due for spaced review.

The unfinished exercises compile, but their tests fail intentionally at `todo!()` and `guess()`. Filtering by lesson runs only that lesson. Leave the tests intact; you can add cases of your own. If stuck, start with the relevant section of the chapter, then open [HINTS.md](HINTS.md). There is no answer key in the exercise files. For guided review with a study partner or tutor, share the lesson number, your predictions and explanations, and your implementation or test failure.

## Route through the chapter

Check a lesson off after its tests **and** explanation prompts are complete. A lesson can take more than one sitting; the overflow and pointer lessons are deliberately denser.

| Done | Lesson | What you should be able to do |
| --- | --- | --- |
| [ ] | [01 · Inference](l01_inference.rs) | Separate explicit function contracts from inferred local types; resolve ambiguous numeric methods. |
| [ ] | [02 · Integers](l02_integers.rs) | Read bases and suffixes, reason about ranges, and recognize deliberate truncating casts. |
| [ ] | [03 · Overflow](l03_overflow.rs) | Choose wrapping, strict, checked, saturating, or overflowing arithmetic; handle shift distances. |
| [ ] | [04 · Floating point](l04_floats.rs) | Use explicit numeric conversions, tolerance, NaN classification, and signed zero. |
| [ ] | [05 · Booleans, tuples, unit](l05_bool_tuples.rs) | Write explicit conditions, destructure multiple results, and recognize `()`. |
| [ ] | [06 · Pointer types](l06_pointers.rs) | Distinguish borrowing, heap ownership, shared ownership, optional references, and raw pointers. |
| [ ] | [07 · Arrays](l07_arrays.rs) | Use fixed lengths, repeated initialization, nested storage, and checked indexing. |
| [ ] | [08 · Vectors](l08_vectors.rs) | Build, edit, grow, and pop vectors; distinguish length from capacity. |
| [ ] | [09 · Slices](l09_slices.rs) | Accept arrays and vectors through one API and work on borrowed regions of their data. |
| [ ] | [10 · Unicode and memory](l10_unicode.rs) | Separate UTF-8 bytes, Unicode scalar values, and valid string slicing boundaries. |
| [ ] | [11 · Strings and literals](l11_strings.rs) | Build owned text, borrow input, use raw literals, and remove a whole Unicode scalar. |
| [ ] | [12 · Boundary types and aliases](l12_boundary_types.rs) | Choose text, bytes, paths, OS strings, or C strings at system boundaries; use an alias. |
| [ ] | [13 · Capstone](l13_capstone.rs) | Combine these decisions into a small sensor-report API. |
| [ ] | [C01 · Code review: a teammate's pull request](c01_code_review.rs) | Find overflow, Unicode, and copying bugs in working-looking code; fix them and clear clippy. |
| [ ] | [Compiler drills](../../drills/ch03/) — `cargo drill ch03` | Predict which programs compile, panic, or run, then fix them. |

```sh
cargo test --test chapter03 l01_
cargo test --test chapter03 -- --list
cargo test --test chapter03
cargo test --release --test chapter03
cargo clippy --test chapter03
cargo drill ch03
cargo run --example ch03_scratch
```

The release run matters: explicit overflow methods should keep the same behavior across build profiles.
The exercises use the chapter's `strict_mul` method; use the repository's pinned Rust 1.99.0 toolchain (see [setup instructions](../../README.md#fresh-clone-setup)).
No extra crates or network access are needed.

## Compiler labs

Run the scratch program with:

```sh
cargo run --example ch03_scratch
```

Some labs are supposed to fail compilation or panic. Record **which** happens and why, then restore the scratch file to a compiling state. Keep intentional failures out of the exercise suite.

## Mastery check

After the capstone, answer these without opening the chapter:

1. Which type declarations does Rust require, and where can inference fill in the rest?
2. What ranges do `u8` and `i8` represent? Why are indices `usize`? Does `as` always preserve a number's value?
3. Which overflow API fits a pixel intensity, a cyclic counter, and a size computation that must reject overflow?
4. Why are NaN, signed zero, and approximate equality worth thinking about?
5. How do a tuple, an array, and `()` differ?
6. Who cleans up data reached through `&T`, `Box<T>`, `Rc<T>`, and `Arc<T>`? What distinguishes a raw pointer?
7. What is stored in `[T; N]`, `Vec<T>`, and `&[T]`? Which of them owns the elements?
8. Why is `&[T]` often a better parameter than `&Vec<T>`?
9. What does string length count? How do a `char`, a Unicode scalar value, and a user-perceived character relate?
10. What is the relationship between `String` and `&str`? Where does string-literal text live?
11. When would `PathBuf`, `OsString`, `Vec<u8>`, or `CString` be more appropriate than `String`?
12. Does a type alias create a distinct type with its own validation rules?

In a later session, redo the capstone from its contract without looking at your implementation. Try two transfer exercises: accept `&[i16]` and explicitly design negative-value behavior; then replace the capstone's tuple with a named struct when you reach the structs chapter.
