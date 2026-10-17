# Expansion, diagnostics, and procedural macros

## Inspect expansions

In an isolated scratch crate copy one of your own macros. Use your IDE expansion view or the current cargo-expand tool following its official installation instructions; if unavailable, write the expansion by hand and compile it beside the invocation. Inspect nested repetitions, helper paths, local bindings, return, and ? inside a macro argument. Use stringify!, concat!, env!/option_env!, include_str! with your own fixture, cfg!, and dbg! in small experiments. Acceptance: record an invocation, its predicted expansion, actual diagnostics, and one corrected expansion. Do not rely on unstable trace_macros/log_syntax in the core package; if you try them on a separate nightly toolchain, label the requirement.

## A procedural derive in an isolated workspace

Create a temporary workspace with `summary-derive` (lib.proc-macro=true) and `demo`. Add current syn (derive/full features as needed), quote, and proc-macro2 using official docs. Define a `Summarize` trait in demo returning Vec<(&'static str,String)>. Implement `#[derive(Summarize)]` for named-field structs, generating field names and Display-formatted values. Use syn::Error::new_spanned(...).to_compile_error() for enums, unions, and tuple structs instead of panicking. Preserve generics and where clauses and add Display bounds only where needed. The generated trait path must have a documented resolution policy; test a nested module and a renamed dependency or provide a helper attribute naming the trait path.

Acceptance: zero-field named struct, ordinary fields, generic field, lifetime-bearing borrowed field, existing where clause, field unsupported by Display, and rejected enum/tuple struct each have a concrete compile/run check. An isolated trybuild suite is appropriate for intentional compile failures. Compare this derive with the macro_rules item generator in l03. Explain the TokenStream boundary and why the derive needs its own crate.

## Optional attribute macro

Add `#[audit_name("...")]` to a function; parse the attribute arguments, reject duplicate/unknown options with spanned diagnostics, and generate a wrapper that records a supplied label through an explicit callback rather than a global logger. Preserve visibility, generics, asyncness, and return type, or reject unsupported signatures clearly. Do not silently drop attributes. Acceptance: two good cases and three precise rejected cases, plus an explanation of hygiene differences between procedural and declarative macros.

References: [Rust Reference: macros by example](https://doc.rust-lang.org/reference/macros-by-example.html), [procedural macros](https://doc.rust-lang.org/reference/procedural-macros.html), [syn](https://docs.rs/syn/latest/syn/), and [quote](https://docs.rs/quote/latest/quote/).

## Real external consumer

Complete and run [export_lab](export_lab/README.md), an isolated library and a consumer that renames its dependency. Check public module reachability as well as `$crate` hygiene. This no-dependency export fixture is compile-checked; its complete reference version is tested separately.
