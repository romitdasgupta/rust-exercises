# Chapter 22 hints

[Chapter guide](README.md) · [All hints](../../HINTS.md)

Try the first hint before opening the second. The implementation constraints in each task are part of the exercise.

## l01 · Expansion and evaluation once

<details><summary>Hint 1</summary>

Bind the expression once inside the expansion rather than substituting it twice.

</details>

<details><summary>Hint 2</summary>

A block expression can contain let statements and return its final expression. Built-in macros expand where the exported expansion invokes them.

</details>

<details><summary>Check your explanation (after writing your own)</summary>

A macro body is plain token substitution. If it says `$value` twice, the caller's expression is evaluated twice, side effects and all. Binding it once with `let value = $value;` inside a block makes the macro behave like a function call. That `value` is hygienic: it has the macro's syntax context, so it can neither see nor shadow a caller variable with the same spelling. Control flow is different. A `return` or `?` inside a macro argument is pasted into the caller's function, so it returns from the *caller*. A function argument is evaluated before the call and cannot do that. Built-ins such as `line!()` and `module_path!()` report the outermost invocation site.

</details>

## l02 · Repetition and separators

<details><summary>Hint 1</summary>

Repeat insertion statements for the map. Counting syntax should transform each fragment into unit, not execute it.

</details>

<details><summary>Hint 2</summary>

An array of repeated () values has a length; stringify! consumes tokens without evaluating their expressions.

</details>

<details><summary>Check your explanation (after writing your own)</summary>

`$( ... ),*` matches zero or more repetitions separated by commas, and `$(,)?` optionally accepts one trailing comma that adds nothing. The transcriber repeats a `$( ... )*` group once per match, and nested repetitions must be repeated at the same depth they were matched. An empty invocation expands to code with no inserts, so the map's key and value types come only from the caller's annotation. Without it, inference fails. Separators are tokens too: a token-tree counter sees `a, b, c` as five trees. `vec![elem; n]` evaluates `elem` once and clones it.

</details>

## l03 · Type and identifier fragments

<details><summary>Hint 1</summary>

Macro-generated items are normal Rust items after expansion; they still need valid implementations.

</details>

<details><summary>Hint 2</summary>

Use Self { value } and &self.value in the item expansion. The turbofish can contain the captured $target:ty.

</details>

<details><summary>Check your explanation (after writing your own)</summary>

`macro_rules!` only checks that the tokens fit each fragment: `ident` is one identifier, `ty` is any type syntax such as `Vec<u8>` or `std::num::NonZeroU8`, `expr` is one expression parsed as a single node, and `literal` also accepts `-5` and `true`. After expansion, the compiler checks the generated items like hand-written code: types, privacy, borrows, and trait bounds. An identifier supplied by the caller keeps the caller's context, so `let $name = 10;` is visible to the caller. A name the macro writes itself is not. Once captured, a fragment such as `$e:expr` is opaque when forwarded: another macro cannot match it as `ident` or against literal tokens like `null`.

</details>

## l04 · Recursive structured literals

<details><summary>Hint 1</summary>

Write a rule for each structural case, then delegate every nested value back to the macro.

</details>

<details><summary>Hint 2</summary>

A tt repetition handles bracketed/braced/grouped values. The final expr rule delegates leaf conversion to From rather than reimplementing type selection.

</details>

<details><summary>Check your explanation (after writing your own)</summary>

The grammar is `null`, `[ tt, ... ]`, `{ "key": tt, ... }`, or any expression. Each container rule calls `data!` again on every element, so nesting is handled by recursion. Every element must be ONE token tree: a delimited group counts as one tree, but `-2` is two (`-` and `2`), so negative or compound values must be wrapped in parentheses. Rules are tried in order, and `null` also parses as an expression (a path), so the special cases must come before the catch-all `$other:expr`. Values are forwarded as `tt`, which stays transparent. A value forwarded as `expr` would no longer match `(null)`. This is a literal builder, not a JSON parser: there is no string escaping and no validation of keys.

</details>

## l05 · Traits handle macro leaf types

<details><summary>Hint 1</summary>

