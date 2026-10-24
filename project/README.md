# Running project — `loglens`

[All chapters](../README.md) · [Hints](HINTS.md)

The chapter capstones practice one chapter at a time. This project is the other half: **one realistic program that grows from Chapter 3 to Chapter 24**, the kind of tool engineers write at work. Each milestone adds a feature with the chapter you just read, and several milestones *rewrite* earlier code with a better technique while the earlier tests stay green. By the end you have a fast, tested command-line log analyzer and the habit of reaching for Rust when a task at work needs one.

`loglens` reads structured log lines like this one:

```text
2026-10-23T17:03:00Z INFO auth user=ada latency_ms=42 msg="login ok"
```

and filters, counts, and summarizes them, across many files, in parallel, or while a service is still writing them:

```console
$ loglens grep --where 'level>=warn' fixtures/app.log fixtures/web.log
loglens: fixtures/app.log: line 9, column 1: invalid timestamp
2026-10-23T17:03:02Z WARN auth user=bob latency_ms=310 msg="slow password check"
2026-10-23T17:03:05Z ERROR auth user=bob latency_ms=97 msg="login failed: \"bad password\""
2026-10-23T17:03:21Z ERROR db latency_ms=5012 msg="connection reset"
2026-10-23T17:03:03Z WARN http method=POST path=/login status=401 latency_ms=330 user=bob
2026-10-23T17:03:07Z ERROR http method=GET path=/report status=500 latency_ms=5020 msg="upstream timeout"

$ loglens report --by user --top 3 --jobs 4 fixtures/app.log fixtures/web.log fixtures/worker.log
loglens: fixtures/app.log: line 9, column 1: invalid timestamp
loglens: fixtures/worker.log: line 4, column 77: unterminated quoted value
records: 19
levels: TRACE=1 DEBUG=1 INFO=11 WARN=3 ERROR=3
top 3 by user:
  -   count=10 errors=2 mean_latency_ms=3864.7 max_latency_ms=13000
  ada count=4 errors=0 mean_latency_ms=37.0 max_latency_ms=58
  bob count=3 errors=1 mean_latency_ms=245.7 max_latency_ms=330

$ loglens count -w 'target=http latency_ms>100' fixtures/web.log
3

$ loglens grep --follow -w 'level>=error' /var/log/myservice.log   # like tail -f
```

## How the project works

`project/` is a **standalone Cargo package**: it has its own `Cargo.toml`, `Cargo.lock`, and toolchain file, it is not part of the repository's root package, and it uses only the standard library. Root commands such as `cargo test --test chapter04` never build it. Run its commands with `--manifest-path project/Cargo.toml` from the repository root, or from inside `project/` without it. The examples below use the project directory:

```sh
cd project
cargo test --test m03_types              # one milestone
cargo test --test m03_types p_           # only its predictions
cargo test --test m03_types --test m04_borrowing --test m07_errors   # everything done so far
cargo test --doc                         # the documentation examples (from m08 on)
cargo run -- report fixtures/app.log     # the program (from m18 on)
cargo test --all-targets --no-run        # compile everything without running it
```

```text
project/
├── Cargo.toml, Cargo.lock, rust-toolchain.toml
├── README.md          this guide
├── HINTS.md           two progressive hints per milestone
├── fixtures/          sample logs used by the tests and by you
├── src/
│   ├── lib.rs         module tree, re-exports, prelude
│   ├── main.rs        the loglens binary (m18)
│   └── num.rs, scan.rs, error.rs, level.rs, record.rs, sink.rs, filter.rs,
│       pipeline.rs, report.rs, text.rs, follow.rs, cli.rs, parallel.rs,
│       aio.rs, macros.rs, fast.rs
└── tests/
    ├── common/mod.rs  shared helpers: guess(), allocation counting, fixtures
    └── mNN_name.rs    one test file per milestone
```

**The API is designed up front.** Every public type and signature already exists, with a doc comment that is the contract: what to return, which errors, which edge cases, and sometimes which technique to use. You replace `todo!()` bodies. Keep the public signatures (all test files compile against them, which is what lets you run milestones independently), but add private helpers, modules, and tests of your own freely. The starter compiles and its tests fail at `todo!()` and `guess()`; that is the starting state, not a broken setup. Until you finish, `src/lib.rs` silences unused-variable and unused-import warnings from the `todo!()` bodies; delete that attribute at the end and clean up what the compiler reports.

