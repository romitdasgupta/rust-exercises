# Expansion, diagnostics, and procedural macros

These labs run outside the exercise crate, in temporary projects of their own (except Lab 4, which is supplied). Work in a directory outside this repository, for example `~/scratch/ch22`, so experiments never break `cargo test --all-targets`.

## Lab 1 · Inspect expansions

Goal: see what your own macros expand to, and connect the expansion to the diagnostics you get.

Minimal `Cargo.toml` (no dependencies):

```toml
[package]
name = "expand-lab"
version = "0.1.0"
edition = "2024"
```

1. Create an isolated scratch crate (`cargo new --lib expand-lab`) and copy in one of your finished macros from this chapter, for example `ordered_map!` or `data!`.
   *Checkpoint:* `cargo test` in the scratch crate compiles your macro unchanged.
2. Choose how you will see expansions: your IDE's expansion view, or the current cargo-expand tool installed by following its official instructions. If neither is available, write the expansion by hand and compile it beside the invocation.
   *Checkpoint:* for one invocation you can see (or have written) the code it becomes.
3. Before expanding each case below, write your predicted expansion. Then compare it with the real expansion:
   - nested repetitions;
   - helper paths (`$crate::...` versus a bare name);
   - local bindings introduced by the macro;
   - `return` inside a macro argument;
   - `?` inside a macro argument.
   *Checkpoint:* you should now see that `return` and `?` in an argument affect the *caller's* function, and that the macro's own locals keep their hygiene even though they print with ordinary names.
4. Run one small experiment with each built-in: `stringify!`, `concat!`, `env!` and `option_env!`, `include_str!` with a fixture file you create yourself, `cfg!`, and `dbg!`.
   *Checkpoint:* you can say for each one whether it runs at compile time or at run time, and what happens when its input is missing (for example `env!` on an unset variable).
5. Break one invocation on purpose (a missing separator or a wrong fragment), record the diagnostic, then fix the macro or the invocation.
6. Do not rely on the unstable `trace_macros!` or `log_syntax!` in the core package. If you try them on a separate nightly toolchain, label that requirement clearly in your notes.

Acceptance criteria:

- [ ] One invocation recorded with its predicted expansion.
- [ ] The actual expansion or diagnostics recorded next to the prediction.
- [ ] One corrected expansion that compiles.
- [ ] Each case in step 3 and each built-in in step 4 has a written observation.
- [ ] Any nightly-only experiment is labeled as such.

## Lab 2 · A procedural derive in an isolated workspace

Goal: implement `#[derive(Summarize)]` and see why procedural macros need their own crate.

Minimal workspace (syn 2, quote 1, and proc-macro2 1 at the time of writing; check the official docs for the current versions and features):

```toml
# Cargo.toml (workspace root)
[workspace]
members = ["summary-derive", "demo"]
resolver = "3"

# summary-derive/Cargo.toml
[package]
name = "summary-derive"
version = "0.1.0"
edition = "2024"

[lib]
proc-macro = true

[dependencies]
syn = { version = "2", features = ["derive"] }  # add "full" only if you need it
quote = "1"
proc-macro2 = "1"

# demo/Cargo.toml
[package]
name = "demo"
version = "0.1.0"
edition = "2024"

[dependencies]
summary-derive = { path = "../summary-derive" }

[dev-dependencies]
trybuild = "1"  # optional: for intentional compile failures
```

1. Create the temporary workspace with the two crates above: `summary-derive` (`proc-macro = true`) and `demo`.
   *Checkpoint:* `cargo build` succeeds with an empty derive function that returns an empty `TokenStream`.
2. In `demo`, define the trait:

   ```rust
   pub trait Summarize {
       fn summarize(&self) -> Vec<(&'static str, String)>;
   }
   ```

3. Implement `#[derive(Summarize)]` for structs with named fields. Generate one `(field_name, value)` pair per field, in declaration order, with each value formatted by `Display`.
   *Checkpoint:* `#[derive(Summarize)] struct Job { id: u32, name: String }` returns `[("id", "7"), ("name", "build")]` for `Job { id: 7, name: "build".into() }`.
4. Reject enums, unions, and tuple structs with `syn::Error::new_spanned(...).to_compile_error()` instead of panicking.
   *Checkpoint:* deriving on an enum gives an error that points at the enum, not a "proc macro panicked" message.
5. Preserve generics and existing `where` clauses (`split_for_impl` helps). Add `Display` bounds only where they are needed.
   *Checkpoint:* a struct with a generic field `T` and an existing `where` clause derives cleanly, and the generated impl requires `T: Display`.