A declarative macro operates on syntax; ordinary trait resolution knows the types.

</details>

<details><summary>Hint 2</summary>

Use a repeated call to one generic helper. The bool Field implementation intentionally differs from Display to expose accidental formatting shortcuts.

</details>

<details><summary>Check your explanation (after writing your own)</summary>

A macro sees only syntax. It cannot know whether `value` is an i32 or a &str, so it should not guess from spelling. Instead, every leaf becomes `render(value)`, and ordinary trait resolution picks the `Field` impl from the leaf's type. The macro handles the shape (repetition, commas, order) and the traits handle the behavior, so adding a type means adding an impl, not a macro rule. An unsupported leaf such as `Vec<u8>` fails with a trait-bound error (E0277) pointing at the expansion, which is clearer than a macro mismatch. Integer literals are inferred from the available impls: with only `Field for i32`, `12` becomes an i32.

</details>

## l06 · Exports and hygiene

<details><summary>Hint 1</summary>

The helper must be reachable through a crate-relative path after expansion in the caller.

</details>

<details><summary>Hint 2</summary>

Use $crate::l06_exports::format_event, because each chapter is its own integration-test crate and l06_exports is the defining module.

</details>

<details><summary>Check your explanation (after writing your own)</summary>

Names that the macro writes itself, such as local variables, are hygienic: they cannot capture or be captured by the caller's locals, and the caller's expression still sees the caller's `count`. Item paths such as function names are NOT hygienic in `macro_rules!`: a plain `format_event(...)` is looked up where the macro is *invoked*, so the caller's own helper, or a missing import, changes the result. `$crate` expands to the defining crate under whatever name the caller gave it, including a renamed dependency, so `$crate::l06_exports::format_event` always reaches the real helper. The path must also be public all the way down: a pub function inside a private module is still unreachable from another crate. `#[macro_export]` places the macro at the crate root, so it is invoked as `crate::ch22_event!` or `dependency::ch22_event!`.

</details>

## l07 · Capstone: structured events

<details><summary>Hint 1</summary>

Separate parsing from constructing the ordinary Event type. Identifier fragments become names via stringify!.

</details>

<details><summary>Hint 2</summary>

Bind kind before repeating fields. A BTreeMap provides deterministic comparison and last-insert-wins behavior.

</details>

<details><summary>Check your explanation (after writing your own)</summary>

The grammar is `kind_expr ; (ident : expr),* ,?`. The kind is bound first, then each value is evaluated once, in source order, and inserted into a BTreeMap. Repeated names overwrite earlier ones, so the last value wins and the map's order is deterministic. Field names come from `stringify!($field)`, and an `ident` fragment guarantees each one is a Rust identifier. Values use `Display` (`to_string`), so the API requires `Display` and a missing impl is an ordinary E0277. Parsing stays in the macro and construction in the plain function `make_event`, which takes owned inputs and moves them in without copying. If the macro were exported, it would call `$crate::...::make_event`. A fallback rule such as `($($t:tt)*) => { compile_error!("expected: kind; name: value, ...") }` turns a mismatch into a readable message.

</details>

## R01 · Spaced review of Chapters 15, 19, and 21

<details><summary>Hint 1</summary>

Each macro is a thin repetition around something you already know: polling a pinned future with a `Context`, spawning inside `std::thread::scope`, and `Iterator::chain`. Write the code for two arguments by hand first, then turn the repeated part into `$( ... )*`.

</details>
<details><summary>Hint 2</summary>

A: build `Context::from_waker(Waker::noop())` once, then `vec![$({ let mut f = pin!($fut); f.as_mut().poll(&mut cx) }),*]`. B: inside `thread::scope(|s| ...)`, make an array of `s.spawn(|| $task)` handles, then join them in order. C: the base case is `IntoIterator::into_iter($only)`; the recursive case is `into_iter($first).chain(chain_all!($($rest),+))`.

</details>

## C01 · Code review

<details><summary>Hint 1</summary>

There are two bugs. In one, an argument appears twice in an expansion. In the other, a helper's name is resolved in the wrong place. Then look for every String that is built only to be copied or thrown away, and for an `unwrap()` inside a function that already returns a `Result`.

