# Chapter 5 hints

[Chapter guide](README.md) · [All hints](../../HINTS.md)

Try the first hint before opening the second. The implementation constraints in each task are part of the exercise.

## 01 · Borrowing and dereferencing

<details><summary>Hint 1</summary>

A shared borrow reads; an exclusive borrow can write its referent.

</details>
<details><summary>Hint 2</summary>

Use len on the borrowed value; use *slot on the left side of assignment.

</details>

## 02 · Reference bindings and identity

<details><summary>Hint 1</summary>

Reference equality usually compares values; pointer identity is a separate question.

</details>
<details><summary>Hint 2</summary>

Reassign a mutable &i32 binding for selection. Use std::ptr::eq for identity and == for content.

</details>

## 03 · Optional references and fat pointers

<details><summary>Hint 1</summary>

Checked access naturally represents a missing borrow as None.

</details>
<details><summary>Hint 2</summary>

Use get to return Option<&T>; return a borrowed prefix and compute length from the slice metadata.

</details>

## 04 · Lifetimes through functions

<details><summary>Hint 1</summary>

Return a subslice of input, whose owner is controlled by the caller.

</details>
<details><summary>Hint 2</summary>

split_once can return two borrows of the original str. A maximum scan can keep a borrowed candidate.

</details>

## 05 · Structs that borrow

<details><summary>Hint 1</summary>

The view stores references into one source; it does not copy text.

</details>
<details><summary>Hint 2</summary>

Use split_once and store its borrowed parts with the same lifetime. Return a field directly.

</details>

## 06 · Independent lifetimes and elision

<details><summary>Hint 1</summary>

Use separate lifetime parameters when output depends on only one independently borrowed input.

</details>
<details><summary>Hint 2</summary>

Give left and right separate lifetimes. The output of left_text needs only the left lifetime.

</details>

## 07 · Sharing versus mutation

<details><summary>Hint 1</summary>

End read access before requesting exclusive mutation.

</details>
<details><summary>Hint 2</summary>

Calculate values before the mutation, then update via &mut. Keep no element borrow alive across push.

</details>

## 08 · Disjoint mutable borrows

<details><summary>Hint 1</summary>

Use a library operation whose returned references describe nonoverlapping storage.

</details>
<details><summary>Hint 2</summary>

Check bounds and equality first, split at the larger index, and update one element from each half.

</details>

## 09 · Capstone: borrowed roster queries

<details><summary>Hint 1</summary>

A returned borrow ties up the relevant owner; an index carries no borrow but can become stale.

</details>
<details><summary>Hint 2</summary>

Scan indices and keep the first longest match. Return a subslice from the chosen String only while borrowing the roster.

</details>
