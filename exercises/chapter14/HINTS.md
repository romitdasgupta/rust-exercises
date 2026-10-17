# Chapter 14 hints

[Chapter guide](README.md) · [All hints](../../HINTS.md)

Try the chapter reference and compiler diagnostics first. Open one level at a time.

## 01 · Borrowed captures and sorting

<details><summary>Hint 1</summary>

A closure may refer to surrounding variables without an explicit parameter for each one.

</details>
<details><summary>Hint 2</summary>

Filter using starts_with and map to owned strings. Sort by (byte length, lexical content) for a deterministic tie-break.

</details>

## 02 · Mutation, move, and consuming captures

<details><summary>Hint 1</summary>

The body’s use of captured data determines the call trait. The move keyword chooses how data enters the closure.

</details>
<details><summary>Hint 2</summary>

Use a mutable local accumulator for the repeated closure; a move closure can retain owned text and still be callable repeatedly if it only reads it.

</details>

## 03 · Choosing callable bounds

<details><summary>Hint 1</summary>

Use the least restrictive call trait needed by how often and how the function invokes its argument.

</details>
<details><summary>Hint 2</summary>

FnOnce can be consumed by one call; FnMut needs a mutable binding; Fn supports repeated shared calls, including function pointers.

</details>

## 04 · Returning borrowed and erased closures

<details><summary>Hint 1</summary>

An opaque return hides one concrete closure type. Boxing erases distinct closure types behind one trait object.

</details>
<details><summary>Hint 2</summary>

Borrow prefix in a move closure with an explicit lifetime. Box each branch separately in choose.

</details>

## 05 · Copy and Clone for captures

<details><summary>Hint 1</summary>

A closure is an anonymous struct-like value with fields for its captures.

</details>
<details><summary>Hint 2</summary>

Copy a closure with a moved scalar capture. Clone a closure with a moved Vec capture before either call; each should mutate its own vector.

</details>

## 06 · Callback registration and invocation

<details><summary>Hint 1</summary>

A callback registry owns callable values and calls them in a documented order.

</details>
<details><summary>Hint 2</summary>

Push the boxed callback. Iterate mutably through callbacks; append each result in registration order.

</details>

## 07 · Capstone: configurable text processing

<details><summary>Hint 1</summary>

Keep predicate, transformation, and audit responsibilities separate. Stateful callbacks must see a documented invocation order.

</details>
<details><summary>Hint 2</summary>

For each row call keep exactly once. Accepted rows are formatted, audited, then pushed. Audit only sees formatted accepted rows.

</details>