</details>
<details><summary>Hint 2</summary>

Bind both arguments once: `match ($a, $b) { (a, b) => if a >= b { a } else { b } }`. Call the helper as `$crate::c01_code_review::tag_name` and build the Vec in one step with `vec![...]`. `format!("#{tag}")` needs no `to_string`. In `fmt`, `write!(f, ...)` writes straight into the formatter and returns its `fmt::Result`.

</details>
<details><summary>Check your review</summary>

`max_of!` pastes `$a` and `$b` into both the comparison and the result, so the winning argument runs twice. Side effects repeat, and the returned value can differ from the one that was compared. `std::cmp::max` would not work as a fix: it returns the second argument on a tie and needs `Ord`, which f64 lacks. `ch22_tags!` calls `tag_name` by its bare name, so the call is resolved where the macro is invoked: any module with its own `tag_name` silently gets different output, and a module without one fails to compile. It also grows a Vec from empty, clones each finished String, and calls `to_string()` inside `format!` (clippy: `to_string_in_format_args`). The Display impl formats into a temporary String, then `unwrap()`s the write, so a failing writer causes a panic instead of an `Err` returned to the caller.

</details>

## Drills

Run `cargo drill ch22`. The compiler output explains *what* happened; these notes explain *why*. Read one only after the drill's verdict.

<details><summary>d01 · Define before use</summary>error (no error code): "cannot find macro". `macro_rules!` names have *textual* scope: a macro is visible only after its definition in the file, unlike functions. Move the definition above `main` (or export it with `#[macro_export]` and use a path).</details>
<details><summary>d02 · A macro cannot see the caller's locals</summary>E0425. The `tax` written in the macro body has the macro's syntax context and is looked up where the macro is *defined*, where no `tax` exists. One fix is to define the macro inside `main` after `let tax`, so `tax` is in scope at the definition. A cleaner API passes it in: `add_tax!(100, tax)`.</details>
<details><summary>d03 · Evaluating an argument twice</summary>Panics. `square!(next())` expands to `next() * next()`, which is 1 × 2 with `calls == 2`. Bind once: `{ let v = $x; v * v }`.</details>
<details><summary>d04 · Grouping of expression fragments</summary>Compiles and runs. An `expr` fragment is substituted as one parsed node, as if parenthesized, so `double!(1 + 2)` is `(1 + 2) * 2`. C preprocessor macros paste text and would give 5. Rust macros do not.</details>
<details><summary>d05 · A trailing comma</summary>error (no error code): "unexpected end of macro invocation". After the final comma the matcher expects another `expr`. Add `$(,)?` after the repetition to accept an optional trailing comma.</details>
<details><summary>d06 · Exported macros and helper paths</summary>E0425. `label` is resolved at the invocation site (`main`), where it is not in scope. Use `$crate::util::label($n)` and make `label` `pub`: the expansion runs in the caller's module, so the helper must be reachable from there.</details>
<details><summary>d07 · The expansion is still type-checked</summary>E0308. The macro produces `vec![1, "two", 3.5]`, and a Vec has one element type. Macros don't bypass type checking. Convert each leaf, for example `vec![$($x.to_string()),*]`, or use a trait as in l05.</details>
<details><summary>d08 · Locals declared by a macro</summary>E0425. The `x` written inside the macro is hygienic: the caller's `x` is a different name even though it is spelled the same. Let the caller supply the identifier: `($name:ident) => { let $name = 1; }` and `make_x!(x)`.</details>
<details><summary>d09 · Where #[macro_export] puts a macro</summary>E0433. `#[macro_export]` always places the macro at the crate root, whichever module defines it. Call `crate::area!(2, 3)`, or `area!` after its textual definition.</details>
<details><summary>d10 · Leaves need the trait the expansion uses</summary>E0277: `Vec<i32>` does not implement `Display`. The macro accepted the syntax, and the trait check failed after expansion. Use `{:?}` (both leaves implement Debug) or define a trait for supported leaves.</details>
