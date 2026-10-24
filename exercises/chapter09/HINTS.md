# Chapter 9 hints

[Chapter guide](README.md) · [All hints](../../HINTS.md)

Try the first hint before opening the second. The implementation constraints in each task are part of the exercise.

## 01 · Named structs and update syntax

<details><summary>Hint 1</summary>

A struct is a named collection of fields; update syntax moves fields that are not supplied.

</details>
<details><summary>Hint 2</summary>

Use shorthand where parameter names match fields. Build a replacement struct with ..old so the untouched String moves.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

A struct expression names every field, so order in the expression does not matter; field shorthand `label` stands for `label: label`. Update syntax `..old` moves (or copies, for Copy fields) every field you did not list out of `old`. Moving a String field moves only its header, so the label keeps its heap buffer and no clone happens. Afterward `old` is partially moved: the fields you supplied yourself are still owned by `old` and usable, but `old` as a whole is not. Fields are dropped in declaration order, not in the order you wrote them.

</details>

## 02 · Tuple structs, unit structs and layout

<details><summary>Hint 1</summary>

Tuple structs are distinct named types even when their representations happen to match.

</details>
<details><summary>Hint 2</summary>

Read .0 to access a tuple field and construct the unit struct by its name.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

Tuple structs are nominal: `UserId(u64)` and `OrderId(u64)` share a representation but are different types, so the compiler rejects mixing them up. A unit struct has one value and occupies zero bytes, so it can mark a proof step ("validated") at no cost; a `Vec` of zero-sized values never allocates. With the default `repr(Rust)` the compiler may reorder fields to reduce padding, so neither field order nor exact size is a promise. `repr(C)` fixes declaration order and C's padding rules: it is a contract for FFI and binary layouts, not an optimization.

</details>

## 03 · Methods and associated items

<details><summary>Hint 1</summary>

Choose receivers according to observation, mutation, or consumption.

</details>
<details><summary>Hint 2</summary>

Validate the upper bound before changing any field; access the bound through Self::MAX.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

Associated functions without `self` are called as `Gauge::new(..)`; methods take `self`, `&self`, or `&mut self` and are called with dot syntax. Pick `&self` to observe, `&mut self` to change, and `self` to consume and finish the value; a `self` method drops its receiver when it returns unless it moves it elsewhere. The constructor and `set` both enforce `reading <= Self::MAX`, and `set` checks before writing, so no code path can store an invalid reading. An associated constant belongs to the type, not to any value, and an inherent method wins over a trait method with the same name.

</details>

## 04 · Smart-pointer self receivers

<details><summary>Hint 1</summary>

The receiver consumes the smart pointer value; moving out is permitted for unique Box ownership.

</details>
<details><summary>Hint 2</summary>

Return the field from a Box receiver. For Rc or Arc return self after inspecting the owned pointer, avoiding another clone.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

A receiver like `self: Rc<Self>` takes ownership of ONE pointer, not of the value: the method holds one strong count while it runs and drops it at the end unless it returns it. `self: Box<Self>` owns the heap value uniquely, so it may move fields out of it. Returning `self` hands the same pointer back without another `Rc::clone`. Autoderef lets you call `&self` methods through any of these pointers, which is why the receiver type only matters when the method needs ownership of the pointer itself.

</details>

## 05 · Generic structs and impl blocks

<details><summary>Hint 1</summary>

Ownership operations do not require Clone or Copy.

</details>
<details><summary>Hint 2</summary>

Take the Option out of its field, and return the previous value on replace.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

`Slot<T>` puts no bounds on `T`, because owning, moving, and replacing values never needs `Clone` or `Copy`. `Option::take` and `Option::replace` (or `mem::replace`) move the old value out and leave a valid one behind. Bounds belong on the impl block or the method that needs them, such as an `impl<T: Copy>` block for a copying `peek`; putting `Copy` on the struct would reject `Slot<String>` for no reason. `let _ = expr;` drops the value at once, while `let _name = expr;` keeps it to the end of the scope.

