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
<details><summary>Check your explanation (after writing your own)</summary>

Each operator is a trait method: `a + b` is `Add::add(a, b)`, `-a` is `Neg::neg(a)`, and `a += b` is `AddAssign::add_assign(&mut a, b)`. `Add` and `Neg` take their operands by value and return a new `Output`, which need not be `Self`: `Mul<i32> for Delta` shows that the right-hand side can be another type too. For `Copy` types like `Delta`, moving the operands is just copying them. For `String`, `s + "x"` consumes `s` and reuses its buffer (P1). Assignment operators are separate traits because they mutate the left operand in place through `&mut self`; they have no `Output`, and implementing `Add` does not give you `+=`.

</details>

## 02 · Equivalence and NaN

<details><summary>Hint 1</summary>

Choose the identity fields explicitly. PartialEq may return false even when both operands are the same float value.

</details>
<details><summary>Hint 2</summary>

Compare ids only for Record. Delegate floating equality to the wrapped f64 and use partial_cmp for comparisons with NaN.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

`PartialEq` must be symmetric (`a == b` implies `b == a`) and transitive (`a == b` and `b == c` imply `a == c`). `Eq` adds reflexivity: `a == a` for every value. `Record` compares only `id`, which still satisfies all three, so it can be `Eq` even though `note` is ignored. `f64` is not reflexive, because NaN != NaN, so `Reading`, which delegates to `f64`'s `==`, must not claim `Eq`; anything that relies on `Eq` (hash keys, dedup) would misbehave. `==` on floats is not bitwise either: `0.0 == -0.0` is true, though their bits differ.

</details>

## 03 · Partial and total comparisons

<details><summary>Hint 1</summary>

Compare primary keys first, then break ties with the secondary key.

</details>
<details><summary>Hint 2</summary>

Use cmp(...).then_with(...). PartialOrd should return Some(self.cmp(other)); partial float comparison may return None.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

`Ord::cmp` defines the order: priority first, then name as the tie-breaker. That tie-breaker is part of the contract, because `sort` and `BTreeMap` rely on it and callers see the resulting order. `PartialOrd` powers `<`, `>`, and friends, and for a totally ordered type it must agree with `Ord`, so `partial_cmp` returns `Some(self.cmp(other))`. Writing it separately invites the two to disagree. `cmp` returning `Equal` must coincide with `==`, which is why both fields take part in the derived `PartialEq`. Floats have only a partial order: `partial_cmp` returns `None` when either side is NaN. `total_cmp` invents a total order, in which -0.0 < 0.0.

</details>

## 04 · Index and IndexMut

<details><summary>Hint 1</summary>

Both traits should refer into the same underlying data and use exactly the same row-major index.

</details>
<details><summary>Hint 2</summary>

Check row<2 and column<3 before indexing row*3+column. Use assert! with the exact documented message.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

`Index::index(&self, idx) -> &Output` enables reads such as `grid[(r, c)]`; the expression means `*grid.index((r, c))`, so it is a place you can borrow (P3). `IndexMut::index_mut` returns `&mut Output` and is chosen when the place is assigned or mutably borrowed. Both must apply the same bounds check and mapping, so reads and writes see the same element. Each coordinate is checked separately: `(0, 3)` maps to flat index 3, which is in range but wrong. The traits return references into `self`; a reference to a temporary computed inside `index` would dangle, so an `Index` impl cannot build a new value. That is also why `Index` panics: a checked `get` returning `Option<&T>` is the non-panicking API.

</details>

## 05 · Capstone: a small bit-set API

<details><summary>Hint 1</summary>

The newtype gives a semantic meaning to bitwise operators without changing the underlying u8 representation.

</details>
<details><summary>Hint 2</summary>

Delegate to the corresponding u8 operators. Check bit<8 before shifting so debug and release agree.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

`Flags` derives `PartialEq`, `Eq`, and `Hash` from the same single field, so equal values always hash equally, which is the contract `HashSet` and `HashMap` depend on. Union, intersection, and complement map directly onto `|`, `&`, and `!` of the `u8`. Complement needs no length because the width is fixed at 8 bits. `|=` mutates in place through `BitOrAssign`. `contains(bit)` must check `bit < 8` before shifting: `1u8 << 8` overflows, which panics in debug builds and wraps in release, so the check keeps both profiles in agreement. Indexing would have to return `&bool` into storage that does not exist, so a method is the honest API.

</details>

## R01 · Spaced review of Chapters 5, 10, and 11

<details><summary>Hint 1</summary>

