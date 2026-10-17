# Chapter 12 hints

[Chapter guide](README.md) · [All hints](../../HINTS.md)

Try the chapter reference and compiler diagnostics first. Open one level at a time.

## 01 · Unary, binary, and assignment operators

<details><summary>Hint 1</summary>

Each operator maps to a method in std::ops. Compound assignment receives &mut self.

</details>
<details><summary>Hint 2</summary>

Add the matching coordinates, negate both coordinates, and update the receiver in AddAssign. Inputs in this lesson stay in -1000..=1000.

</details>

## 02 · Equivalence and NaN

<details><summary>Hint 1</summary>

Choose the identity fields explicitly. PartialEq may return false even when both operands are the same float value.

</details>
<details><summary>Hint 2</summary>

Compare ids only for Record. Delegate floating equality to the wrapped f64 and use partial_cmp for comparisons with NaN.

</details>

## 03 · Partial and total comparisons

<details><summary>Hint 1</summary>

Compare primary keys first, then break ties with the secondary key.

</details>
<details><summary>Hint 2</summary>

Use cmp(...).then_with(...). PartialOrd should return Some(self.cmp(other)); partial float comparison may return None.

</details>

## 04 · Index and IndexMut

<details><summary>Hint 1</summary>

Both traits should refer into the same underlying data and use exactly the same row-major index.

</details>
<details><summary>Hint 2</summary>

Check row<2 and column<3 before indexing row*3+column. Use assert! with the exact documented message.

</details>

## 05 · Capstone: a small bit-set API

<details><summary>Hint 1</summary>

The newtype gives a semantic meaning to bitwise operators without changing the underlying u8 representation.

</details>
<details><summary>Hint 2</summary>

Delegate to the corresponding u8 operators. Check bit<8 before shifting so debug and release agree.

</details>

