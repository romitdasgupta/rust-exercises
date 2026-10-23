# Chapter 7 hints

[Chapter guide](README.md) · [All hints](../../HINTS.md)

Try the first hint before opening the second. The implementation constraints in each task are part of the exercise.

## 01 · Panics and invariants

<details><summary>Hint 1</summary>

A panic represents a violated invariant here; use the exact requested message.

</details>
<details><summary>Hint 2</summary>

Use assert with the explicit string. The checked task should return Err, leaving panic for the invariant-only API.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

A panic says that the program has a bug: an invariant the code relies on, such as "capacity is never zero", was broken, and continuing could do harm. By default the panicking thread *unwinds*: it runs the destructors of every live local, in reverse order, which is why the drop log in P1 is complete. With `panic = "abort"` the process stops immediately, no destructors run, and `catch_unwind` cannot catch anything. An empty label typed by a user is not a bug but an expected input, so `label` returns `Err` and lets the caller decide. A panic payload is a `&'static str` for a plain literal message and a `String` when the message is formatted.

</details>

## 02 · Catching Result errors and printing context

<details><summary>Hint 1</summary>

Result is a normal enum; handle both paths explicitly.

</details>
<details><summary>Hint 2</summary>

Match parse::<u16>(); format only the error message with the supplied context rather than platform-dependent parse wording.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

Matching on a `Result` is ordinary control flow: no unwinding happens and both arms are just code. Replacing every error with a fallback is a *recovery policy*, and it is only right when the caller truly does not care why parsing failed, because the reason (empty, invalid digit, overflow) is thrown away. A user-facing message such as "jobs: invalid count" adds context the parse error does not have (which setting failed) and stays stable even if the standard library rewords its `Display` text. A CLI prints diagnostics to stderr so they do not mix with the program's real output on stdout.

</details>

## 03 · Propagating failures with question mark

<details><summary>Hint 1</summary>

Use ? to stop at the first failure without writing repetitive nesting.

</details>
<details><summary>Hint 2</summary>

Parse both halves with trim().parse()?; collect or a loop with ? can preserve the same ParseIntError type.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

`expr?` evaluates to the `Ok` value when the result is `Ok`; on `Err(e)` it returns early from the enclosing function with `Err(From::from(e))`. So the function itself must return a `Result` (or an `Option`, for `?` on options) whose error type can be built from `e`. Because `?` returns at the first failure, later inputs are not parsed, which P1 and P2 make visible. Preserving the `ParseIntError` instead of turning it into a default value keeps the failure visible and keeps its kind for the caller.

</details>

## 04 · Result methods and aliases

<details><summary>Hint 1</summary>

map transforms an Ok value; and_then chains another fallible operation.

</details>
<details><summary>Hint 2</summary>

Parse with map_err, then and_then for the nonzero check. For optional parsing, transpose converts Option<Result<T,E>> into Result<Option<T>,E>.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

`map` transforms the `Ok` value and leaves an `Err` untouched; `map_err` does the reverse. `and_then` chains another fallible step: its closure returns a `Result`, so failures from either step come out flat, while `map` with a fallible closure would nest them. `unwrap_or(x)` evaluates `x` before the call, even on `Ok`; `unwrap_or_else(f)` calls `f` only on `Err`. `transpose` swaps `Option<Result<T, E>>` and `Result<Option<T>, E>`, so "absent" stays `Ok(None)` instead of becoming an error. A `type` alias is only another name for the same type: no conversion, no new API.

</details>

## 05 · Multiple error types at a boundary

<details><summary>Hint 1</summary>

A boxed Error trait object can hold different concrete errors and preserve their dynamic identity.

</details>
<details><summary>Hint 2</summary>

Read into a String with Read::read_to_string, then parse with ?. Both error types implement Error.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

`?` converts any error that implements `Error` into `Box<dyn Error>` through a blanket `From` impl, so one boundary function can return I/O failures and parse failures through the same `Result`. The box keeps the concrete error alive and its dynamic type available: `is`, `downcast_ref`, and `source()` still work. What it hides is the static set of possibilities: the compiler can no longer check that a caller handles every case, and recovering means guessing types to downcast to. The box also costs an allocation, but only on the error path. Use a concrete error enum where callers recover by kind, and a boxed error (or `anyhow`) at the application boundary where you mostly report.

</details>

## 06 · Custom errors with a source chain

<details><summary>Hint 1</summary>

Keep parse errors as values inside an enum, and use a separate variant for a valid number rejected by the domain.

</details>
<details><summary>Hint 2</summary>

Use write! for each variant. source returns the inner parse error only for Parse; map_err chooses that variant.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

`Debug` is for developers and is usually derived; `Display` is the message for users and you write it by hand. `Error::source` exposes the lower-level cause, so a caller or logger can walk the chain without parsing strings, and keeping the `ParseIntError` as a value preserves its kind. Not every error has a source: `TooLarge` is a domain rule broken by a perfectly valid number, so it returns `None`. Typed variants let a caller `match` on `TooLarge(n)` to recover, for example by clamping, while still reporting `Parse` failures.

</details>

## 07 · Impossible errors and deliberate recovery

<details><summary>Hint 1</summary>

An expect needs a local invariant you can explain; a public arbitrary input rarely supplies one.

</details>
<details><summary>Hint 2</summary>

Parse a fixed known literal for the invariant. For batch recovery, retain successful values and count rejected inputs explicitly.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

`expect` is justified when failure would mean the program itself is wrong, not its input: parsing the literal "64" can only fail if someone edits the literal, so the message should say why it cannot fail. Writing `let _ = fallible();` silences the `#[must_use]` warning but does not make the operation succeed; it states that you decided the error does not matter. `let _ =` also drops the value immediately, unlike `let _name =`, which keeps it alive to the end of the scope. `recover_rows` deliberately recovers, but it reports what it discarded as an error count instead of hiding it.

