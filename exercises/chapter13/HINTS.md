# Chapter 13 hints

[Chapter guide](README.md) · [All hints](../../HINTS.md)

Try the chapter reference and compiler diagnostics first. Open one level at a time.

## 01 · Destruction and resource lifetimes

<details><summary>Hint 1</summary>

A destructor runs when its owner leaves scope; local bindings usually drop in reverse declaration order.

</details>
<details><summary>Hint 2</summary>

Push the label into the shared log. In scoped_order, create outer, create inner in a nested scope, explicitly drop early, then return a clone of the final log.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

The compiler inserts the destructor call when an owner goes out of scope. If you could call `value.drop()` yourself, the value would still be in scope and would be dropped a second time, so it is forbidden (E0040). `std::mem::drop` is just `fn drop<T>(_x: T) {}`: it takes ownership, and the value is destroyed when that empty function returns. Locals drop in reverse declaration order, struct fields in declaration order (P1). `let _ = value` does not bind, so a temporary is dropped at once (P2), and assignment drops the old value (P3). `mem::forget`, `Rc` cycles, and `Box::leak` skip destructors. That is memory-safe, but whatever `Drop` would have released (memory, file handles, locks) leaks.

</details>

## 02 · Sized, Clone, and Copy

<details><summary>Hint 1</summary>

Unsized referents can be borrowed through sized references. Clone and Copy express different ownership operations.

</details>
<details><summary>Hint 2</summary>

Use size_of_val for an unsized referent. Clone the vector before changing the copy; return both Copy values after assignment.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

`Copy` tells the compiler that duplicating the bytes produces a valid, independent value, so assignment and passing by value copy implicitly and leave the source usable. There is no code to customize: a `Copy` type's `clone` must do the same as the bitwise copy. A type is `Copy` only if every field is (`String`, `Vec`, and `Box` are not, because they own heap memory). A type with `Drop` cannot be `Copy` either, since each bitwise copy would run the destructor. `Clone` is explicit and may do real work, such as allocating a new buffer for a `Vec<String>`. `clone_from` can reuse the target's existing allocation (P2). `?Sized` lets a generic function accept unsized referents like `str` and `[T]` behind a reference, which is then a fat pointer (P1).

</details>

## 03 · Deref, DerefMut, and Default

<details><summary>Hint 1</summary>

Deref targets the wrapped vector; method lookup can then find Vec and slice methods.

</details>
<details><summary>Hint 2</summary>

Return references to self.0. Use Vec::new for Default; test mutation through coercion as well as method lookup.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

`Deref` suits a type that is a transparent smart pointer or a thin wrapper around a value it wants to expose, such as `Samples` around a slice. Deref coercion then turns `&Samples` into `&[i32]`, and method lookup finds slice methods, though inherent methods on the wrapper still win (P1). `deref` returns a reference, so it never transfers ownership; moving out through `*wrapper` is an error. For a domain type, `Deref` is surprising: callers silently get every method of the target, including ones that break your invariants, and it reads like inheritance, which Rust does not have. `Default` should produce a meaningful value (here, an empty sequence), not necessarily zero bytes.

</details>

## 04 · AsRef and AsMut views

<details><summary>Hint 1</summary>

AsRef and AsMut expose a cheap reference to another representation without consuming it.

</details>
<details><summary>Hint 2</summary>

Obtain the slice once with as_ref/as_mut. Sum into u64 and fill through a mutable byte slice.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

`AsRef<[u8]>` means "can cheaply lend me a `&[u8]`". `Vec<u8>`, arrays, `&[u8]`, `String`, and `str` all implement it, so one generic parameter accepts every owned and borrowed shape without the caller converting first. The conversion is a borrow of the existing storage (P2), so it never allocates, and `AsMut` lends `&mut [u8]` for in-place edits. `AsRef` promises nothing about equality or hashing: it is only a cheap view. `Path` is the classic example: `&str`, `String`, `OsStr`, and `PathBuf` all lend `&Path`, and OS paths need not be UTF-8.

</details>

## 05 · Borrow and borrowed lookup

