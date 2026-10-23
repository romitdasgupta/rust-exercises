# Chapter 11 hints

[Chapter guide](README.md) · [All hints](../../HINTS.md)

Try the chapter reference and compiler diagnostics first. Open one level at a time.

## 01 · Traits, implementations, and defaults

<details><summary>Hint 1</summary>

A default trait body can call a required method on self. Implementations use exactly the declared receiver and result types.

</details>
<details><summary>Hint 2</summary>

Build the sensor description with format!, and let brief call description rather than duplicating sensor formatting.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

`Sensor` supplies only the required method, `description`; `brief` comes from the trait's default body, which calls `self.description()` and so picks up whatever each implementor provides. A type that overrides `brief` replaces the default entirely, and generic code calling `value.brief()` runs the override. A trait is a contract about behavior, not about layout, so two unrelated types can implement it without sharing fields or a common ancestor; implementing it adds no data to either type (P2). Blanket impls such as `impl<T: Display> Exclaim for T` give the trait to every type that meets the bound.

</details>

## 02 · Static and dynamic dispatch

<details><summary>Hint 1</summary>

A reference to dyn Trait erases the concrete implementor while retaining a callable interface.

</details>
<details><summary>Hint 2</summary>

Iterate over the trait objects and call measure. Use Write::write_all twice with ? so an I/O failure is propagated.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

`total_static<T: Measure>` is monomorphized: the compiler generates one copy per concrete `T` and can inline `measure`, but the slice must hold a single type. `&dyn Measure` is a fat pointer (data pointer plus vtable pointer, two words), so one slice can mix types, and every `measure` call goes through the vtable. Default methods have vtable entries too, so an override is found at run time. Generic parameters are implicitly `Sized`; `W: Write + ?Sized` lifts that, so `write_record` accepts `&mut dyn Write` as well as concrete writers. `write_all` loops until every byte is accepted, while a single `write` may stop early.

</details>

## 03 · Self, supertraits, and associated items

<details><summary>Hint 1</summary>

Self denotes the implementing type. A supertrait makes another trait’s methods available under one bound.

</details>
<details><summary>Hint 2</summary>

Construct Count directly, use the associated LIMIT, and invoke T::zero() without an instance.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

`Resettable: Named` means every implementor must also implement `Named`, so code bounded by `T: Resettable` can call `name()` too. `fn zero() -> Self` returns `Self` by value, which needs a known size, hence `Sized`; that bound also makes the trait unusable as `dyn Resettable`. An associated function such as `zero` has no `self` and is called as `T::zero()`. A method takes `self`, `&self`, or `&mut self` and can be called with dot syntax. Associated constants belong to the implementation, not to each value: `Count` is still 2 bytes (P1), and a default method sees each implementor's own `Self::LIMIT`.

</details>

## 04 · Extension traits and qualified calls

<details><summary>Hint 1</summary>

An extension trait belongs to your crate, so its implementation may target an external type.

</details>
<details><summary>Hint 2</summary>

Use a wide signed accumulator. Fully qualify both Tone methods; ordinary value.tone() is ambiguous.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

Coherence requires at most one implementation of a trait for a type in the whole program. The orphan rule enforces it locally: you may implement a trait for a type only if your crate defines the trait or the type. So `impl SliceStats for [i32]` is fine (your trait), but `impl Display for Vec<i32>` is rejected (both foreign); wrap the type in a newtype instead. Trait methods are callable only when the trait is in scope. If two traits in scope provide the same method name for one type, `value.tone()` is ambiguous and you name the trait: `<Palette as Warm>::tone(&value)` or `Warm::tone(&value)`. Inherent methods always win over trait methods (P1).

</details>

## 05 · Associated types and generic traits

<details><summary>Hint 1</summary>

The associated Item is chosen once per implementation. The generic Rhs is part of which trait is implemented.

</details>
<details><summary>Hint 2</summary>

Use Iterator::next on the stored iterator; implement the two Combine specializations independently.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

An associated type is an output: the implementation fixes it once, so each type has exactly one `Item` per trait and callers never have to name it (`Iterator`, `Deref::Target`). A generic trait parameter is an input chosen by the caller, so one type can implement `Combine<u8>` and `Combine<&str>` side by side, and the argument type selects the implementation. Use an associated type when the relationship is determined by the implementing type; use a parameter when several relationships make sense. `Vec<u8>` and `&Vec<u8>` are different implementing types, so they can have different `Item`s (P1). Implementing the one required method gives every provided method that is built on it (P2).

</details>

## 06 · Opaque results, lifetimes, and precise capture

<details><summary>Hint 1</summary>

An opaque result hides a single concrete type. Its lifetime contract still describes references yielded by that type.

</details>
<details><summary>Hint 2</summary>

Use split_whitespace for borrowed words, move for the returned closure, and value.clone() for the first element of the duplicate pair.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

`impl Trait` in return position hides one concrete type chosen by the function body. In edition 2024 the hidden type may capture every generic and lifetime parameter in scope. `use<'a>` narrows that list, so the iterator from `words` does not borrow `_scratch`, which can then be dropped first. `+ 'a` is a different statement: the hidden type outlives `'a`, so the closure from `starts_with` may hold `&'a str`. `T: Clone` is the only bound `duplicated` needs, because it clones once and moves the original. A returned closure is a struct of its captures: a `&str` capture makes it two words, and calling it allocates nothing.

</details>

## 07 · Capstone: a generic validation pipeline

<details><summary>Hint 1</summary>

