# Chapter 3 notes — Fundamental Types

Your workbook. Write answers in your own words, *before* checking the hints. When `cargo progress` says this chapter is due for review, answer the mastery questions at the bottom again from memory, then compare with what you wrote here.

## l01 · Inference and numeric method lookup

**Predictions I got wrong, and why:**

- 

**Explain:** Which signatures must be explicit? How does information from a return type flow back to `Vec::new()` and the loop's integers?

> 

**Edge case I added:**

- 

## l02 · Integer representations, literals, and casts

**Predictions I got wrong, and why:**

- 

**Explain:** Distinguish a numeric value from its bit pattern. When can `as` discard information? Why is truncation appropriate for task B specifically?

> 

**Edge case I added:**

- 

## l03 · Choosing an overflow policy

**Predictions I got wrong, and why:**

- 

**Explain:** Pick policies for image brightness, a cyclic counter, and a buffer size that must reject overflow. Does a checked shift detect bits shifted out, or an invalid shift distance? Why is casting AFTER addition too late?

> 

**Edge case I added:**

- 

## l04 · Floating-point values and explicit conversions

**Predictions I got wrong, and why:**

- 

**Explain:** Why do the area tests use a tolerance? Does casting an f32 to f64 recover precision already lost when that f32 was created?

> 

**Edge case I added:**

- 

## l05 · Explicit conditions, tuples, and unit

**Predictions I got wrong, and why:**

- 

**Explain:** Why can a tuple have mixed field types? Why can `t.0` work while `t[i]` doesn't? What is the single value of the unit type?

> 

**Edge case I added:**

- 

## l06 · Pointer types encode different policies

**Predictions I got wrong, and why:**

- 

**Explain:** Why are nested Rust tuples values stored together rather than automatically separate heap objects? Why use `Option<&T>` for absence? How do raw pointers differ in nullability and dereferencing requirements? Recognizing these policies is enough here; full borrowing comes later.

> 

**Edge case I added:**

- 

## l07 · Fixed-size, inline sequences

**Predictions I got wrong, and why:**

- 

**Explain:** Why must an array length be known at compile time? How can a nested array live inside another value? Compare indexing with `get` on invalid input. Avoid assuming a particular physical location after compiler optimization.

> 

**Edge case I added:**

- 

## l08 · Growable owned sequences

**Predictions I got wrong, and why:**

- 

**Explain:** Why is capacity not the same as length? Why avoid asserting a particular allocation growth factor? How do insert/remove differ from pop in the number of elements they may need to shift?

> 

**Edge case I added:**

- 

## l09 · Borrow a region instead of owning a collection

**Predictions I got wrong, and why:**

- 

**Explain:** Distinguish `[i32]`, `[i32; 4]`, and `&[i32]`. Why prefer a slice parameter over `&Vec<i32>`? Why can both arrays and vectors call `sort`?

> 

**Edge case I added:**

- 

## l10 · UTF-8 bytes, scalar values, and string slices

**Predictions I got wrong, and why:**

- 

**Explain:** A char is a Unicode scalar value. Why can one user-perceived character consist of multiple chars? Contrast String metadata, its UTF-8 buffer, and an &str view. Do string literals need a runtime heap allocation?

> 

**Edge case I added:**

- 

## l11 · Building text and writing literals

**Predictions I got wrong, and why:**

- 

**Explain:** Why accept &str when reading input, and return String for newly built text? Which value owns the UTF-8 buffer? Does `let mut s = "hi"` turn that borrowed text into a growable owned string?

> 

**Edge case I added:**

- 

## l12 · Pick the representation at a system boundary

**Predictions I got wrong, and why:**

- 

**Explain:** Choose types for user text, arbitrary file contents, filesystem paths, environment-variable names, and C API arguments. Distinguish owned and borrowed forms. Why doesn't an alias enforce a new validation rule?

> 

**Edge case I added:**

- 

## l13 · Sensor report capstone

**Predictions I got wrong, and why:**

- 

**Explain:** Why is the sorted output independent of the input? Where are numeric conversions necessary? Does aliasing Summary create a new type? What does the compiler guarantee about your &str inputs? Add your own edge-case test, then explain each field without the book.

> 

**Edge case I added:**

- 

## c01 · A teammate's pull request

**My review, written before running the tests:**

- 

**Edge case I added:**

- 

## Drills

Drills whose verdict surprised me, and the rule I learned:

- 

## Mastery check (answer from memory)

**Which type declarations does Rust require, and where can inference fill in the rest?**

> 

**What ranges do `u8` and `i8` represent? Why are indices `usize`? Does `as` always preserve a number's value?**

> 

**Which overflow API fits a pixel intensity, a cyclic counter, and a size computation that must reject overflow?**

> 

**Why are NaN, signed zero, and approximate equality worth thinking about?**

> 

**How do a tuple, an array, and `()` differ?**

> 

**Who cleans up data reached through `&T`, `Box<T>`, `Rc<T>`, and `Arc<T>`? What distinguishes a raw pointer?**

> 

**What is stored in `[T; N]`, `Vec<T>`, and `&[T]`? Which of them owns the elements?**

> 

**Why is `&[T]` often a better parameter than `&Vec<T>`?**

> 

**What does string length count? How do a `char`, a Unicode scalar value, and a user-perceived character relate?**

> 

**What is the relationship between `String` and `&str`? Where does string-literal text live?**

> 

**When would `PathBuf`, `OsString`, `Vec<u8>`, or `CString` be more appropriate than `String`?**

> 

**Does a type alias create a distinct type with its own validation rules?**

>