6. Choose and document how the generated code names the trait (for example `::demo::Summarize`, or a helper attribute that names the trait path). Test the policy from a nested module, and either test a renamed dependency or provide that helper attribute.
   *Checkpoint:* the derive works when invoked from `mod inner { ... }`, and your policy for a renamed crate is written down and tested.
7. Write the checks listed below. Use run-time tests for the cases that compile, and an isolated trybuild suite for the intentional compile failures.
8. Write up: compare this derive with the `macro_rules!` item generator in l03 (`define_cell!`), explain the `TokenStream` boundary, and explain why the derive needs its own crate.

Acceptance criteria (each case has a concrete compile or run check):

- [ ] Named struct with zero fields.
- [ ] Ordinary fields.
- [ ] A generic field.
- [ ] A lifetime-bearing borrowed field (for example `name: &'a str`).
- [ ] An existing `where` clause.
- [ ] A field whose type does not implement `Display` (fails with a clear trait-bound error).
- [ ] A rejected enum and a rejected tuple struct, with spanned errors, not panics.
- [ ] The trait-path resolution policy is documented and tested (nested module, plus renamed dependency or helper attribute).
- [ ] Written comparison with l03 and explanation of the `TokenStream` boundary and the separate crate.

## Lab 3 · Optional attribute macro

Goal: parse attribute arguments, report precise errors, and generate a wrapper without losing anything from the original function.

Minimal manifest (a new crate in the Lab 2 workspace):

```toml
# audit-attr/Cargo.toml
[package]
name = "audit-attr"
version = "0.1.0"
edition = "2024"

[lib]
proc-macro = true

[dependencies]
syn = { version = "2", features = ["full"] }
quote = "1"
proc-macro2 = "1"
```

1. Add `#[audit_name("...")]` for functions. Parse the attribute's arguments.
   *Checkpoint:* `#[audit_name("load")] fn load() {}` compiles and the label `"load"` reaches your generated code.
2. Reject duplicate and unknown options with spanned diagnostics.
   *Checkpoint:* `#[audit_name("a", color = 1)]` reports an error under `color`, not at the attribute as a whole.
3. Generate a wrapper that records the supplied label through an explicit callback the caller passes in, rather than through a global logger.
4. Preserve visibility, generics, asyncness, and return type, or reject unsupported signatures with a clear error. Do not silently drop other attributes on the function (doc comments, `#[inline]`, `#[must_use]`, and so on).
   *Checkpoint:* `pub async fn fetch<T: Clone>(x: T) -> T` keeps its signature exactly, or is rejected with a message that says why.

Acceptance criteria:

- [ ] Two working cases.
- [ ] Three precise rejected cases, each with a spanned diagnostic.
- [ ] No attribute on the original function is silently dropped.
- [ ] Written explanation of the hygiene differences between procedural and declarative macros.

## Lab 4 · Real external consumer

Goal: prove that `ch22_event!` works from another crate that renames its dependency.

The workspace is supplied: [export_lab](export_lab/README.md) has an isolated library and a consumer. It needs no registry dependencies. The library's only dev-dependency is this repository's own `practice` helper crate.

1. Finish l06 (both `todo!()`s and its `p_predictions`).
2. From the repository root, run `cargo test --manifest-path exercises/chapter22/export_lab/Cargo.toml`.
   *Checkpoint:* the consumer test `exported_macro_resolves_public_definition_path` passes, even though the consumer renames the library and defines its own `format_event`.
3. Check public module reachability: temporarily remove `pub` from `pub mod l06_exports` in `export_lab/library/src/lib.rs`, record the privacy error the consumer gets, then restore it.
4. Check `$crate` hygiene: temporarily replace `$crate::` with a bare `format_event` call in l06, record which helper the consumer now reaches (or which error it gets), then restore it.

Acceptance criteria:

- [ ] The consumer test passes with the renamed dependency.
- [ ] The privacy error from step 3 is recorded and explained.
- [ ] The `$crate` experiment from step 4 is recorded and explained.

This no-dependency export fixture is compile-checked by CI. Its complete reference version is tested separately.

References: [Rust Reference: macros by example](https://doc.rust-lang.org/reference/macros-by-example.html), [procedural macros](https://doc.rust-lang.org/reference/procedural-macros.html), [syn](https://docs.rs/syn/latest/syn/), and [quote](https://docs.rs/quote/latest/quote/).
