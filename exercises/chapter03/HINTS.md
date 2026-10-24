# Chapter 3 hints

[Chapter guide](README.md) · [All hints](../../HINTS.md)

Open only the lesson you need. Try the first hint before revealing the second.

## 01 · Inference

<details><summary>Hint 1</summary>

The returned `Vec<u16>` constrains what goes into the vector. In the numeric method task, method lookup needs a concrete integer type before it can select `abs`.

</details>
<details><summary>Hint 2</summary>

Use `Vec::new()`, a `0..3` loop, and `push`. For the second task, suffix the negative literal with `_i32` and put parentheses around the entire negative value before calling the method.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

Function signatures (parameter and return types) must always be written; inside a body, the compiler infers local types from how values are used. The return type `Vec<u16>` tells inference that the vector returned at the end is a `Vec<u16>`, so `Vec::new()` and the pushed values become u16, even though no local has an annotation. Without any constraint, an integer literal defaults to i32 and a float literal to f64. A method call needs a concrete receiver type at that point, so `(-17).abs()` is ambiguous until a suffix or `i32::abs` decides it. A method call binds tighter than unary minus, so `-17_i32.abs()` is `-(17_i32.abs())`.

</details>

## 02 · Integers

<details><summary>Hint 1</summary>

Use the prefixes `0x`, `0o`, and `0b`. A cast to a narrower integer keeps the low bits. A same-width unsigned-to-signed cast interprets those bits as a signed value.

</details>
<details><summary>Hint 2</summary>

Use `as u8` for the deliberately truncated low byte, `as i8` for the signed view, and the integer method `count_ones` for the number of set bits. Its return type is `u32`.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

A numeric value is what the bits mean; the bit pattern is the bits themselves. `255_u8` and `-1_i8` share the pattern `1111_1111`. `as` between integers never fails: narrowing keeps the low bits (dropping the rest), same-width signed/unsigned casts reinterpret the bits, and widening a signed value sign-extends it, so `-1_i8 as u32` is `u32::MAX`. Information is lost whenever the value does not fit the target. In task B, discarding the high bits *is* the specification ("the low byte"), so truncation is correct there; when you need the same number or an error, use `u8::try_from` instead.

</details>

## 03 · Overflow

<details><summary>Hint 1</summary>

Each requested behavior has a named method on integers. Ordinary `+` is profile dependent, so it cannot stand in for these explicit policies.

</details>
<details><summary>Hint 2</summary>

Look up `wrapping_add`, `checked_add`, `saturating_add`, `overflowing_add`, `strict_mul`, `checked_shl`, and `wrapping_shl`. For midpoint, widen both operands **before** adding, or use the integer `midpoint` method. For shifts on `u8`, a distance of 8 is already out of range, even when the value is zero.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

Brightness should saturate (clamp at the brightest value), a cyclic counter should wrap, and a buffer size should use checked arithmetic and reject the request when it returns `None`. A checked shift checks the *distance*: `checked_shl` returns `None` only when the distance is at least the bit width, and bits shifted out of the top are silently lost. Plain `+` panics on overflow in debug builds and wraps in release builds, so it is never a policy. Casting after the addition is too late, because the overflow already happened in the narrow type; widen the operands first.

</details>

## 04 · Floating point

<details><summary>Hint 1</summary>

Classify a NaN with a method, not equality. IEEE floating-point division by zero produces infinities here. Preserve the sign of zero in the reciprocal exercise.

</details>
<details><summary>Hint 2</summary>

Use `is_finite`, `is_infinite`, and `is_nan`; `std::f64::consts::PI`; and explicit casts on both inputs of the scaling expression. Calculate the reciprocals from `0.0_f64` and `-0.0_f64`.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

Most decimal fractions (0.1, 0.2) have no exact binary representation, so each operation rounds and results differ from the decimal ideal in the last bits; tests compare with a tolerance instead of `==`. NaN is not equal to anything, including itself, so classify it with `is_nan`; `0.0 == -0.0` is true even though their reciprocals are opposite infinities. Converting an f32 to f64 is exact, but it cannot restore digits that were rounded away when the f32 was created, which is why task D converts *before* multiplying. f32 has a 24-bit significand, so integers above 2^24 can no longer all be represented.

