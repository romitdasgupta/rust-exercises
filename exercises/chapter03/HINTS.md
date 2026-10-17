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

## 02 · Integers

<details><summary>Hint 1</summary>

Use the prefixes `0x`, `0o`, and `0b`. A cast to a narrower integer keeps the low bits. A same-width unsigned-to-signed cast interprets those bits as a signed value.

</details>
<details><summary>Hint 2</summary>

Use `as u8` for the deliberately truncated low byte, `as i8` for the signed view, and the integer method `count_ones` for the number of set bits. Its return type is `u32`.

</details>

## 03 · Overflow

<details><summary>Hint 1</summary>

Each requested behavior has a named method on integers. Ordinary `+` is profile dependent, so it cannot stand in for these explicit policies.

</details>
<details><summary>Hint 2</summary>

Look up `wrapping_add`, `checked_add`, `saturating_add`, `overflowing_add`, `strict_mul`, `checked_shl`, and `wrapping_shl`. For midpoint, widen both operands **before** adding, or use the integer `midpoint` method. For shifts on `u8`, a distance of 8 is already out of range, even when the value is zero.

</details>

## 04 · Floating point

<details><summary>Hint 1</summary>

Classify a NaN with a method, not equality. IEEE floating-point division by zero produces infinities here. Preserve the sign of zero in the reciprocal exercise.

</details>
<details><summary>Hint 2</summary>

Use `is_finite`, `is_infinite`, and `is_nan`; `std::f64::consts::PI`; and explicit casts on both inputs of the scaling expression. Calculate the reciprocals from `0.0_f64` and `-0.0_f64`.

</details>

## 05 · Booleans, tuples, unit

<details><summary>Hint 1</summary>

A condition must already be `bool`. A Boolean can be cast to an integer, but an integer cannot be cast to `bool`. A pair of return values can be destructured with `let (left, right) = ...`.

</details>
<details><summary>Hint 2</summary>

Use a comparison with zero, a Boolean cast, and `split_at`. The validation function can use an assertion and no explicit return value; it then returns `()` when the assertion succeeds.

</details>

## 06 · Pointer types

<details><summary>Hint 1</summary>

Dereference a mutable reference to change its referent. Two separately allocated values with equal contents do not demonstrate shared ownership of one allocation.

</details>
<details><summary>Hint 2</summary>

Use `Box::new`, then `Rc::new` plus `Rc::clone`, and `Arc::new` plus `Arc::clone`. The clone here is of the smart pointer. For the optional borrow, return `Some(value)` or `None`. No exercise requires `unsafe`.

</details>

## 07 · Arrays

<details><summary>Hint 1</summary>

An array's length belongs to its type. Repeat-value syntax can initialize arrays, including arrays whose elements are themselves small arrays.

</details>
<details><summary>Hint 2</summary>

Start with `[0; 8]` for the buffer. The supplied row contains only `i16` values, so it can be repeated with `[row; 2]`. Use `get` to return an optional reference instead of panicking on an invalid index.

</details>

## 08 · Vectors

<details><summary>Hint 1</summary>

Reserving storage does not create elements. Capture the initial length and capacity before pushing anything. A vector's capacity is at least its length; no particular growth factor is promised.

</details>
<details><summary>Hint 2</summary>

Use `collect`, `insert`, `remove`, `push`, `Vec::with_capacity`, `len`, `capacity`, and `pop`. In the queue task, insert first, then remove index zero. In the capacity task, fill the vector with `push`, not with another vector allocation.

</details>

## 09 · Slices

<details><summary>Hint 1</summary>

An array, a vector, and a subrange can all supply `&[i32]`. A subslice borrows existing elements. Be careful before subtracting from the length of an empty slice.

</details>
<details><summary>Hint 2</summary>

For lengths below two, return a zero-length subslice. Otherwise borrow the range `1..len - 1`. Sum in a loop with an `i32` accumulator. Borrow the mutable `start..end` region before calling `sort`.

</details>

## 10 · Unicode and memory

<details><summary>Hint 1</summary>

String lengths and string indices count bytes. `chars` walks Unicode scalar values; it does not group combining marks into user-perceived characters.

</details>
<details><summary>Hint 2</summary>

Use `len`, `chars().count()`, `chars().next()`, `size_of_val`, and `len_utf8`. Check `is_char_boundary(mid)` before using `split_at(mid)`; it also rejects an index beyond the end.

</details>

## 11 · Strings and literals

<details><summary>Hint 1</summary>

Borrow input text with `&str`, create output with `String`, and use string methods to maintain valid UTF-8. A raw literal treats backslashes literally; a hash delimiter lets it include quotes.

</details>
<details><summary>Hint 2</summary>

Use `format!` for the message; `push_str` and `push` for the tag. Trim each borrowed input, collect the trimmed references into a vector, then call `join`. `String::pop` removes a complete Unicode scalar. The literal task can use `r#"..."#` as its raw form.

</details>

## 12 · Boundary types and aliases

<details><summary>Hint 1</summary>

The correct type depends on the external representation. Paths and OS strings need not be valid Unicode. C strings must be terminated by a zero byte and cannot contain interior zero bytes.

</details>
<details><summary>Hint 2</summary>

Copy a `b"..."` byte literal into a vector for `PacketBytes`. Use `to_path_buf`, `as_os_str`, and a `c"..."` literal for the remaining tasks. The alias and `Vec<u8>` are interchangeable types.

</details>

## 13 · Capstone

<details><summary>Hint 1</summary>

Solve each total independently before assembling the report. The checked total must preserve `None` on overflow, while the saturated total clamps at 255. The sorted data must be an owned copy.

</details>
<details><summary>Hint 2</summary>

For the checked helper, start a `u16` accumulator at zero. Within the loop, `total = total.checked_add(reading as u16)?;` returns `None` immediately if addition fails; finish with `Some(total)`. The supplied `Option` return type supports `?`. For the other helper, use `saturating_add`. In the report, trim the label, copy readings with `to_vec`, sort the copy, and call your helpers. Measure the trimmed label, not the formatted message.

</details>
