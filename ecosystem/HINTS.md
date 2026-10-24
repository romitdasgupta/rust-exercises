# Ecosystem track hints

[Track guide](README.md) · [Tooling workday](TOOLING.md)

Try the first hint before opening the second. Open *Check your explanation* only after writing your own answer to the lesson's EXPLAIN prompt.

## E01 · Library and application errors

<details><summary>Hint 1</summary>

The `#[error("...")]` string can name fields: `{line}` for named fields, `{0}` for tuple fields. thiserror makes a field the error's source when it is named `source`, or marked `#[source]` or `#[from]`. `#[from]` also generates `From<io::Error>`, which is what `?` needs inside `parse_settings`. In `load_settings`, `use anyhow::Context;` and call `.with_context(|| format!(...))` on each `Result`.

</details>
<details><summary>Hint 2</summary>

Iterate `reader.lines().enumerate()`, `let line = line?;`, skip blanks and comments, then `split_once('=')` with `.ok_or(SettingsError::MissingEquals { line: index + 1 })?`. Map a parse failure with `map_err(|cause| SettingsError::BadPort { line, cause })`. Keep `Option`s for host and port and convert them at the end with `ok_or(MissingKey(..))?`. For `exit_code`, loop over `err.chain()`; use `cause.is::<SettingsError>()` and `cause.downcast_ref::<io::Error>()` and return on the first match.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