</details>

## 05 · Booleans, tuples, unit

<details><summary>Hint 1</summary>

A condition must already be `bool`. A Boolean can be cast to an integer, but an integer cannot be cast to `bool`. A pair of return values can be destructured with `let (left, right) = ...`.

</details>
<details><summary>Hint 2</summary>

Use a comparison with zero, a Boolean cast, and `split_at`. The validation function can use an assertion and no explicit return value; it then returns `()` when the assertion succeeds.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

A tuple's type lists each field's type, so mixed types are fine: the compiler knows the type at every position. `t.0` names a field at compile time; `t[i]` would need a run-time index, and the compiler could not know which type the result has. Conditions must be `bool`; Rust never treats integers as truthy. The unit type `()` has exactly one value, also written `()`; functions and blocks without a final expression produce it.

</details>

## 06 · Pointer types

<details><summary>Hint 1</summary>

Dereference a mutable reference to change its referent. Two separately allocated values with equal contents do not demonstrate shared ownership of one allocation.

</details>
<details><summary>Hint 2</summary>

Use `Box::new`, then `Rc::new` plus `Rc::clone`, and `Arc::new` plus `Arc::clone`. The clone here is of the smart pointer. For the optional borrow, return `Some(value)` or `None`. No exercise requires `unsafe`.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

Tuples and arrays store their fields inline in one block of memory; Rust allocates on the heap only when you ask, for example with `Box`, `Vec`, or `Rc`. `&T` and `&mut T` borrow and never free anything; `Box<T>` uniquely owns a heap value and frees it when dropped; `Rc<T>` and `Arc<T>` free it when the last clone is dropped, and Arc's count is atomic so it can cross threads. References are never null, so absence is spelled `Option<&T>`, which costs no extra space because `None` uses the null bit pattern. Raw pointers may be null or dangling, and dereferencing them requires `unsafe`.

</details>

## 07 · Arrays

<details><summary>Hint 1</summary>

An array's length belongs to its type. Repeat-value syntax can initialize arrays, including arrays whose elements are themselves small arrays.

</details>
<details><summary>Hint 2</summary>

Start with `[0; 8]` for the buffer. The supplied row contains only `i16` values, so it can be repeated with `[row; 2]`. Use `get` to return an optional reference instead of panicking on an invalid index.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

The length is part of an array's type (`[u8; 8]` is not `[u8; 9]`), so the compiler must know it to lay out the value; a run-time length needs a `Vec`. Because arrays store elements inline, a nested array like `[[i16; 3]; 2]` is one contiguous block of 12 bytes inside whatever contains it, with no heap allocation. Indexing out of range panics at run time (or is rejected at compile time for a constant index), while `get` returns `None`. Where a local array actually lives is up to the compiler; only its size and layout are guaranteed.

</details>

## 08 · Vectors

<details><summary>Hint 1</summary>

Reserving storage does not create elements. Capture the initial length and capacity before pushing anything. A vector's capacity is at least its length; no particular growth factor is promised.

</details>
<details><summary>Hint 2</summary>

Use `collect`, `insert`, `remove`, `push`, `Vec::with_capacity`, `len`, `capacity`, and `pop`. In the queue task, insert first, then remove index zero. In the capacity task, fill the vector with `push`, not with another vector allocation.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

A `Vec` is a pointer, a capacity, and a length: capacity is how many elements fit in the current allocation, length is how many are initialized and usable. Reserving with `with_capacity` creates no elements, so indexing it still panics. When a push exceeds the capacity, the vector allocates a larger buffer and moves its elements; the growth factor is an implementation detail, so tests should not depend on it. `pop` removes the last element in O(1); `insert` and `remove` at index i shift every element after i.

</details>

## 09 · Slices

<details><summary>Hint 1</summary>

An array, a vector, and a subrange can all supply `&[i32]`. A subslice borrows existing elements. Be careful before subtracting from the length of an empty slice.

</details>
<details><summary>Hint 2</summary>

For lengths below two, return a zero-length subslice. Otherwise borrow the range `1..len - 1`. Sum in a loop with an `i32` accumulator. Borrow the mutable `start..end` region before calling `sort`.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