</details>

## 06 · Lifetimes and const parameters

<details><summary>Hint 1</summary>

The window borrows an array whose length is part of its type.

</details>
<details><summary>Hint 2</summary>

Use array.get for checked access; the stored reference already carries the source lifetime.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

`T` is the element type, `'a` says the window borrows an array that must outlive it, and `N` is part of the type, so `Window<T, 3>` and `Window<T, 4>` are different types. The window owns nothing; it is one thin pointer, because the length is known at compile time, while a `&[T]` slice also carries its length at run time. `get` returns `&'a T`, a borrow of the array itself rather than of the window, so the element outlives the window. Returning a window over a local array fails because the array dies at the end of the function.

</details>

## 07 · Deriving common traits

<details><summary>Hint 1</summary>

Derived Clone delegates to each field; the cloned String owns independent bytes.

</details>
<details><summary>Hint 2</summary>

Clone the struct and mutate only its label. For an all-default record call Default::default rather than duplicating defaults.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

Derived traits work field by field: `Clone` clones every field (a new buffer for each String, a count bump for each Rc), `PartialEq` compares every field, `Ord` compares fields lexicographically in declaration order, and `Default` uses each field's default. Each derive requires every field to implement the trait, which is why a struct holding a String or Vec cannot be `Copy`. `Debug` is for developers and is derivable; `Display` is user-facing text and must be written by hand. A derived `Default` can produce a value your domain considers invalid, such as an empty id, so derive it only when the zero value really is valid.

</details>

## 08 · Cell and RefCell

<details><summary>Hint 1</summary>

Cell allows replacement through shared access; RefCell checks exclusive borrowing dynamically.

</details>
<details><summary>Hint 2</summary>

Use get/set for a saturating counter; use try_borrow_mut and return false on conflict before pushing.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

Interior mutability moves the aliasing check from compile time to run time. `Cell<T>` never hands out references; you `get`, `set`, `replace`, or `take` whole values, which is why it suits Copy counters. `RefCell<T>` tracks guards at run time: many `borrow()`s or one `borrow_mut()`. Overlapping them panics with `borrow`/`borrow_mut`, or returns `Err` with the `try_` versions. A guard lives as long as the temporary that holds it, which for a `match` scrutinee is the whole match. Neither type is `Sync`, so they cannot be shared between threads; that is what `Mutex` and atomics are for.

</details>

## 09 · Capstone: validated inventory

<details><summary>Hint 1</summary>

Separate state mutation requiring &mut self from observational metadata updated via Cell.

</details>
<details><summary>Hint 2</summary>

Validate names before construction; checked_add before changing stock; use Cell for the read counter and mem::take or an owned receiver for final extraction.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

`new` establishes the invariant (a trimmed, nonempty, owned name) so every other method can rely on it, and the private fields stop outside code from bypassing it. `restock` computes `checked_add` first and writes only on success, so a failed call changes nothing. `inspect` only needs `&self` because the read counter is a statistic, not part of the inventory's state; a `Cell<u32>` lets a shared borrow update it without `&mut`. The counter is single-threaded by design: sharing it across threads would need an `AtomicU32`.

</details>

## R01 · Spaced review of Chapters 3, 5, 7 and 8

<details><summary>Hint 1</summary>

The tests live in a sibling module, so they can only reach `Tag` through `pub` items: validate with `bytes().all(..)` and the length check, and only then call `to_owned`. For the transfer, compute both new balances with checked arithmetic before writing either.

</details>
<details><summary>Hint 2</summary>

`checked_sub` on the source gives the `Insufficient` case, `checked_add` on the target gives `WouldOverflow`; `ok_or(..)?` turns each into an early return. For `Words`, slice `&self.text[self.pos..]`, use `trim_start()` and `find(char::is_whitespace)`, and advance `pos` by byte lengths, which always land on char boundaries. Return type `&'a str` means the result borrows the text, not `self`.