</details>

## 08 · Capstone: fallible batch report

<details><summary>Hint 1</summary>

Use a typed enum for domain categories and carry the original one-based row number.

</details>
<details><summary>Hint 2</summary>

Parse each trimmed row, reject zero, and checked_add into u16. Convert each failure at its origin rather than losing the row number.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

`ReportError` has a variant per thing a caller might act on (fix the syntax, remove the zero, split the batch), and each carries the row that caused it. Rows are processed in order and the first failure returns immediately, so the caller sees the earliest problem and never a partial sum presented as success. Parsing, the zero check, and `checked_add` each produce their error at the point where the row number is known. A CLI `main` can return `Result<(), Box<dyn Error>>` for quick tools, or match the report and return an `ExitCode` with a stable message on stderr; either way the process exits with a non-zero status on failure.

</details>

## R01 · Spaced review of Chapters 3, 5 and 6

<details><summary>Hint 1</summary>

An early `return Ok(Some(i))` inside the loop means later lines are never parsed. Give the field the lifetime of `record`. `u8::try_from` returns a `Result`, so it composes with `map_err` and `collect`.

</details>
<details><summary>Hint 2</summary>

In A, `let v: i64 = line.trim().parse()?;` inside `for (i, line) in lines.iter().enumerate()`. In B, `record.split(',').nth(index)` gives an `Option<&str>`; on `None`, count the fields with `record.split(',').count()`. In C, `values.iter().enumerate().map(|(i, &v)| u8::try_from(v).map_err(|_| i)).collect()`.

</details>

## C01 · Code review

<details><summary>Hint 1</summary>

There are two bugs: one turns bad input into a panic, and one turns bad input into a success. The third function is correct but builds Strings nobody reads.

</details>
<details><summary>Hint 2</summary>

Use `line.split_once('=')` and reject a value that still contains '='; return a real error type such as an enum with `Malformed` and `Parse(ParseIntError)` variants (or at least propagate with `?` after `map_err`). `limit_or_default` is `value.map(|v| v.trim().parse()).transpose().map(|n| n.unwrap_or(10))`. In `tally`, `match` the parse result and count errors with an integer.

</details>
<details><summary>Check your review</summary>

`setting` calls `unwrap()` on user-controlled text, so an invalid number panics instead of returning `Err` as documented; it also collects a `Vec` for every line and reports errors as `String`s, which callers can only print, not inspect. A typed error (or `Box<dyn Error>`) keeps the `ParseIntError` as a source. `limit_or_default` swallows a present-but-invalid value and silently returns the default, which hides configuration mistakes; the nested `match` is a hand-written `transpose`. `tally` checks `is_ok()` and then unwraps, which clippy flags, and formats every error into a String only to count them. Ignoring an error should be a decision you can see, and it should never cost an allocation.

</details>

## Drills

Run `cargo drill ch07`. The compiler output explains *what* happened; these notes explain *why*. Read one only after the drill's verdict.

<details><summary>d01 · ? needs somewhere to return the error</summary>E0277: `?` returns the error early, but `print_double` returns `()`, which cannot hold a `ParseIntError`. Return `Result<(), ParseIntError>` and end with `Ok(())`, or handle the error locally with `match` or `unwrap_or`.</details>
<details><summary>d02 · ? converts errors only with From</summary>E0277: `?` calls `From::from` on the error, and there is no `From<io::Error>` for `ParseIntError`. Return `Result<u32, Box<dyn Error>>` and write `Ok(text.trim().parse()?)`, or define an enum with a variant for each error and implement `From` for both.</details>
<details><summary>d03 · An ignored Result</summary>Compiles and runs: an unused `Result` is only a `#[must_use]` *warning* (the drill runner hides warnings). The error still happened; nobody looked. In real code, handle it, propagate it with `?`, or write `let _ = save();` to say on purpose that it does not matter.</details>
<details><summary>d04 · unwrap on user input</summary>Panics at run time: 70000 does not fit in a `u16`, so `parse` returns `Err(PosOverflow)` and `unwrap` panics. The compiler cannot know what the text will contain. Decide a policy: `unwrap_or(1024)`, or return the error to the caller.</details>
<details><summary>d05 · Matching a Result</summary>E0004: `match` must be exhaustive, and `Err(_)` is not covered. Add an `Err` arm, or use `unwrap_or`, `if let`, or `?`.</details>
<details><summary>d06 · A &str is not a boxed error</summary>E0308: `Err("empty input")` has an error of type `&str`, but the function promises `Box<dyn Error>`. `?` would convert automatically; a plain `Err(...)` does not. Write `Err("empty input".into())`.</details>
<details><summary>d07 · What makes a type an Error</summary>E0277: `Error` requires both `Debug` and `Display`. Derive `Debug`, then implement `Display` by hand with the user-facing message (`write!(f, "quota exceeded: {} used", self.used)`).</details>
<details><summary>d08 · Returning a borrowed error</summary>E0308: matching on `&Result` binds `e` as `&ParseIntError`, but the function returns an owned `ParseIntError`. Clone it (`Err(e.clone())`), or better, take the `Result` by value so you can move the error out. `result.clone()` would replace the whole match.</details>
<details><summary>d09 · ? with a From impl</summary>Compiles and runs. `?` calls `From::from`, and the `impl From<ParseIntError> for AppError` makes the conversion exist, so `?` wraps the parse error in `AppError::BadNumber`. Compare with d02.</details>
