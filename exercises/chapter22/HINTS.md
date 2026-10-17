# Chapter 22 hints

[Chapter guide](README.md) · [All hints](../../HINTS.md)

Try hint 1 before opening hint 2.

## l01 · Expansion and evaluation once

<details><summary>Hint 1</summary>

Bind the expression once inside the expansion rather than substituting it twice.

</details>

<details><summary>Hint 2</summary>

A block expression can contain let statements and return its final expression. Built-in macros expand where the exported expansion invokes them.

</details>

## l02 · Repetition and separators

<details><summary>Hint 1</summary>

Repeat insertion statements for the map. Counting syntax should transform each fragment into unit, not execute it.

</details>

<details><summary>Hint 2</summary>

An array of repeated () values has a length; stringify! consumes tokens without evaluating their expressions.

</details>

## l03 · Type and identifier fragments

<details><summary>Hint 1</summary>

Macro-generated items are normal Rust items after expansion; they still need valid implementations.

</details>

<details><summary>Hint 2</summary>

Use Self { value } and &self.value in the item expansion. The turbofish can contain the captured $target:ty.

</details>

## l04 · Recursive structured literals

<details><summary>Hint 1</summary>

Write a rule for each structural case, then delegate every nested value back to the macro.

</details>

<details><summary>Hint 2</summary>

A tt repetition handles bracketed/braced/grouped values. The final expr rule delegates leaf conversion to From rather than reimplementing type selection.

</details>

## l05 · Traits handle macro leaf types

<details><summary>Hint 1</summary>

A declarative macro operates on syntax; ordinary trait resolution knows the types.

</details>

<details><summary>Hint 2</summary>

Use a repeated call to one generic helper. The bool Field implementation intentionally differs from Display to expose accidental formatting shortcuts.

</details>

## l06 · Exports and hygiene

<details><summary>Hint 1</summary>

The helper must be reachable through a crate-relative path after expansion in the caller.

</details>

<details><summary>Hint 2</summary>

Use $crate::l06_exports::format_event, because each chapter is its own integration-test crate and l06_exports is the defining module.

</details>

## l07 · Capstone: structured events

<details><summary>Hint 1</summary>

Separate parsing from constructing the ordinary Event type. Identifier fragments become names via stringify!.

</details>

<details><summary>Hint 2</summary>

Bind kind before repeating fields. A BTreeMap provides deterministic comparison and last-insert-wins behavior.

</details>