</details>

## C01 · Code review

<details><summary>Hint 1</summary>

There are two bugs: one validates the input before it is cleaned up, and one makes equality depend on how often a value was *looked at*. Every `.clone()` in this file is unnecessary.

</details>
<details><summary>Hint 2</summary>

Derived `PartialEq` compares every field, including the statistic, so write `impl PartialEq` by hand for the fields that define identity. `Cell<u32>` is enough for a Copy counter. A getter can return `&str`; a builder-style method can take `self` and use `..self`. `format!` or `write!` builds the summary in one go.

</details>
<details><summary>Check your review</summary>

`new` checks `is_empty()` before trimming, so an all-whitespace id gets through; it also clones the owned `items` vector and allocates the trimmed id twice. The derived `PartialEq` includes `views`, so two equal shipments stop being equal after one is summarized; compare `id`, `items`, and `grams` only. `id()` returns a fresh String where `&str` would do, and `with_grams(&self)` clones the whole shipment; taking `self` moves everything for free. `RefCell<u32>` with two separate borrows is a heavyweight `Cell::update(|n| n.saturating_add(1))`. In `summary`, clippy flags `len() == 0`, the `to_string()` inside `format!`, and the repeated `out = out + ..`. The derived `Clone` copies the view count too; decide whether that is what callers expect.

</details>

## Drills

Run `cargo drill ch09`. The compiler output explains *what* happened; these notes explain *why*. Read one only after the drill's verdict.

<details><summary>d01 · Update syntax moves fields</summary>E0382. `..base` moved `base.path` (a String) into `custom`, so `base.path` is gone; `base.retries` was only copied. Supply `path: base.path.clone()` in the new struct, or borrow it before the move if you only need to read it.</details>
<details><summary>d02 · Private fields outside the module</summary>E0451. Fields are private to their module unless marked `pub`, and a struct expression must name every field, so outside code cannot build `Price` at all. Add a `pub fn new(cents: u32) -> Price` to `shop` and call it; that constructor is where invariants belong.</details>
<details><summary>d03 · Newtypes are distinct types</summary>E0308. `UserId` and `OrderId` are different nominal types even though both wrap a u64; that is the point of a newtype. Pass `order`.</details>
<details><summary>d04 · Associated functions are not methods</summary>E0599. `new` has no `self` parameter, so it is not a method and cannot be called with dot syntax. Call it through the type: `Gauge::new(5)`.</details>
<details><summary>d05 · Calling a &mut self method</summary>E0596. `counter.bump()` auto-borrows `&mut counter`, which needs a `mut` binding. Write `let mut counter`.</details>
<details><summary>d06 · Derives need every field</summary>E0369. Derived `PartialEq` compares field by field, and `Celsius` has no `==`. Add `#[derive(PartialEq)]` to `Celsius` (f64 supports PartialEq but not Eq, so `Eq` could not be derived).</details>
<details><summary>d07 · RefCell borrows at run time</summary>Panics: "already borrowed". The `first` guard is still alive when `borrow_mut` runs, and RefCell enforces one writer or many readers at run time. Read the length with a temporary guard (`let before = log.borrow().len();`) or `drop(first)` before mutating.</details>
<details><summary>d08 · Update syntax with Copy fields</summary>Compiles and runs. `..base` copies Copy fields instead of moving them, so `base` stays fully usable when every remaining field is Copy.</details>
<details><summary>d09 · Moving a field out of &self</summary>E0507. Through `&self` you may read the String but not move it out, because the caller still owns the Badge. Return `&str` (`&self.name`), take `self` by value to consume the badge, or clone if you really need a second String.</details>
<details><summary>d10 · A struct that borrows</summary>E0106. A struct field that is a reference needs a lifetime parameter: `struct Excerpt<'a> { text: &'a str }`. It tells the compiler an Excerpt cannot outlive the text it points into.</details>
