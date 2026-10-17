# Chapter 8 hints

[Chapter guide](README.md) · [All hints](../../HINTS.md)

Try the first hint before opening the second. The implementation constraints in each task are part of the exercise.

## 01 · Crates, editions and profiles

<details><summary>Hint 1</summary>

cfg! returns a compile-time Boolean. Package metadata is available through compile-time environment variables.

</details>
<details><summary>Hint 2</summary>

Use cfg!(debug_assertions) for the profile-sensitive task. Read env!("CARGO_PKG_NAME") and env!("CARGO_PKG_VERSION") rather than runtime environment variables.

</details>

## 02 · Nested modules and private implementation

<details><summary>Hint 1</summary>

Items are private by default. Export the API while keeping the calculation helper private.

</details>
<details><summary>Hint 2</summary>

The public cost function can invoke a nested module function marked pub(super). Use widened u32 arithmetic.

</details>

## 03 · Paths, imports and reexports

<details><summary>Hint 1</summary>

A facade can reexport a public type from a private implementation module.

</details>
<details><summary>Hint 2</summary>

Create Ticket through its inherent new constructor; name may be borrowed through an accessor while representation stays private.

</details>

## 04 · Constants and statics

<details><summary>Hint 1</summary>

Constants are values evaluated at compile time; immutable statics have one storage location.

</details>
<details><summary>Hint 2</summary>

Compute a byte count from the constant. Borrow the static array directly instead of constructing an equal temporary.

</details>

## 05 · Attributes, cfg and included files

<details><summary>Hint 1</summary>

Attributes can remove code before type checking. include_str! embeds UTF-8 content into the executable.

</details>
<details><summary>Hint 2</summary>

Use cfg! for the target family value. Split the included literal by lines and trim nonempty lines.

</details>

## 06 · Tests and executable documentation

<details><summary>Hint 1</summary>

Public APIs should make their normalization rules precise enough for examples and tests.

</details>
<details><summary>Hint 2</summary>

trim and split_whitespace cover leading/trailing and repeated Unicode whitespace. Test the public contract rather than internal steps.

</details>

## 07 · Dependency requirements and lockfiles

<details><summary>Hint 1</summary>

These tasks model only the numeric stable-version caret intervals explicitly stated, not full Cargo semver parsing.

</details>
<details><summary>Hint 2</summary>

For a nonzero major caret interval, keep the same major and compare minor/patch lexicographically. For 0.minor with nonzero minor, keep both major and minor.

</details>

## 08 · Capstone: library facade and workspace

<details><summary>Hint 1</summary>

Keep validation private, reexport the public function, and let command-line concerns call the library.

</details>
<details><summary>Hint 2</summary>

The facade accepts text, validates and formats it; a batch function propagates the first indexed failure.

</details>