`[i32]` is a run of i32 values of some length, with no size known at compile time, so you use it through a pointer. `[i32; 4]` is a fixed-size array value, and `&[i32]` is a fat reference: a pointer plus a length. Arrays, vectors, and subranges of either all coerce to `&[i32]`, so a slice parameter accepts more callers than `&Vec<i32>` and promises not to resize. `sort` and many other methods are defined on slices, and both arrays and vectors provide them by borrowing their elements as a slice. Taking a subslice copies nothing: it is a new pointer and length into the same storage.

</details>

## 10 · Unicode and memory

<details><summary>Hint 1</summary>

String lengths and string indices count bytes. `chars` walks Unicode scalar values; it does not group combining marks into user-perceived characters.

</details>
<details><summary>Hint 2</summary>

Use `len`, `chars().count()`, `chars().next()`, `size_of_val`, and `len_utf8`. Check `is_char_boundary(mid)` before using `split_at(mid)`; it also rejects an index beyond the end.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

A `char` is one Unicode scalar value, stored in 4 bytes, but a user-perceived character can be several scalars, such as "e" plus a combining accent, and `==` compares scalars without normalizing. A `String` is a pointer, capacity, and length that owns a heap buffer of UTF-8 bytes; an `&str` is a pointer and length that borrows UTF-8 bytes from somewhere. Lengths and string indices count bytes, so a slice must start and end on char boundaries or it panics. String literals are stored in the program's read-only data and need no heap allocation at run time.

</details>

## 11 · Strings and literals

<details><summary>Hint 1</summary>

Borrow input text with `&str`, create output with `String`, and use string methods to maintain valid UTF-8. A raw literal treats backslashes literally; a hash delimiter lets it include quotes.

</details>
<details><summary>Hint 2</summary>

Use `format!` for the message; `push_str` and `push` for the tag. Trim each borrowed input, collect the trimmed references into a vector, then call `join`. `String::pop` removes a complete Unicode scalar. The literal task can use `r#"..."#` as its raw form.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

Accepting `&str` lets callers pass string literals, `String`s, and slices of either without giving up ownership; returning `String` hands the caller a new buffer it owns. The `String` owns its UTF-8 buffer; an `&str` only borrows bytes owned by something else. `let mut s = "hi"` makes the *binding* mutable, so `s` can point at different text, but the text itself is still borrowed and cannot grow; you need `String::from` or `to_string` for that. `push_str` and `push` append into the existing buffer, reallocating only when it is full.

</details>

## 12 · Boundary types and aliases

<details><summary>Hint 1</summary>

The correct type depends on the external representation. Paths and OS strings need not be valid Unicode. C strings must be terminated by a zero byte and cannot contain interior zero bytes.

</details>
<details><summary>Hint 2</summary>

Copy a `b"..."` byte literal into a vector for `PacketBytes`. Use `to_path_buf`, `as_os_str`, and a `c"..."` literal for the remaining tasks. The alias and `Vec<u8>` are interchangeable types.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

User text is `String`/`&str`; arbitrary file contents are `Vec<u8>`/`&[u8]`; filesystem paths are `PathBuf`/`&Path`; environment-variable names and other OS-provided strings are `OsString`/`&OsStr`; arguments to C functions are `CString`/`&CStr`. In each pair, the first type owns its buffer and the second borrows it. Paths and OS strings need not be valid UTF-8, and C strings end at one terminal zero byte, so they cannot contain an interior zero. A type alias is just another name for the same type: it adds no checks, and values of the alias and the original type are interchangeable.

</details>

## 13 · Capstone

<details><summary>Hint 1</summary>

Solve each total independently before assembling the report. The checked total must preserve `None` on overflow, while the saturated total clamps at 255. The sorted data must be an owned copy.

</details>
<details><summary>Hint 2</summary>

For the checked helper, start a `u16` accumulator at zero. Within the loop, `total = total.checked_add(reading as u16)?;` returns `None` immediately if addition fails; finish with `Some(total)`. The supplied `Option` return type supports `?`. For the other helper, use `saturating_add`. In the report, trim the label, copy readings with `to_vec`, sort the copy, and call your helpers. Measure the trimmed label, not the formatted message.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

