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
<details><summary>Check your explanation (after writing your own)</summary>

A *package* is a directory with a `Cargo.toml`; it contains one or more *targets* (a library, binaries, examples, tests), and each target is compiled as its own *crate*. That is why this lesson's tests live in a crate named `chapter08` inside the package `types`, and the support library is the crate `practice`, named by `[lib]`. The installed toolchain (`rustc --version`, pinned here by `rust-toolchain.toml`) decides which compiler runs; `edition` selects language rules such as keywords and defaults; `rust-version` declares the oldest compiler the package promises to support. A *profile* (dev, release, test) chooses settings like optimization and `debug_assertions`, which `cfg!` can observe at compile time.

</details>

## 02 · Nested modules and private implementation

<details><summary>Hint 1</summary>

Items are private by default. Export the API while keeping the calculation helper private.

</details>
<details><summary>Hint 2</summary>

The public cost function can invoke a nested module function marked pub(super). Use widened u32 arithmetic.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

Items are private to the module that declares them and visible to that module's descendants, so a child can use its ancestors' private items, but a parent cannot see into a child unless the child marks items `pub`, `pub(super)`, or `pub(crate)`. A module boundary should expose operations that keep its invariants true, such as "zero items are free", rather than raw pieces a caller could combine wrongly. Keeping `rules::fee` private means you can change the fee formula without breaking any caller. Moving a module into `tariff.rs` and `tariff/rules.rs` changes the files but not the paths, because `mod` declarations define the tree.

</details>

## 03 · Paths, imports and reexports

<details><summary>Hint 1</summary>

A facade can reexport a public type from a private implementation module.

</details>
<details><summary>Hint 2</summary>

Create Ticket through its inherent new constructor; name may be borrowed through an accessor while representation stays private.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

`crate::` starts at the crate root, `self::` at the current module, and `super::` at the parent; a bare path is resolved from the current scope. `use` creates a new *name* for an existing item, not a new item: an alias and a `pub use` reexport both name the same type, which is why `type_name` still reports the defining module. Names in the standard prelude, such as `Option`, `Some`, and `drop`, are available everywhere, but a local item with the same name shadows them. A facade `pub use` lets callers write a short stable path while the implementation module stays private, and `pub struct` with private fields forces callers through constructors that enforce the invariants.

</details>

## 04 · Constants and statics

<details><summary>Hint 1</summary>

Constants are values evaluated at compile time; immutable statics have one storage location.

</details>
<details><summary>Hint 2</summary>

Compute a byte count from the constant. Borrow the static array directly instead of constructing an equal temporary.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

A `const` is a value: the compiler pastes it into every place it is used, so different uses are not guaranteed to share an address. A `static` is one location in memory that lives for the whole program, so every borrow of it, from any function, points to the same bytes, and a static declared inside a function body is still shared by every call. Only `const fn`s can run in a const or static initializer. Because a static is shared by all threads, mutating one requires an atomic, a lock, or `unsafe` with a proof that no data race can occur; prefer the first two.

</details>

## 05 · Attributes, cfg and included files

<details><summary>Hint 1</summary>

Attributes can remove code before type checking. include_str! embeds UTF-8 content into the executable.

</details>
<details><summary>Hint 2</summary>

Use cfg! for the target family value. Split the included literal by lines and trim nonempty lines.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

`#[cfg(...)]` removes an item or statement before type checking, so code behind a false predicate is never compiled at all. `cfg!(...)` is just a `bool` known at compile time; both branches of an `if cfg!(...)` must still compile, and the optimizer then drops the dead one. `cfg!(all())` is true and `cfg!(any())` is false, which makes them handy switches. `env!` and `include_str!` are evaluated when the crate is compiled, embedding the value or file contents in the binary; `include_str!` paths are relative to the source file. Build scripts can generate files into `OUT_DIR` and pass values to `env!` with `cargo::rustc-env`.

</details>

## 06 · Tests and executable documentation

<details><summary>Hint 1</summary>

Public APIs should make their normalization rules precise enough for examples and tests.

</details>
<details><summary>Hint 2</summary>

trim and split_whitespace cover leading/trailing and repeated Unicode whitespace. Test the public contract rather than internal steps.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

Unit tests sit inside the crate, usually in a `#[cfg(test)]` module, so they can test private items. Integration tests in `tests/` are separate crates that can only use the public API, exactly like a real consumer, so they prove what the public contract is. Doc-tests are the examples in `///` comments of a library; `cargo test` compiles and runs them, so documentation that drifts from the code fails the build, and a `compile_fail` example can show that something is rejected. Assertions like `assert_eq!` report both values, which makes failures easier to diagnose than a bare `assert!`.

</details>

## 07 · Dependency requirements and lockfiles

<details><summary>Hint 1</summary>

These tasks model only the numeric stable-version caret intervals explicitly stated, not full Cargo semver parsing.

</details>
<details><summary>Hint 2</summary>

For a nonzero major caret interval, keep the same major and compare minor/patch lexicographically. For 0.minor with nonzero minor, keep both major and minor.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

A requirement such as `"1.2.3"` is a *caret* range: any version that should be compatible according to SemVer, here `>=1.2.3, <2.0.0`. Before 1.0 the leftmost non-zero component acts as the major version, so `"0.2.3"` means `>=0.2.3, <0.3.0`. Cargo resolves each requirement to one concrete version and records the result in `Cargo.lock`, one lockfile per workspace, so every build uses the same versions until you run `cargo update`. Path dependencies point at a local directory, which is ideal for offline experiments and for crates developed together. Versions must be compared as numbers, not as strings, because "1.10.0" sorts before "1.9.9" as text.

</details>

## 08 · Capstone: library facade and workspace

<details><summary>Hint 1</summary>