**Milestones build on each other.** m09's `parse_record` calls m04's `split_word`; m18's program calls almost everything. Do them in table order. m20 (async) and m23 (unsafe) are optional; nothing later depends on them.

**Each test file has three kinds of tests:**

- *Behavior tests* check the contract, including edge cases (empty input, Unicode, overflow, ties).
- *Technique tests* check how you solved it, where the contract says so. They count heap allocations (`common::count`), check that slices point into the original line (`common::borrows_from`), compare thread ids, or count polls. A clone-everything or sequential solution produces the right output and still fails them.
- *`p_predictions`*: before running anything, replace each `guess()` with your prediction. A wrong prediction is the most useful kind of feedback: write down why you were wrong.

The `m18_cli` and `m19_parallel` tests also run the **built binary** (`env!("CARGO_BIN_EXE_loglens")`) on the files in `fixtures/` and check stdout, stderr, and the exit status.

### The loop for each milestone

1. **Read** the chapters listed for the milestone.
2. **Predict.** Open `tests/mNN_*.rs`, read the header, and answer `p_predictions`.
3. **Implement** the tasks in the files listed below. Run the milestone's tests until they pass, in both profiles: `cargo test --test mNN_name` and `cargo test --release --test mNN_name`.
4. **Refactor** where the milestone says so, then re-run every earlier milestone. Run `cargo clippy --all-targets` and fix warnings in your code.
5. **Explain.** Answer the header's EXPLAIN prompt in your own notes, add one edge-case test, and only then open [HINTS.md](HINTS.md) if you were stuck.

### Refactoring with a safety net

At work you rarely write code once. You change it: for speed, for a new requirement, or because you learned a better tool. Tests are what make that safe. Several milestones below have a **Refactor** step that rewrites code you already finished. The rule is always the same:

1. Run the earlier milestones' tests and see them pass *before* you touch anything.
2. Change one thing.
3. Run the same tests again. If they pass, behavior is unchanged; if not, the failure tells you exactly what you broke. Undo or fix before going on.

The equivalence tests are deliberate: `split_field` must agree with `scan_field` (m07), `process_reader` with `process` (m18), the parallel results with the sequential ones (m19), `process_async` with `process` (m20), and `find_byte` with a plain search (m23).

## The log format

```text
TIMESTAMP LEVEL TARGET [KEY=VALUE]...
2026-10-23T17:03:00Z INFO auth user=ada latency_ms=42 msg="login ok"
```

- `TIMESTAMP` is exactly `YYYY-MM-DDTHH:MM:SSZ` (UTC). In this format, string order is time order.
- `LEVEL` is `TRACE`, `DEBUG`, `INFO`, `WARN` (or `WARNING`), or `ERROR`, in any case.
- `TARGET` is the component that wrote the line.
- A value is bare (`user=ada`) or in double quotes (`msg="login ok"`), where `\"`, `\\`, `\n`, `\r`, and `\t` are escapes.
- Blank lines and lines starting with `#` are ignored. Any other line that does not parse is *malformed*: it is reported with its line and column and skipped; it never stops processing.

**Filters** (m14) are terms separated by spaces, all of which must match: `level>=warn`, `target=auth`, `target~au` (contains), `user=ada`, `user!=bob`, `msg~timeout`, `latency_ms>100`, `ts>=2026-10-23T17:00:00Z`. The full grammar is in [`src/filter.rs`](src/filter.rs).

**The program** (m18) has three commands, `grep`, `count`, and `report`, and the options in `loglens --help` ([`src/cli.rs`](src/cli.rs)). Matching output goes to stdout; warnings and errors go to stderr. The exit status is 0 if a record matched, 1 if none did, and 2 on an error (bad arguments, an unreadable file, an invalid filter, or a malformed line under `--strict`), like `grep`.

## Milestones

