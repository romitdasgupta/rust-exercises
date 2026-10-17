# Programming Rust practice: Chapter 3 — Fundamental Types

[All chapters](../../README.md) · [Next chapter](../chapter04/README.md)

Original exercises accompanying [Chapter 3 of *Programming Rust*, 3rd edition](https://learning.oreilly.com/library/view/programming-rust-3rd/9781098176228/ch03.html).
The book is your reference; these exercises are a place to practice and explain its ideas.

## Start here

Open [lesson 1](l01_inference.rs). Read its prediction prompts **before** running anything, then replace its two `todo!()` bodies. Check your work with:

```sh
cargo test --test chapter03 l01_
```

The unfinished exercises compile, but their tests fail intentionally when they reach `todo!()`.
Filtering by lesson runs only that lesson; unfinished later lessons will not run.
Leave the tests intact. You can add cases of your own.

## How to work through a lesson

1. **Predict:** write down the answers at the top of the lesson without executing the examples.
2. **Implement:** replace its `todo!()` bodies. Read the contracts, then the tests.
3. **Check:** run that lesson's command. Treat a failure as feedback about one specific behavior.
4. **Experiment:** use [the scratch example](../../examples/ch03_scratch.rs) for the compiler lab. Repair or remove each experiment before trying the next one.
5. **Explain:** answer the lesson's explanation questions in your own words. Passing tests alone does not finish a lesson.
6. **Transfer:** invent one new edge case and predict its result before adding a test.

If stuck, start with the relevant section of the chapter. Then open [HINTS.md](HINTS.md), which has two levels of hints per lesson. There is no answer key in the exercise files.

For guided review, send me the lesson number, your predictions and explanations, and your implementation or test failure. I can review the reasoning and give the next useful hint.

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

Use the same command pattern for each lesson:

```sh
cargo test --test chapter03 l03_
cargo test --test chapter03 l10_
```

To list all checks without running them:

```sh
cargo test --test chapter03 -- --list
```

To check the complete chapter after finishing the lessons:

```sh
cargo test --test chapter03
cargo test --release --test chapter03
cargo fmt --check
```

The release run matters: explicit overflow methods should keep the same behavior across build profiles.
The exercises use the chapter's `strict_mul` method; this workspace has Rust 1.99 installed.
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