Keep validation private, reexport the public function, and let command-line concerns call the library.

</details>
<details><summary>Hint 2</summary>

The facade accepts text, validates and formats it; a batch function propagates the first indexed failure.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

The library exposes a small facade, `tag` (and a batch helper), while validation stays private, so its rules can change without breaking callers. A CLI binary would be a thin layer that reads arguments, calls the library, and maps errors to exit codes; keeping it out of the library makes the logic testable and reusable. An external integration test proves the privacy: it can call `tag`, but it cannot name `core::validate`. Before publishing, `cargo package --list` shows exactly which files ship in the `.crate` archive; `Cargo.lock` is used for binaries and ignored by consumers of a library. Every `pub` item is part of the compatibility promise, so expose as little as possible.

</details>

## R01 · Spaced review of Chapters 4, 6 and 7

<details><summary>Hint 1</summary>

`?` calls `From::from` on the error, so once the `From` impl exists, `text.trim().parse::<u16>()?` produces a `ConfigError`. For B, find where the digits end. For C, ownership can move out of the vector without copying any text.

</details>
<details><summary>Hint 2</summary>

A2: `let port: u16 = text.trim().parse()?;` then reject zero. B: `let split = text.find(|c: char| !c.is_ascii_digit()).unwrap_or(text.len());`, parse `&text[..split]` (empty fails), and `match &text[split..] { "" => Some(n), unit => { let (_, m) = UNITS.iter().find(|(u, _)| *u == unit)?; n.checked_mul(*m) } }`. C: `Vec::extract_if(.., |m| m.starts_with(prefix)).collect()`, or `mem::take` the vector and `partition` it.

</details>

## C01 · Code review

<details><summary>Hint 1</summary>

There are two bugs: one comparison lets the bin hold one item too many, and one counter can hand out the same id twice. A public field and a cloning getter make the module boundary useless.

</details>
<details><summary>Hint 2</summary>

Make `items` private and return `&[String]` from `items()`; the tests call only methods. Compare with `>=`. Make the read and the write one atomic step: return `NEXT_ID.fetch_add(1, Ordering::Relaxed) + 1`.

</details>
<details><summary>Check your review</summary>

`Bin::add` checks `len() > CAPACITY`, so a fifth item fits; and because `items` is `pub`, any caller can push past the limit anyway, so the type cannot promise its invariant. Make the field private; the module boundary is what makes the invariant enforceable. `items()` clones the whole vector on every call where a borrowed `&[String]` is enough. `next_id` loads and then stores as two separate steps: two threads can load the same value and both return it (a lost update). Each operation is atomic, so this is not undefined behavior, but the sequence is not. `fetch_add` makes read-modify-write a single atomic step. The `static mut` version this replaces would have been worse: a real data race, which is undefined behavior.

</details>

## Drills

Run `cargo drill ch08`. The compiler output explains *what* happened; these notes explain *why*. Read one only after the drill's verdict.

<details><summary>d01 · Private by default</summary>E0603: `fee` has no `pub`, so it is private to `billing` and its children; `main` is in the parent. Write `pub fn fee` (or `pub(crate)`). Privacy is checked by module, not by file.</details>
<details><summary>d02 · A pub struct with a private field</summary>E0451: a struct literal must set every field, and `owner` is private outside `tickets`. That is the point of private fields: callers must go through `Ticket::new`, which trims the owner. Use the constructor.</details>
<details><summary>d03 · A path that does not exist</summary>E0432: `Circle` lives in `shapes::round`, not `shapes`. Write `use shapes::round::Circle;`, or add `pub use round::Circle;` inside `shapes` to give it a shorter public path.</details>
<details><summary>d04 · A const needs a const fn</summary>E0015: a const initializer is evaluated by the compiler, which can only call `const fn`s. Mark `kib` as `const fn`; the multiplication is allowed in const context.</details>
<details><summary>d05 · A static must be shareable</summary>E0277: every thread can reach a `static`, so its type must be `Sync`, and `Cell` is not. For a per-thread counter, use `thread_local! { static HITS: Cell<u32> = const { Cell::new(0) }; }` (the `KEEP` line still works, because `LocalKey<Cell<T>>` has `get` and `set`). For one shared counter, use `AtomicU32`.</details>
<details><summary>d06 · cfg removes items</summary>E0425: the `fast` feature is not enabled, so `#[cfg(feature = "fast")]` removed `helper` before name resolution, and nothing called `helper` exists. Remove the attribute, enable the feature (in Cargo, `--features fast`), or provide a `#[cfg(not(feature = "fast"))]` version.</details>
<details><summary>d07 · What cfg hides is not type-checked</summary>Compiles and runs. Code behind `#[cfg(any())]` is removed before type checking, so the ill-typed `broken` is never compiled. The code must still *parse*, though. `if cfg!(any()) { ... }` would be different: both branches are type-checked.</details>
<details><summary>d08 · Re-exporting a crate-private item</summary>E0364: `pub use` cannot make an item more visible than it is declared; `start` is only `pub(crate)`. Make it `pub fn start`, or keep it crate-private and write `pub(crate) use engine::start;`.</details>
<details><summary>d09 · Two globs, one name</summary>E0659: both globs bring a `parse` into scope, and neither wins, so the call is ambiguous. Import the one you mean explicitly (`use toml::parse;`), because an explicit import shadows glob imports, or call `toml::parse` with its path.</details>
<details><summary>d10 · An environment variable read at compile time</summary>Panics at run time: `option_env!` is evaluated while *compiling*, and the variable was not set then, so it produced `None` and `expect` panics when the program runs. Use `.unwrap_or("default")`. `env!` would instead fail the build, and `std::env::var` reads the variable at run time.</details>