| Done | Milestone | Read first | You write | Run |
| --- | --- | --- | --- | --- |
| [ ] | [m03 · Numbers with an overflow policy](#m03--numbers-with-an-overflow-policy) | Ch 3 | `num.rs` | `cargo test --test m03_types` |
| [ ] | [m04 · Zero-copy fields](#m04--zero-copy-fields) | Ch 4–5 | `scan.rs` | `cargo test --test m04_borrowing` |
| [ ] | [m07 · Errors with positions](#m07--errors-with-positions) | Ch 6–7 | `scan.rs`, `error.rs` | `cargo test --test m07_errors` |
| [ ] | [m08 · Modules, a public API, doc tests](#m08--modules-a-public-api-doc-tests) | Ch 8 | `error.rs`, doc comments | `cargo test --test m08_modules` |
| [ ] | [m09 · The Level enum and the Record struct](#m09--the-level-enum-and-the-record-struct) | Ch 9–10 | `level.rs`, `record.rs` | `cargo test --test m09_records` |
| [ ] | [m11 · Traits: Sink, Display, FromStr, TryFrom](#m11--traits-sink-display-fromstr-tryfrom) | Ch 11–13 | `level.rs`, `record.rs`, `sink.rs` | `cargo test --test m11_traits` |
| [ ] | [m14 · A filter language compiled to closures](#m14--a-filter-language-compiled-to-closures) | Ch 14 | `filter.rs` | `cargo test --test m14_filters` |
| [ ] | [m15 · Iterator pipelines](#m15--iterator-pipelines) | Ch 15 | `pipeline.rs` | `cargo test --test m15_pipelines` |
| [ ] | [m16 · Aggregation and top-k](#m16--aggregation-and-top-k) | Ch 12, 16 | `report.rs` | `cargo test --test m16_reports` |
| [ ] | [m17 · Quoted values and Unicode](#m17--quoted-values-and-unicode) | Ch 17 | `text.rs`, `scan.rs`, `record.rs`, `filter.rs` | `cargo test --test m17_text` |
| [ ] | [m18 · Streaming input and the CLI](#m18--streaming-input-and-the-cli) | Ch 18 | `pipeline.rs`, `follow.rs`, `cli.rs`, `main.rs` | `cargo test --test m18_io --test m18_cli` |
| [ ] | [m19 · Parallel files, identical results](#m19--parallel-files-identical-results) | Ch 19 | `parallel.rs`, `cli.rs` | `cargo test --test m19_parallel` |
| [ ] | [m20 · Optional: a std-only async reader](#m20--optional-a-std-only-async-reader) | Ch 20–21 | `aio.rs` | `cargo test --test m20_async` |
| [ ] | [m22 · Macros: field! and log_line!](#m22--macros-field-and-log_line) | Ch 22 | `macros.rs` | `cargo test --test m22_macros` |
| [ ] | [m23 · Optional: an unsafe fast path](#m23--optional-an-unsafe-fast-path) | Ch 23–24 | `fast.rs` | `cargo test --test m23_unsafe` |

Each section names the tasks by the letters used in the source (`m07 A`, `m07 B`, ...); search for them.

### m03 · Numbers with an overflow policy

**Goal.** Read numeric fields such as `latency_ms=42` without ever wrapping or panicking, and make the overflow behavior a visible choice.

**Tasks** in [`src/num.rs`](src/num.rs):

- A. `parse_u64(text, policy)`: digits only, the byte index of the first bad byte, and `Overflow::Reject` or `Overflow::Saturate` for values above `u64::MAX`. Accumulate with `checked_mul`/`checked_add`; do not call `str::parse`.
- B. `mean(total, count)`: integer-to-float conversion with `as`.

**Run** `cargo test --test m03_types`. **Done when** both profiles pass and you can say which `u64` values `as f64` rounds.

### m04 · Zero-copy fields

**Goal.** Split a line into words and `key=value` fields without copying a single byte: every result is a `&str` into the line.

**Tasks** in [`src/scan.rs`](src/scan.rs): A. `split_word`; B. `split_field` (bare and quoted values, with `\"` not closing a quote); C. `find_field`, whose signature promises a value that outlives the vector of fields.

**Run** `cargo test --test m04_borrowing`. The technique tests compare pointer ranges: a result that is equal but copied fails. **Done when** you can draw the line, the `Field` structs, and the arrows between them.

### m07 · Errors with positions

**Goal.** Replace "it didn't parse" (`None`) with "line 3, column 17: unterminated quoted value".

**Tasks:** in [`src/scan.rs`](src/scan.rs), A. `scan_field` (an `ErrorKind` per problem), B. `column_of` (characters, not bytes), C. `parse_fields` (use `?` and `map_err`); in [`src/error.rs`](src/error.rs), D. `Display` for `ErrorKind` and E. `Display` for `ParseError`.

**Refactor.** `split_field` and `scan_field` now contain the same scanner twice. Make `split_field` the one-liner `scan_field(s).ok()` and delete the duplicate. Run `cargo test --test m04_borrowing --test m07_errors`; the m04 tests, plus `a_split_field_agrees_with_scan_field`, prove nothing changed.

**Run** `cargo test --test m07_errors`.

### m08 · Modules, a public API, doc tests

**Goal.** Treat the crate as a library someone else uses: one error type for everything, documented paths, and examples that are tested.

**Tasks:** in [`src/error.rs`](src/error.rs), A. `Display` for `Error`, B. `source` (so error chains can be walked), C. the `From` conversions that let `?` turn an `io::Error`, `ParseError`, or `FilterError` into `loglens::Error`. Then read [`src/lib.rs`](src/lib.rs): the module tree, the `pub use` re-exports, and the `prelude`.

**Doc tests.** Run `cargo test --doc -- num:: scan::`: the examples on `parse_u64`, `split_word`, and `scan_field` must pass. Add a runnable example (a fenced code block in the `///` comment) to two more functions you wrote, and run `cargo doc --no-deps --open` to read your crate as a user would. The crate-level example in `lib.rs` needs m09 and m14; run plain `cargo test --doc` again after m14.

**Run** `cargo test --test m08_modules`.

### m09 · The Level enum and the Record struct

**Goal.** Turn a line into a typed value: a `Level` enum whose derived ordering means severity, and a `Record<'a>` struct that borrows everything from its line.

**Tasks:** in [`src/level.rs`](src/level.rs), A. `Level::from_name` (a `match` with guards), B. `as_str`; in [`src/record.rs`](src/record.rs), C. `is_timestamp` (one slice pattern), D. `parse_record` (`let ... else`, `?`, and the column table in its contract), E. `Record::get`, F. `Record::number` (`Option` and `Result` combined with `transpose`).

**Run** `cargo test --test m09_records`. **Done when** you can explain the `'a` in `Record<'a>` and what an owned record would cost per line.

### m11 · Traits: Sink, Display, FromStr, TryFrom

**Goal.** Give the types the standard traits users expect, and define the crate's own trait: a [`Sink`](src/sink.rs) receives matching records.

**Tasks:** in [`src/level.rs`](src/level.rs), A. `FromStr`, B. `Display` that honors `{:<5}`, C. `TryFrom<u8>` and `From<Level> for u8`; in [`src/record.rs`](src/record.rs), D. `Display` for `Record` (the canonical line, written without allocating); in [`src/sink.rs`](src/sink.rs), E–I. the sinks: `Vec<String>`, `Count`, `LineWriter`, `Tee`, and the blanket impls for `&mut S` and `Box<S>`.

**Run** `cargo test --test m11_traits`. The allocation tests fail if `Display` or `LineWriter` builds a `String` per record.

### m14 · A filter language compiled to closures

**Goal.** `level>=warn user!=bob latency_ms>100` is parsed **once** into a list of boxed closures, then evaluated for millions of records without parsing or allocating.

**Tasks** in [`src/filter.rs`](src/filter.rs): A. `Filter::parse` (find the operator, validate the value, build a `move` closure per term, with the exact error reasons in the contract); B. `Filter::matches`.

**Run** `cargo test --test m14_filters`. Technique tests: the filter must be `Send + Sync + 'static` (owning its values) and matching must not allocate. After this milestone, `cargo test --doc` should pass completely.

### m15 · Iterator pipelines

**Goal.** Process a whole text as a lazy pipeline: lines → records → matching records → a sink, with line numbers and error reports.

**Tasks** in [`src/pipeline.rs`](src/pipeline.rs): A. `Iterator for Records` (skip ignored lines, number the errors); B. `AddAssign for Tally`; C. `matching` (adapters only, no `collect`); D. `slowest` (the *first* maximum, which `max_by_key` does not give you); E. `process` (count, filter, forward to the sink, report errors through a closure).

**Refactor.** Rewrite the loops you wrote in m04, m07, and m09 (`find_field`, `parse_fields`) with iterator adapters or `std::iter::from_fn` where it reads better, and keep loops where it does not. `cargo test --test m04_borrowing --test m07_errors --test m09_records` must stay green.

**Run** `cargo test --test m15_pipelines`.

### m16 · Aggregation and top-k

**Goal.** Turn records into a report: counts per level, per-group counts and latency statistics, the top groups, and merging partial results (which m19 needs).

**Tasks** in [`src/report.rs`](src/report.rs): A. `Group::key_of`; B. `From<&str> for Group`; C. `GroupStats::mean_latency`; D. `+=` for `GroupStats` (Chapter 12); E. `Summary::add` (allocate a group name only the first time it appears); F. `Summary::top` (a `BinaryHeap` that never holds more than k entries); G. `+=` for `Summary`; H. `Sink for Summary`; I. `render`, the exact text shown by `loglens report`.

**Refactor.** Anywhere you aggregate by hand (for instance in a test of your own), pass a `Summary` to `process` instead: it is a `Sink`.

**Run** `cargo test --test m16_reports`. The allocation test fails if `add` calls `to_string` for every record.

### m17 · Quoted values and Unicode

**Goal.** Decode quoted values correctly, quote values for output, and shorten text without splitting characters, borrowing whenever nothing changes (`Cow<str>`).

**Tasks** in [`src/text.rs`](src/text.rs): A. `unescape`, B. `quote`, E. `truncate`; in [`src/scan.rs`](src/scan.rs), C. `Field::text`; in [`src/record.rs`](src/record.rs), D. `Record::text`.

**Refactor.** m14's filter compares `=`, `!=`, and `~` against the raw value, so `path=C:\temp` does not match `path="C:\\temp"`. Switch those comparisons to `record.text(key)`. Run `cargo test --test m14_filters` (including its zero-allocation check, which `Cow::Borrowed` keeps passing) and `cargo test --test m17_text`.

**Run** `cargo test --test m17_text`.

### m18 · Streaming input and the CLI

**Goal.** Make it a real program: read files or stdin line by line without loading them into memory, follow a growing file, parse arguments by hand, and use stdout, stderr, and the exit status like a well-behaved Unix tool.

**Part 1, `m18_io`:** in [`src/pipeline.rs`](src/pipeline.rs), A. `process_reader` (one reused buffer, `read_until`, invalid UTF-8 as a malformed line); in [`src/follow.rs`](src/follow.rs), B. `Follower::poll` (complete lines only; keep a partial line for the next poll) and C. `Follower::finish`.

**Refactor.** `process` and `process_reader` now do the same per-line work. Extract one private helper that both call (m20 will reuse it). Run `cargo test --test m15_pipelines --test m18_io`; the equivalence test `a_process_reader_equals_process` checks every line of both.

**Part 2, `m18_cli`:** in [`src/cli.rs`](src/cli.rs), D. `parse_args` (by hand, no crates; file names stay `OsString`), E. `run`, F. `main_with`; in [`src/main.rs`](src/main.rs), G. `main` (lock the streams, buffer stdout, return the `ExitCode`). These tests run both in memory and through the built binary on `fixtures/`, including a `--follow` test that appends to a file while the program runs.

**Run** `cargo test --test m18_io --test m18_cli`, then try it: `cargo run -- report --by user fixtures/app.log`, `cat fixtures/web.log | cargo run -- grep -w status=500`, and `cargo run -- grep fixtures/app.log | head -1` (a closed pipe must not print an error).

### m19 · Parallel files, identical results

**Goal.** `loglens report --jobs 8 *.log` uses every core, and its output is byte-for-byte the output of `--jobs 1`.

**Tasks** in [`src/parallel.rs`](src/parallel.rs): A. `map_ordered` (scoped threads, dynamic work distribution through an atomic index, results sent back over a channel and put in input order; no threads at all for one worker); B. `summarize_files` (per-file summaries merged in path order with m16's `+=`).

**Refactor** `run` in [`src/cli.rs`](src/cli.rs): with `--jobs` other than 1 and several files, process the files with `map_ordered`, buffering each file's stdout and stderr, then write them in order. `cargo test --test m18_cli` must stay green, and `c_cli_output_is_identical_for_every_jobs` compares every command's output, messages, and status across `--jobs` values.

**Run** `cargo test --test m19_parallel`. Technique tests check that several threads really run at once, that a slow file does not stall the others, and that one worker spawns nothing.

### m20 · Optional: a std-only async reader

**Goal.** See what an async runtime does by writing the smallest one: a future that reads one line from a source that is often "not ready", and an executor that parks the thread until it is woken.

**Tasks** in [`src/aio.rs`](src/aio.rs): A. `Future for NextLine`; B. `block_on` (a `Waker` built from `std::task::Wake` that unparks the thread); C. `process_async`, which reuses your m18 per-line helper. `Trickle`, the simulated source, is provided; read its `poll_read` first.

**Run** `cargo test --test m20_async`. A busy-polling executor fails `b_block_on_sleeps_until_woken`. For real programs use an async runtime such as Tokio (see the [Chapter 20](../exercises/chapter20/README.md) labs); this milestone is about understanding it.

### m22 · Macros: field! and log_line!

**Goal.** Remove repetition at call sites with two `macro_rules!` macros: `field!(record, user)` / `field!(record, latency_ms as u64)`, and `log_line!(ts, Info, "auth", user = "ada")`, which builds a correctly quoted line.

**Tasks** in [`src/macros.rs`](src/macros.rs): A. the expansions of `field!`; B. the expansion of `log_line!`. The patterns are given (so the tests compile); the expansions must use `$crate::` paths, evaluate each argument once, and be hygienic, which the tests check with a caller that defines its own `quote`, `text`, and `Level`.

**Refactor (optional).** Build the log lines of your own new tests with `log_line!`; it quotes values correctly, which hand-written test lines often do not.

**Run** `cargo test --test m22_macros`.

### m23 · Optional: an unsafe fast path

**Goal.** Make byte search faster with unsafe code that no caller can misuse: an eight-bytes-at-a-time search (SWAR) with unaligned reads, and an unchecked split whose safety you prove in a comment.

**Tasks** in [`src/fast.rs`](src/fast.rs): A. `find_byte`; B. `split_ascii`. Write each `// SAFETY:` comment *before* running the code.

**Refactor (optional).** Use `find_byte` in `scan.rs` to look for `=`, `"`, and `\`. Every milestone must stay green: `cargo test --tests` (with m20 and m23 done, or skip the unfinished ones).

**Miri.** Tests show that the code works on the inputs they try; Miri, an interpreter that detects undefined behavior such as out-of-bounds or misaligned reads, shows that those executions were sound:

```sh
rustup toolchain install nightly --component miri
cargo +nightly miri test --manifest-path project/Cargo.toml --test m23_unsafe
```

Miri interprets every instruction, so this takes a few minutes.

Neither proves soundness for every input; your written argument does. **Chapter 24 extension (manual):** declare C's `memchr` in an `unsafe extern "C"` block, wrap it in a safe function, and compare it with `find_byte` in a scratch test. Keep it out of the committed starter, since it depends on the platform's C library.

**Run** `cargo test --test m23_unsafe`.

## After the last milestone

- Delete the `#![allow(unused_variables, unused_imports)]` line in `src/lib.rs` and fix every warning; then run `cargo clippy --all-targets` and the whole suite in both profiles: `cargo test --tests && cargo test --release --tests`.
- Point it at real logs. Most services can write `key=value` lines; for others, write a converter, or change the parser (with the tests as your safety net).
- Change a requirement, predict what breaks, then make it pass: fractional seconds in timestamps (careful: string order is no longer time order), a p95 latency in the report, `--since 10m`, JSON lines, or log rotation in `--follow`.
- Redo one milestone from its contract after a break, without looking at your old code.

## Validation (for maintainers)

The starter must compile and stay formatted; the commands run from the repository root:

```sh
cargo test --manifest-path project/Cargo.toml --locked --offline --all-targets --no-run
cargo test --manifest-path project/Cargo.toml --locked --offline --release --all-targets --no-run
cargo fmt --manifest-path project/Cargo.toml --all --check
cargo clippy --manifest-path project/Cargo.toml --locked --offline --all-targets
```

As for the chapters, no reference solution is shipped. A reference solution was written in a scratch copy and passes every milestone in both profiles; when you change a contract or a test, repeat that check (see [AUTHORING.md](../AUTHORING.md)).