The bounds on `dot` already list everything you may do with `T`: copy it, start from `T::default()`, add, and multiply. Compare `Money` by converting both sides to cents in a type wider than `u32`. `Index` returns a reference, so `get` must return a reference into `self.rows`.

</details>
<details><summary>Hint 2</summary>

`a.iter().zip(b).fold(T::default(), |acc, (&x, &y)| acc + x * y)`. A helper `fn cents(m: Money) -> u64` with a `match` makes `eq` one line. `self.rows.iter().find(|(n, _)| n == name).map(|(_, v)| v)`. In `index`, `self.get(name).unwrap_or_else(|| panic!("no row named {name}"))`.

</details>

## C01 · Code review

<details><summary>Hint 1</summary>

There are two bugs: one breaks the rule that equal values must hash equally, and one silently drops coefficients. Every `clone`, every `to_ascii_lowercase`, and the fresh `Vec` in `add` can go.

</details>
<details><summary>Hint 2</summary>

`str::eq_ignore_ascii_case` compares without allocating. In `hash`, feed `b.to_ascii_lowercase()` for each byte to `state.write_u8`, then a terminator such as `0xff`. Put the addition in `add_assign`: add the overlapping part in place, then `extend_from_slice` the rest of `rhs`. `add` can then pick the longer operand as its accumulator and `+=` the other into it.

</details>
<details><summary>Check your review</summary>

`Tag` overrides equality to ignore ASCII case but hashes the original bytes, so `"Rust"` and `"rust"` are equal yet land in different hash buckets: a `HashSet<Tag>` can hold both. Equality also allocates two lowercase copies per comparison. `Poly::add` zips the coefficients, which stops at the shorter operand and loses the longer one's tail, and it allocates a new vector even though it owns both inputs. `add_assign` clones both sides just to call `add`, so `+=` copies everything instead of mutating in place. The lesson: when you implement `PartialEq` by hand, implement `Hash` from the same normalized identity.

</details>

## Drills

Run `cargo drill ch12`. The compiler output explains *what* happened; these notes explain *why*. Read one only after the drill's verdict.

<details><summary>d01 · Operators take their operands by value</summary>E0382. `Add::add(self, rhs)` takes `wallet` by value, so `wallet + ...` moves it. Derive `Clone` and write `wallet.clone() + ...`, or also implement `Add` for `&Money` so you can write `&wallet + ...`.</details>
<details><summary>d02 · Add needs an Output</summary>E0046: `type Output` is missing. The result type of `+` is an associated type the impl must choose, here `type Output = Meters;`.</details>
<details><summary>d03 · Eq for a struct holding a float</summary>E0277: `Eq` can be derived only when every field is `Eq`, and `f64` is not, because NaN != NaN. Derive only `PartialEq`.</details>
<details><summary>d04 · Unary minus on an unsigned integer</summary>E0600. Unsigned integers do not implement `Neg`: there is no negative `u32`. Use a signed type (`let speed: i32 = 30;`), or express the intent with subtraction.</details>
<details><summary>d05 · Operand order matters</summary>E0277. `3 * v` calls `Mul<Vec2>` on `i32`, which does not exist; `Mul<i32> for Vec2` only covers `v * 3`. Add `impl Mul<Vec2> for i32` (allowed because `Vec2` is local) that forwards to `v * self`.</details>
<details><summary>d06 · String + &String</summary>Compiles. `Add<&str> for String` needs `&str` on the right, and `&host` (a `&String`) deref-coerces to `&str`. The left `String` is consumed and its buffer reused; `host` is only borrowed.</details>
<details><summary>d07 · &str + &str</summary>E0369. Neither operand owns a buffer that could grow, so std defines no `Add` for `&str`. Start from an owned `String`: `first.to_owned() + second`, or use `format!` or `concat`.</details>
<details><summary>d08 · PartialOrd builds on PartialEq</summary>E0277: `PartialOrd<Rhs>` has `PartialEq<Rhs>` as a supertrait, because an ordering must agree with equality. Add `#[derive(PartialEq)]`, or better, derive `PartialEq, Eq, PartialOrd, Ord` together.</details>
<details><summary>d09 · Indexing a map with a missing key</summary>Panics. `Index` must return a reference, and there is nothing to point at, so `HashMap`'s `Index` impl panics. Use `ports.get("ssh")`, which returns `Option<&V>`, and handle `None`.</details>
<details><summary>d10 · Compound assignment is its own trait</summary>E0368. `+=` is `AddAssign`, not `Add`. Implement `AddAssign` (`self.0 += rhs.0`), or write `total = total + points`, which works because `Score` is `Copy`.</details>