A library's callers need to *decide* what to do. An enum with one variant per decision lets them match: retry on `Io`, tell the user to fix line 3 on `MissingEquals`. An application usually only needs to *report*, so `anyhow::Result` with context at each layer gives a readable chain ("parsing app.conf: line 2: invalid port: invalid digit found in string") without a type per layer. Each Display message describes only its own layer. If it also included the source's message, a reporter that prints the chain (`{:#}`, anyhow's Debug, or `tracing`) would print the cause twice. `downcast_ref` and `chain()` still give the application typed access when it needs a decision, such as an exit code.

</details>

## E02 · serde

<details><summary>Hint 1</summary>

Container attributes go on the struct or enum: `#[serde(rename_all = "camelCase", deny_unknown_fields)]`, and `#[serde(tag = "kind", rename_all = "snake_case")]`. Field attributes: `#[serde(default = "default_port")]`, `#[serde(default, skip_serializing_if = "Vec::is_empty")]`, `#[serde(skip)]`. `&'a str` borrows automatically. `Cow<'a, str>` borrows only with `#[serde(borrow)]`; without it, serde always builds an owned value.

</details>
<details><summary>Hint 2</summary>

`count_levels` loops over the lines, calls `parse_log_line`, and uses `parsed.level` (a `&str` into `input`) as the map key. For `PortRange`, serialize with `serializer.collect_str(&format_args!("{}-{}", ..))`. To deserialize, write a `Visitor` whose `visit_str(self, text: &str)` handles both borrowed and escaped strings. Parse with `split_once('-')` and `parse::<u16>()`, build with `PortRange::new`, and report errors with `E::custom(...)`. Call it with `deserializer.deserialize_str(YourVisitor)`. A shorter alternative is `#[serde(try_from = "String", into = "String")]` with `TryFrom<String>` and `From<PortRange> for String` impls.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

A `&'a str` must point into the input. A JSON string with an escape (`\"`, `8`) does not exist in decoded form anywhere in the input, so the deserializer has nothing to lend and fails. A `Cow<'a, str>` with `#[serde(borrow)]` borrows when the bytes can be used as they are, and allocates a decoded `Owned` copy otherwise. That is why it never fails on valid JSON. Hand-write `Deserialize` when you need control over *which* input shapes are accepted (several formats, zero-copy, custom error messages per shape). `try_from` is enough when the rule is "deserialize this simpler type, then validate". Both keep a derived impl from building a `PortRange { start: 9, end: 1 }` without calling your constructor.

</details>

## E03 · clap

<details><summary>Hint 1</summary>

Field attributes: `#[arg(short, long, action = clap::ArgAction::Count, global = true)]` for `-v`, `#[arg(long, default_value_t = 5, value_parser = clap::value_parser!(u8).range(1..=10))]` for priority, `#[arg(long = "tag")]` on a `Vec<String>` for a repeatable flag, `#[arg(long, value_enum, default_value_t = Format::Table)]`, and `#[arg(required = true)]` on `ids`. A plain `Vec<u32>` positional means "zero or more".

</details>
<details><summary>Hint 2</summary>

`parse_duration`: find the first non-digit (`text.find(|c: char| !c.is_ascii_digit())`), `split_at` there, parse the digits as u64 (empty digits fail the parse → "invalid number"), and `match` the unit. In `run`, `match Cli::try_parse_from(args)`: on `Err(err)`, pick the stream with `err.use_stderr()`, `write!(stream, "{err}")`, and return `err.exit_code()`. A `&mut dyn Write` variable can hold either stream.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

`try_parse_from` tokenizes argv, matches `submit`, and runs each value through its value parser. The ranged `u8` parser rejects 11 and returns a `clap::Error` of kind `ValueValidation`. Its message names `--priority` and the allowed range, and `exit_code()` is 2, the usage-error convention. `run` writes it to stderr because `use_stderr()` is true (for `--help` it is false, and the exit code is 0). Rejecting in the parser means the rest of the program never sees an invalid `Cli`. The check runs once, the error message names the flag, and `--help` can show the constraint. A check later in `run` would duplicate all of that by hand.

</details>

## E04 · tracing

<details><summary>Hint 1</summary>

`Capture` clones share an `Arc<Mutex<Vec<u8>>>`. `io::Write::write` locks, `extend_from_slice`s, and returns `Ok(bytes.len())`. `make_writer` returns `self.clone()`. For `capture`, build `tracing_subscriber::fmt()` with `.with_writer(sink.clone())`, `.with_ansi(false)`, `.without_time()`, `.with_target(false)`, `.with_max_level(max_level)`, then `.finish()`, and run `f` inside `tracing::subscriber::with_default(subscriber, f)`.

</details>
<details><summary>Hint 2</summary>

`#[tracing::instrument(name = "login", skip_all, fields(user = %username))]`: `skip_all` records no arguments, and `fields` adds back exactly what you want, with Display (`%`). For the batch: `let span = tracing::info_span!("batch", id, total = tracing::field::Empty); let _guard = span.enter();`. Emit `tracing::warn!(item, "skipping negative item")`, and at the end `span.record("total", total)` before `info!("batch done")`. Holding `enter()`'s guard is fine here because the function is synchronous.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

An *event* is a moment ("login failed"). A *span* is a period of time with context (`login{user=alice}`); every event inside it inherits that context, which is what makes logs from concurrent work readable. Emitting code describes what happens. The *subscriber* decides what is recorded, how it is formatted, and where it goes. Level filtering happens before the arguments are evaluated, so disabled events cost almost nothing. A library must not install a global subscriber: there is only one per process, the application owns that choice, and a second `set_global_default` fails. Tests use `with_default` for the same reason: it is scoped to one thread and one closure.

</details>

## E05 · Property-based testing

<details><summary>Hint 1</summary>

`arb_data`: combine generators with `prop_oneof![Just(Vec::new()), vec(any::<u8>(), 0..64), runs]`. Build `runs` from `vec((any::<u8>(), 1usize..700), 1..5)` and `prop_map` it into the concatenated bytes. End with `.boxed()`. Properties return `Result<(), TestCaseError>`: use `prop_assert_eq!` and `prop_assert!`, then `Ok(())`. For `check_canonical`, `runs.windows(2)` gives you each adjacent pair.

</details>
<details><summary>Hint 2</summary>

Canonical means: all counts `>= 1`, the counts (as `usize`) sum to `data.len()`, and for each adjacent pair `(c1, b1), (c2, b2)`, `b1 != b2 || c1 == 255`. `arb_ops`: `prop_oneof![3 => any::<u8>().prop_map(Op::Push), 1 => Just(Op::Pop)]` inside `vec(.., 0..64)`, paired with `1usize..=8`. The model is a `VecDeque`: on push, `pop_front` first if it is at capacity; on pop, compare `ring.pop()` with `model.pop_front()`; after every op, compare lengths.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

Each generated value is a *value tree* that knows how to simplify itself: vectors drop elements, numbers move toward zero with a binary search. After a failure, proptest repeatedly tries simpler versions, keeps any that still fail, and stops when nothing simpler fails. The result is a minimal counterexample, such as `[0]` for a lost last run or 256 equal bytes for a missing split. `SplitsEarly` produces runs that still decode correctly, so a round-trip check is satisfied. Only the canonical-form property states the extra contract that the mutant breaks. A model-based test compares the implementation with something too simple to be wrong (`VecDeque` plus one `if`). If the model is as complex as the code, a disagreement doesn't tell you which one is buggy.

</details>

## E06 · tokio

<details><summary>Hint 1</summary>

`run_limited`: before each `set.spawn(work(id))`, `while set.len() >= limit` await `set.join_next_with_id()`. Store `handle.id() → id` in a `HashMap`. On `Err(err)`, look up `err.id()`. `try_enqueue` maps `TrySendError::Full(job)` / `Closed(job)`. `enqueue_within`: `time::timeout(patience, tx.reserve())`. `reserve` does not take the job, so you still own it on timeout. On success, `permit.send(job)`.

</details>
<details><summary>Hint 2</summary>

`batch`: create the deadline once, `let deadline = time::sleep(Duration::ZERO); tokio::pin!(deadline);`, and `deadline.as_mut().reset(Instant::now() + max_wait)` only when an item starts a new batch. Loop over `tokio::select! { biased; _ = &mut shutdown => break, item = rx.recv() => ..., () = &mut deadline, if !current.is_empty() => flush }`. After the loop, flush any leftover items. `checksum_all`: wrap the function in an `Arc`, spawn one `spawn_blocking(move || f(&input))` per input *before* awaiting any of them, then await the handles in order.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

A future is cancellation safe when dropping it before completion loses nothing. `rx.recv()` is: if another branch wins, no message has been taken. `timeout(.., tx.send(job))` is not: `job` moved into the `send` future, so on timeout it is dropped with it. `reserve()` waits for capacity without taking the job, so the caller still owns it on every path. For the same reason, the batcher's buffer and deadline live outside `select!`: the branch futures are rebuilt on each iteration, and a `sleep` created inside the loop would restart on every item. CPU-heavy work in an async task blocks the worker thread, and with it every other task scheduled there. `spawn_blocking` moves it to a thread pool built for blocking.

</details>

## E07 · Capstone: jobq

<details><summary>Hint 1</summary>

Config: the same attributes as E02 (`deny_unknown_fields` on all three types, `default = "..."`, `tag = "type"`). Errors: like E01, `#[from]` on `Io` and `Json`. Validation runs after `serde_json::from_str`, in the documented order; use a `HashSet<&str>` to find duplicate ids. `execute` adds `.with_context(|| format!("loading {}", path.display()))`. `error_exit_code` walks `err.chain()` twice: first for `NotFound`, then for `ConfigError`.

</details>
<details><summary>Hint 2</summary>

Pool: `let (tx, rx) = mpsc::channel(queue_capacity); let rx = Arc::new(tokio::sync::Mutex::new(rx));`. Spawn `workers` tasks on a `JoinSet`. Each loops on `let next = rx.lock().await.recv().await;`, so the guard is dropped at the end of that statement. Run each job as `run_one(job).instrument(tracing::warn_span!("job", id = %job.id))`, wrapping the task in `time::timeout`. Send `(index, job)` from the main task, `drop(tx)` so the workers' `recv` returns `None`, collect with `join_all`, and sort by index. In `main`: `Cli::parse()`, `tracing_subscriber::fmt().with_writer(std::io::stderr)...init()`, a `new_current_thread().enable_time()` runtime, and `ExitCode::from(code as u8)`.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

clap turns argv into a typed `Cli` or exits with code 2 before any of your code runs. `load_config` reads the file (`io::Error` → `ConfigError::Io` via `#[from]`), parses with serde (`ConfigError::Json`), and validates (`ConfigError::Invalid`). That is the library layer, with typed errors. `execute` is the application layer: it adds human context ("loading jobs.json") with anyhow and has no new error types. The worker pool is pure tokio, and tracing spans attach the job id to everything a job logs. Jobs don't return errors to the caller; they become data (`Outcome`), serialized by serde into the report. Finally, `main` turns the result into an exit code. `error_exit_code` recovers the typed cause from the anyhow chain, so 66 vs 78 is decided by the library's types, not by parsing messages.

</details>

## E08 · Tooling workday

<details><summary>Hint 1</summary>

`parse_size`: split at the first non-digit like E03's `parse_duration`. Map the unit to a shift (`""` → 0, `KiB` → 10, `MiB` → 20, `GiB` → 30), parse the digits as u64 (a parse error on all digits means overflow), then `checked_mul(1 << shift)`. `double!`: change the matcher to `($e:expr)` and the body to `$e * 2`. `summarize`: `to_vec()`, `sort()`, and index `(len - 1) / 2`. `measure`: loop with `black_box(f())`, timing each sample with `Instant::now()` and `elapsed()`.

</details>
<details><summary>Hint 2</summary>

For folded stacks, parse each non-blank line with `rsplit_once(' ')` into the stack and the count, and `split(';')` the stack into frames. Inclusive: sum the counts of stacks that `contains` the frame (once per line, so recursion is not double-counted). Self: sum the counts where the frame is `last()`. `hottest_self`: total self counts per leaf in a `BTreeMap` (sorted by name), then pick the maximum count. For ties, keep the first: for example, iterate in reverse and use `max_by_key`, which returns the last maximum.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

In a flamegraph, the x-axis is sample count, not time order. A box's width is the share of samples whose stack contains that frame (inclusive cost). The frames above it are its callees, and the uncovered top edge is self time. Wide boxes with deep stacks above them mean the cost is in the callees; wide flat tops are where the CPU actually spends its time. One `Instant` measurement is not a benchmark because (1) the first run includes warm-up effects (cold caches, page faults, lazy initialization), (2) noise from frequency scaling and other processes varies between runs, and (3) the optimizer may delete or hoist work whose result is unused unless `black_box` hides it. Debug builds also measure a different program.

</details>
<details><summary>Answers: TOOLING.md task 2 (cargo tree)</summary>

1. `syn` appears twice: 2.x through `tracing-attributes` (from `tracing`), and 3.x through `serde_derive`, `clap_derive`, `thiserror-impl`, and `tokio-macros`. Both compile on a clean build, which adds build time but nothing to the binary: proc-macro dependencies run only at compile time. You can't fix it locally; it goes away when tracing-attributes moves to syn 3.
2. `cargo build`: `macros` (which enables `tokio-macros`), `rt`, `sync`, and `time`. `cargo test` adds `test-util`, which comes from the `[dev-dependencies]` entry. With edition 2024 (resolver 3), dev-dependency features are enabled only when building targets that need them.
3. `proptest`, which is a dev-dependency. It is not in `jobq`.
4. `fmt` and `std`. The defaults would add `ansi` (terminal colors), `smallvec`, and `tracing-log` (collecting `log` records).

</details>
<details><summary>Answers: TOOLING.md task 7 (flamegraph)</summary>

1. `serve` (375 of 380 samples); `log` is 65/380 ≈ 17%.
2. `format` has 130 self samples: 90 under `render` plus 40 under `log`. A flamegraph shows two separate boxes, so add up a function's self time across call sites before you rank it. Profilers' "inverted" or "bottom-up" views do this for you.
3. `parse_json` could save up to 150 samples (120 self + 30 in the `alloc` it calls); `format` up to 130. Then ask which one is cheaper to fix: a faster parser, or avoiding formatting in two places (for example, by caching a rendered fragment or logging fewer fields).

</details>
