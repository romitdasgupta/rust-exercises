# Chapter 7 hints

[Chapter guide](README.md) · [All hints](../../HINTS.md)

Try the first hint before opening the second. The implementation constraints in each task are part of the exercise.

## 01 · Panics and invariants

<details><summary>Hint 1</summary>

A panic represents a violated invariant here; use the exact requested message.

</details>
<details><summary>Hint 2</summary>

Use assert with the explicit string. The checked task should return Err, leaving panic for the invariant-only API.

</details>

## 02 · Catching Result errors and printing context

<details><summary>Hint 1</summary>

Result is a normal enum; handle both paths explicitly.

</details>
<details><summary>Hint 2</summary>

Match parse::<u16>(); format only the error message with the supplied context rather than platform-dependent parse wording.

</details>

## 03 · Propagating failures with question mark

<details><summary>Hint 1</summary>

Use ? to stop at the first failure without writing repetitive nesting.

</details>
<details><summary>Hint 2</summary>

Parse both halves with trim().parse()?; collect or a loop with ? can preserve the same ParseIntError type.

</details>

## 04 · Result methods and aliases

<details><summary>Hint 1</summary>

map transforms an Ok value; and_then chains another fallible operation.

</details>
<details><summary>Hint 2</summary>

Parse with map_err, then and_then for the nonzero check. For optional parsing, transpose converts Option<Result<T,E>> into Result<Option<T>,E>.

</details>

## 05 · Multiple error types at a boundary

<details><summary>Hint 1</summary>

A boxed Error trait object can hold different concrete errors and preserve their dynamic identity.

</details>
<details><summary>Hint 2</summary>

Read into a String with Read::read_to_string, then parse with ?. Both error types implement Error.

</details>

## 06 · Custom errors with a source chain

<details><summary>Hint 1</summary>

Keep parse errors as values inside an enum, and use a separate variant for a valid number rejected by the domain.

</details>
<details><summary>Hint 2</summary>

Use write! for each variant. source returns the inner parse error only for Parse; map_err chooses that variant.

</details>

## 07 · Impossible errors and deliberate recovery

<details><summary>Hint 1</summary>

An expect needs a local invariant you can explain; a public arbitrary input rarely supplies one.

</details>
<details><summary>Hint 2</summary>

Parse a fixed known literal for the invariant. For batch recovery, retain successful values and count rejected inputs explicitly.

</details>

## 08 · Capstone: fallible batch report

<details><summary>Hint 1</summary>

Use a typed enum for domain categories and carry the original one-based row number.

</details>
<details><summary>Hint 2</summary>

Parse each trimmed row, reject zero, and checked_add into u16. Convert each failure at its origin rather than losing the row number.

</details>