<details><summary>Hint 1</summary>

Borrowed lookup uses a borrowed key whose identity agrees with the stored key. A mutation must preserve the meaning of any surrounding container invariants.

</details>
<details><summary>Hint 2</summary>

Use map.get(name).copied(). For the generic mutable string helper, call BorrowMut::borrow_mut then push_str.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

`Borrow<B>` is `AsRef` plus a promise: the borrowed form compares, orders, and hashes exactly like the owner. That promise is what lets `HashMap<String, V>` look up a `&str` key: the map hashes the `&str`, and the result must match the hash of the stored `String` (P1). A case-insensitive key type must not implement `Borrow<str>`, because two keys equal to each other could hash differently as plain `str`. `BorrowMut` adds mutable access with the same identity guarantee. Never mutate a key in place inside a map, because its hash bucket would then be wrong.

</details>

## 06 · Infallible and fallible conversions

<details><summary>Hint 1</summary>

Conversion traits are for clear representation changes. Validate before constructing a constrained value.

</details>
<details><summary>Hint 2</summary>

Wrap String directly. Accept only ports 1..=65535 from u32; use map over TryInto in the generic helper.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

`From<A> for B` promises a conversion that always succeeds and keeps the value's meaning. Std then provides `Into<B> for A` automatically, along with an infallible `TryFrom`. A conversion that can fail or lose information, like `u32` to `Port`, belongs in `TryFrom`, whose `Result` and `Error` type make the failure visible at the call site. `as` silently truncates or rounds instead (P1, P3). Taking `String` by value lets `From` move the buffer in with no copy, and `label_text` can move it back out the same way.

</details>

## 07 · ToOwned and clone-on-write

<details><summary>Hint 1</summary>

Return Borrowed when no transformation is necessary; allocate only for changed output.

</details>
<details><summary>Hint 2</summary>

Check for carriage returns before calling replace. to_mut gives a mutable owned String, cloning borrowed input only when needed.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

`Clone` turns `&T` into `T`, which cannot work for unsized types like `str` or `[T]`: you cannot return a `str` by value. `ToOwned` generalizes it with an associated `Owned` type, `str -> String` and `[T] -> Vec<T>`. `Cow<'a, B>` holds either a `&'a B` or a `B::Owned`, so a function can return borrowed input untouched and allocate only when it has to change something. `to_mut` gives `&mut Owned`, cloning a borrowed value first (even if you then make no edit, P2), and on an already owned value it edits in place. `into_owned` consumes the Cow and returns the owned value, allocating only if it was borrowed.

</details>

## 08 · Capstone: validated names with cheap views

<details><summary>Hint 1</summary>

Validation can return a borrowed trimmed substring; ownership is needed only when constructing the final Name.

</details>
<details><summary>Hint 2</summary>

Trim first and reject empty results. Delegate AsRef to the stored String and use Cow for a temporary display annotation.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

`TryFrom<&str>` is the validation boundary: it trims, rejects blank input with an error, and allocates once for the accepted text, so every `Name` value is valid. `AsRef<str>` is a cheap read-only view of that validated text, with no copy and no way to break the invariant. `Default` would be misleading because there is no meaningful blank name, and neither `From<&str>` (cannot fail) nor `DerefMut` (would allow unvalidated edits) should exist. `Borrow<str>` is safe only while `Name`'s equality and hash are those of its text. `annotated` returns `Cow`, which borrows when the tag is empty and allocates only when it adds text.

</details>

## R01 · Spaced review of Chapters 9, 11, and 12

<details><summary>Hint 1</summary>

Widen before you multiply: `u32::MAX * 100` does not fit in `u32`. A `for` loop over a `Vec` by value hands you each `Box` as an owned value, and it is dropped at the end of that iteration. Struct update syntax fills the remaining fields from another value of the same type.

</details>
<details><summary>Hint 2</summary>

`Cents(u64::from(dollars) * 100 + u64::from(cents))`. `for job in jobs { job.run(); }`. `Settings { retries: 3, name: "anonymous".into(), verbose: false }`, and `Settings { name: name.into(), ..Default::default() }`.