`to_vec` copies the readings into a new vector before sorting, so the input slice is untouched and the output owns its own buffer. Numeric conversions are needed where a u8 reading joins a u16 total (`u16::from` or a widening cast), and the totals use explicit policies (`checked_add`, `saturating_add`) so the result does not depend on the build profile. The `Summary` alias names the tuple type; it is still the same tuple. The compiler guarantees every `&str` is valid UTF-8 and outlives the call, so trimming and counting scalars cannot produce invalid text.

</details>

## C01 · Code review

<details><summary>Hint 1</summary>

There are two bugs: one is an accumulator that is too narrow (run the tests in both profiles and compare the failures), and one assumes every character is one byte. The clone in the third function is unnecessary.

</details>
<details><summary>Hint 2</summary>

Sum into a `u64` and divide by `len() as u64`; the mean always fits back into u16. Take the first character with `chars().next()` and append `c.to_uppercase()` (an iterator of chars) with `extend`, so no String is created per word. A wrapping checksum is `bytes.iter().fold(0u8, |sum, &b| sum.wrapping_add(b))`. Accept `&[u16]` and `&[u8]`.

</details>
<details><summary>Check your review</summary>

`mean` sums into a `u16`, which overflows for two readings of 60_000: debug builds panic and release builds silently wrap to a wrong answer. It also truncates `len()` with `as u16`, which breaks for 70_000 readings. `initials` slices `&word[0..1]`, a byte range, so it panics on any word that starts with a non-ASCII letter, and it allocates a new String per word with `to_uppercase()` on a `&str`. `checksum` clones the whole vector just to read it, and spells wrapping arithmetic as a wider sum plus `% 256` instead of `wrapping_add`. Clippy also flags `&Vec<_>` parameters, `len() == 0`, the index loop, and `sum = sum + ...`.

</details>

## Drills

Run `cargo drill ch03`. The compiler output explains *what* happened; these notes explain *why*. Read one only after the drill's verdict.

<details><summary>d01 · Calling a method on an untyped literal</summary>E0689. Method lookup needs the receiver's concrete type, and `(-17)` is still "some integer" at that point; the later `17_i32` comes too late. Write `(-17_i32).abs()` or `i32::abs(-17)`.</details>
<details><summary>d02 · No implicit widening</summary>E0308. Rust never converts between numeric types implicitly, even when no information can be lost. Use `u32::from(small)` (lossless, preferred) or `small as u32`.</details>
<details><summary>d03 · Plain addition overflows</summary>Panics at run time ("attempt to add with overflow"), because drills are debug builds; a release build would silently wrap to 4. Choose the policy: `level.saturating_add(amount)`.</details>
<details><summary>d04 · Dividing a float by zero</summary>Compiles and runs. IEEE 754 defines `1.0 / 0.0` as positive infinity and `0.0 / 0.0` as NaN; only integer division by zero panics.</details>
<details><summary>d05 · An integer is not a condition</summary>E0308: an `if` condition must be `bool`, and integers are never truthy. Write the comparison you mean: `if pending != 0`.</details>
<details><summary>d06 · Indexing a tuple with a variable</summary>E0608: tuples have no run-time indexing, because each field may have a different type. Name the field: `reading.0`.</details>
<details><summary>d07 · Array lengths are part of the type</summary>E0435: `[0_u8; n]` needs `n` at compile time, and a parameter is a run-time value. Use `vec![0_u8; n]` for a run-time length, or a `const` length.</details>
<details><summary>d08 · Indexing a String</summary>E0277: `String` and `str` cannot be indexed by an integer, because a byte offset is not a character. Use `word.chars().next()` for the first scalar, or `word.as_bytes()[0]` for the first byte.</details>
<details><summary>d09 · Slicing inside a character</summary>Panics at run time: "ñ" is two bytes, so byte 1 is not a char boundary. Slice `&word[..2]`, or find the end with `word.char_indices().nth(1)`.</details>
<details><summary>d10 · Casting an integer to char</summary>E0604: only `u8` can be cast to `char`, because every u8 is a valid scalar but not every u32 is (surrogates, values above 0x10FFFF). Use `char::from_u32(code)`, which returns an `Option`.</details>