Collecting a Source does not require knowing its concrete type, only its associated Item.

</details>
<details><summary>Hint 2</summary>

Pull owned strings with a while let loop; all on an empty rule list is true. Keep accepted records in input order.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

`Rule` methods take `&self` and `&str` and return `bool`, with no generics and no `Self` by value, so the trait is dyn compatible and one slice can mix `MinLength` and `Forbid`. A source's `Item` is fixed by its implementation, and the bound `S: Source<Item = String>` says which sources `run` accepts; `run` is monomorphized per source type. A generic method such as `fn accepts_any<T: AsRef<str>>(&self, t: T)` cannot have a vtable entry, because there would be one instance per `T`. It would break `&dyn Rule` unless it is marked `where Self: Sized`, which leaves it uncallable through the trait object. Strings move from source to output without being cloned.

</details>

## R01 · Spaced review of Chapters 7, 9, and 10

<details><summary>Hint 1</summary>

A recursive enum is processed by a `match` whose `Crate` arm calls `weight` on each child. `PartialOrd` gives `<` and `>`, and `Copy` lets you keep the current low and high by value. `?` converts any `E: Error` into `Box<dyn Error>`.

</details>
<details><summary>Hint 2</summary>

`500 + items.iter().map(Weight::weight).sum::<u64>()`. For `bounds`, start with `let first = *items.first()?;`. For `parse_ids`, `let n: u16 = item.trim().parse()?;`, then `if n == 0 { return Err("zero is not a valid id".into()); }`.

</details>

## C01 · Code review

<details><summary>Hint 1</summary>

There are two bugs: one includes a boundary value it should exclude, and one lets a later tie replace the earliest. Every `Box`, `to_string`, and `clone` here can go, and two bounds are unnecessary.

</details>
<details><summary>Hint 2</summary>

Return `impl Iterator<Item = &'a str> + 'a` from `above`, taking `&'a [Box<dyn Metric>]`. `largest<T: PartialOrd>(items: &[T]) -> Option<&T>`, replacing only on `>`. In `render`, drop the `Clone` bound, iterate by reference, and `writeln!` into one `String` (import `std::fmt::Write`).

</details>
<details><summary>Check your review</summary>

`above` uses `>=` where the contract says strictly above, and it boxes an iterator that could be returned as `impl Iterator`. It also allocates a `String` per name where `&str` borrowed from the metrics would do, and takes `&Vec` where `&[_]` is enough (clippy: `ptr_arg`). `largest` uses `>=`, so the last of equal items wins. It clones on every improvement, demands `Clone` and `Display` bounds it does not need, and indexes where it could iterate (clippy: `needless_range_loop`). `render` clones every metric with `to_vec`, forces `M: Clone` (which rules out `Box<dyn Metric>` callers), and rebuilds the string for each line with `out = out + &format!(...)`.

</details>

## Drills

Run `cargo drill ch11`. The compiler output explains *what* happened; these notes explain *why*. Read one only after the drill's verdict.

<details><summary>d01 · Trait objects need dyn-compatible traits</summary>E0038. `scaled` returns `Self`, whose size is unknown behind `dyn Shape`, so no vtable entry can exist for it. Add `where Self: Sized` to `scaled`: concrete types can still call it, and the trait becomes dyn compatible.</details>
<details><summary>d02 · Trait methods need the trait in scope</summary>E0599. `write_all` is a method of `std::io::Write`; trait methods resolve only when the trait is in scope. Add `use std::io::Write;`.</details>
<details><summary>d03 · The orphan rule</summary>E0117. Neither `Display` nor `Vec` is defined in this crate, so the impl could conflict with one in another crate. Wrap it: `struct Ids(Vec<u32>);` and implement `Display` for `Ids`.</details>
<details><summary>d04 · Generic code only gets what its bounds promise</summary>E0369. Inside a generic function, `T` supports only what its bounds declare, and `>` comes from `PartialOrd`. Declare `T: PartialOrd`.</details>
<details><summary>d05 · impl Trait is one concrete type</summary>E0308. `Rev<RangeInclusive<u32>>` and `RangeInclusive<u32>` are different types, and `impl Iterator` stands for exactly one. Return `Box<dyn Iterator<Item = u32>>` (one allocation, dynamic dispatch), or restructure so both branches have one type.</details>
<details><summary>d06 · Extension traits on foreign types</summary>Compiles. Your crate defines `Halves`, so it may implement it for the foreign type `str`. The orphan rule only forbids a foreign trait on a foreign type.</details>
<details><summary>d07 · Associated types are part of the impl</summary>E0046: the impl is missing `type Item`. The signature of `next` does not declare it; add `type Item = u32;`.</details>
<details><summary>d08 · Sorting floats</summary>E0277: `sort` needs `Ord`, and `f64` is only `PartialOrd` because NaN is unordered. Use `readings.sort_by(f64::total_cmp)`, which defines a total order over all floats.</details>
<details><summary>d09 · impl Trait captures every lifetime in scope</summary>E0597. In edition 2024 the opaque type captures `separator`'s lifetime too, so `iter` keeps `separator` borrowed after its block ends. Add `+ use<'a>` to the return type so only `text` is captured.</details>
<details><summary>d10 · Downcasting a trait object</summary>Panics. Nothing constrains the literal, so `5` is an `i32`. `downcast::<i64>` checks the exact `TypeId` at run time and returns `Err`. Write `Box::new(5_i64)`, or downcast to `i32`.</details>