</details>

## C01 · Code review

<details><summary>Hint 1</summary>

There are two bugs: one function panics on input it should reject (and accepts a value it should reject), and one doubles some line breaks. `From` is the wrong trait for a conversion that can fail, and two functions copy text they only need to read.

</details>
<details><summary>Hint 2</summary>

Implement `TryFrom<&str> for Port` with an error type, parse into `u16`, and reject 0. Return `Cow<'_, str>` from `unix_newlines`: borrow when `!text.contains('\r')`, otherwise `Cow::Owned(text.replace("\r\n", "\n"))`. Take `&[String]` (or `&[impl AsRef<str>]`) in `total_chars` and sum `n.chars().count()` over borrowed names.

</details>
<details><summary>Check your review</summary>

`From<&str> for Port` hides a fallible parse behind an infallible trait and `unwrap`s it, so bad input panics instead of returning an error. It also accepts `"0"`. `TryFrom` with a real error type fixes both, and callers already use `Port::try_from`. `unix_newlines` replaces every `'\r'` with `"\n"`, turning `"\r\n"` into `"\n\n"`. It always copies the text (twice: `to_string`, then `replace`) even when there is nothing to change; `Cow` borrows in that case. `total_chars` clones every name just to count its characters, and takes `&Vec<String>` where a slice is enough (clippy: `ptr_arg`, `map_clone`).

</details>

## Drills

Run `cargo drill ch13`. The compiler output explains *what* happened; these notes explain *why*. Read one only after the drill's verdict.

<details><summary>d01 · Copy and Drop cannot mix</summary>E0184. A `Copy` value is duplicated bit for bit, so each copy would run the destructor and the resource would be released twice. Remove `Copy` and keep `Clone`.</details>
<details><summary>d02 · Calling drop yourself</summary>E0040. The compiler calls `Drop::drop` when the owner goes out of scope; calling it yourself would run it twice. Write `drop(conn);`: `std::mem::drop` takes ownership, so the value is gone afterward.</details>
<details><summary>d03 · Generic parameters are Sized by default</summary>E0277: `str` has no size known at compile time, and `T` implicitly requires `Sized`. Write `T: ?Sized`; `&T` is then a fat pointer when `T` is unsized.</details>
<details><summary>d04 · Moving out through Deref</summary>E0507. `deref` returns `&Vec<i32>`, so `*samples` is a place you can only borrow, not move out of. Move the field directly (`samples.0`), clone (`(*samples).clone()`), or borrow (`let inner = &*samples;`).</details>
<details><summary>d05 · From and TryFrom for the same type</summary>E0119. `From<u16>` already gives you `TryFrom<u16>` through std's blanket impl (with `Error = Infallible`), so yours conflicts. A conversion that can fail should implement only `TryFrom`; delete the `From` impl.</details>
<details><summary>d06 · Looking up String keys with &str</summary>Compiles. `String: Borrow<str>`, and `HashMap::get` accepts any `&Q` where the key type borrows as `Q`, so no temporary `String` is needed.</details>
<details><summary>d07 · Cow cannot borrow a local</summary>E0515: `replaced` is dropped when the function returns, so a borrow of it cannot be returned. Return the owned value: `Cow::Owned(replaced)`.</details>
<details><summary>d08 · TryFrom at run time</summary>Panics: 300 does not fit in a `u8`, so `try_from` returns `Err` and `unwrap` panics. Handle the error, here with `.unwrap_or(u8::MAX)` to saturate. Never use `as`, which would silently give 44.</details>
<details><summary>d09 · Deriving Clone needs Clone fields</summary>E0277: the derived `clone` calls `clone` on every field, and `Token` does not implement `Clone`. Add `#[derive(Clone)]` to `Token` (or `Clone, Copy`, since it holds only a `u64`).</details>
<details><summary>d10 · Into needs to know the target</summary>E0283: `u8` implements `Into` for many types (`u16`, `u32`, `i64`, `f64`...), and nothing pins down which one. Annotate: `let wide: u16 = small.into();`, or write `u16::from(small)`.</details>
