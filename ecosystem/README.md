# Production Rust: the ecosystem track

[All chapters](../README.md) · [Hints](HINTS.md) · [Tooling workday](TOOLING.md)

The chapter exercises use only the standard library, on purpose. Professional Rust code also uses a small set of crates and tools almost everywhere. This track gives you guided, tested practice with them: serde, thiserror, anyhow, clap, tracing, tokio, proptest, and the Cargo tooling around them. The aim is that Rust feels as practical at work as it does in the book.

Each lesson tells you which chapter to finish first. Each lesson also ends with **judgment**: why this crate, what the alternatives are, and when you would choose differently. Knowing the API is half of the work; choosing well is the other half.

## Setup

This is a standalone Cargo package with its own `Cargo.lock` and pinned versions. It is **not** part of the root package, so the chapter checks never download or build these crates. Rust 1.99.0 is pinned in `ecosystem/rust-toolchain.toml`, identical to the root one. Run commands from the repository root:

```sh
# The first build downloads the pinned crates; afterwards you can add --offline.
cargo test --manifest-path ecosystem/Cargo.toml --locked --all-targets --no-run
cargo test --manifest-path ecosystem/Cargo.toml --locked --release --all-targets --no-run
```

The starters compile; tests fail at `todo!()` and `guess()` until you fill them in. That is the starting state, not a broken setup.

## Work through the track

Same loop as the chapters:

1. **Read** the lesson file's header: what to predict, what to implement, what to explain, and the judgment notes.
2. **Predict.** Answer the `p_predictions` test first: replace each `guess()` with a value. The prediction tests don't need your implementation.
3. **Implement** the `todo!()` bodies from their contracts. Some tasks are attributes on supplied types (serde, clap, thiserror, `#[instrument]`). Run only that lesson, for example `cargo test --manifest-path ecosystem/Cargo.toml e01_`.
4. **Check technique.** Some tests check *how*: borrowed pointers in E02, the job handed back on timeout in E06, mutants your properties must catch in E05. Then run clippy and fix warnings in your code.
5. **Explain** the EXPLAIN prompt in your notes, add one edge-case test, then compare with *Check your explanation* in [HINTS.md](HINTS.md).

| Done | Lesson | Start after | Crates |
| --- | --- | --- | --- |
| [ ] | [E01 · Library and application errors](src/e01_errors.rs) | Chapter 7 | thiserror, anyhow |
| [ ] | [E02 · serde: derive, attributes, borrowing, validation](src/e02_serde.rs) | Chapters 11 and 13 | serde, serde_json |
| [ ] | [E03 · Command-line interfaces](src/e03_clap.rs) | Chapter 18 | clap |
| [ ] | [E04 · Structured logging](src/e04_tracing.rs) | Chapter 18 | tracing, tracing-subscriber |
| [ ] | [E05 · Property-based testing](src/e05_proptest.rs) | Chapters 15 and 16 | proptest |
| [ ] | [E06 · Async services](src/e06_tokio.rs) | Chapters 20 and 21 | tokio |
| [ ] | [E07 · Capstone: `jobq` job runner](src/e07_capstone.rs) | E01–E06 | all of the above |
| [ ] | [E08 · Tooling workday](TOOLING.md) ([code](src/e08_tooling.rs)) | E01; tasks 6–7 after Chapter 15 | cargo, clippy, rustdoc |

E01–E06 are independent of each other; follow the book. E07 builds on all of them, and its tracing test also uses your E04 `capture`. E08 can be done in parts whenever you have a free afternoon.

```sh
cargo test --manifest-path ecosystem/Cargo.toml -- --list
cargo test --manifest-path ecosystem/Cargo.toml e01_
cargo test --manifest-path ecosystem/Cargo.toml --release e01_
cargo test --manifest-path ecosystem/Cargo.toml --test e07_capstone --test e07_tracing
cargo run --manifest-path ecosystem/Cargo.toml --bin jobq -- -v run ecosystem/jobs.example.json
cargo test --manifest-path ecosystem/Cargo.toml --doc
cargo clippy --manifest-path ecosystem/Cargo.toml --all-targets
cargo fmt --manifest-path ecosystem/Cargo.toml --all --check
```

Lesson filters (`e01_` … `e08_`) match the module path of the unit tests. A filtered run also starts the integration-test binaries, which then report 0 tests. That is expected.

## The lessons

### E01 · Library errors with thiserror, application errors with anyhow

**Teaches:** an error enum whose variants callers can match; Display messages that don't repeat their source; `#[source]` and `#[from]`; `?` conversions; anyhow's `context`, `{:#}` chains, and `downcast_ref`; mapping errors to exit codes by walking `chain()`.

**Judgment:** A *library* returns an error type callers can inspect, because they may recover differently from `MissingKey` than from `Io`. An *application* mostly reports errors to a human, so one opaque type with context at each layer is better. thiserror writes exactly the `Display`/`Error`/`From` impls you would write by hand, so it adds nothing to your public API. Never put `anyhow::Error` in a public library signature. Alternatives: hand-written impls (no dependency; fine for two or three variants), [snafu](https://crates.io/crates/snafu) (context selectors, good for large libraries), [eyre](https://crates.io/crates/eyre)/color-eyre (anyhow-style with customizable reports), [miette](https://crates.io/crates/miette) (diagnostics with source spans, for compilers and linters). Don't create a variant nobody will match: a variant is a promise to your callers.

### E02 · serde

**Teaches:** derive with `rename_all`, `default`, `skip`, `skip_serializing_if`, `deny_unknown_fields`; internally tagged enums; zero-copy deserialization with `&str` and `Cow` + `#[serde(borrow)]`; a hand-written `Serialize`/`Deserialize` that keeps a type's invariant; round-trip tests.

**Judgment:** serde is the standard: one data model and many formats (JSON, TOML, YAML, MessagePack, bincode...). Your wire format is a public API. `deny_unknown_fields` suits config files, where typos should fail loudly. It is risky for messages between services, where an old reader must accept a newer writer's extra fields. `#[serde(default)]` is how a format evolves. A derived `Deserialize` never calls your constructor, so validated types need `#[serde(try_from = "...")]` or a hand-written impl. Borrowing matters on hot paths; elsewhere owned `String`s are simpler. Alternatives: hand parsing for tiny formats; [miniserde](https://crates.io/crates/miniserde) (much smaller, less flexible); schema-first formats like protobuf ([prost](https://crates.io/crates/prost)) when other languages share the schema; [rkyv](https://crates.io/crates/rkyv) for zero-copy archives.

### E03 · clap

**Teaches:** derive-based parsers with subcommands, global flags, counted flags, defaults, ranges, `ValueEnum`, and custom value parsers. Testing them with `try_parse_from` and `Command::debug_assert`. Routing help and version output to stdout with exit code 0, and usage errors to stderr with exit code 2.

**Judgment:** For a CLI that people use, clap's help, suggestions, validation, and shell completions are worth its compile time. Validation belongs in the parser (`value_parser`): the error then names the flag, and nothing downstream sees invalid input. Keep `parse()` in `main` and test `try_parse_from`. Alternatives: [lexopt](https://crates.io/crates/lexopt) or [pico-args](https://crates.io/crates/pico-args) (tiny, manual), [argh](https://crates.io/crates/argh) (small derive), [bpaf](https://crates.io/crates/bpaf) (combinators), or plain `std::env::args` for a two-flag internal tool.

### E04 · tracing

**Teaches:** events vs spans; fields recorded with Display (`%`) and Debug; `#[instrument]` with `skip_all` so secrets never reach logs; fields recorded after the span is created (`field::Empty` + `record`); a custom `MakeWriter` that captures output in tests; `with_default` vs a global subscriber; level filtering, which skips argument evaluation.

**Judgment:** Libraries *emit* tracing data; only the binary installs a subscriber, once, in `main`. Spans carry context across `.await` points and threads, which is why async services choose tracing over [log](https://crates.io/crates/log). log + [env_logger](https://crates.io/crates/env_logger) is simpler and fine for small synchronous tools, and tracing-subscriber can collect `log` records. Production setups add the `env-filter` feature (`RUST_LOG=info,my_crate=debug`), `json` output, or [tracing-opentelemetry](https://crates.io/crates/tracing-opentelemetry). They are left out here to keep the build small. A span filtered out by its level also disappears from the context of the events inside it. E07 makes you deal with that.

### E05 · Property-based testing with proptest

**Teaches:** writing generators (`prop_oneof!`, `prop_map`, `collection::vec`, weights, `.boxed()`); round-trip, invariant, and model-based properties; `prop_assert!` inside property functions; why generators must reach the edge cases (runs longer than 255, empty input); how shrinking turns a random 700-byte failure into a minimal one. The supplied code is correct. The `m_` tests check your properties against deliberately broken mutants, and show that a round-trip property alone misses real bugs.

**Judgment:** proptest's integrated shrinking (each value remembers how to simplify itself) usually gives better counterexamples than [quickcheck](https://crates.io/crates/quickcheck), which is smaller and simpler. Coverage-guided fuzzing ([cargo-fuzz](https://github.com/rust-fuzz/cargo-fuzz) with [arbitrary](https://crates.io/crates/arbitrary)) explores deeper and runs for hours, outside `cargo test`. [cargo-mutants](https://mutants.rs) automates what the `m_` tests do by hand. Keep example tests for documented behavior and properties for searching. A property is only as good as its generator. When proptest finds a failure, it saves the seed under `ecosystem/proptest-regressions/`. Real projects commit those files; this package ignores them.

proptest is a dev-dependency, so this lesson's module exists only in test builds (`#[cfg(test)] mod e05_proptest;`).

### E06 · tokio

**Teaches:** `JoinSet` with a concurrency limit and per-task panic reporting; load shedding with `try_send`; backpressure with a deadline, using `reserve()` so a timeout hands the job back; a `select!` batcher whose state and deadline survive cancelled branches; `spawn_blocking` for CPU work. Tests use `#[tokio::test(start_paused = true)]`, so time is virtual and exact: no real sleeps, no flaky timing.

**Judgment:** tokio is the default runtime for network services, and most of that ecosystem (hyper, axum, reqwest, tonic, sqlx) builds on it. Async pays off when you have many concurrent waits (sockets, timers). For CPU-bound work, use threads or [rayon](https://crates.io/crates/rayon); for a handful of blocking tasks, plain threads and channels are simpler. Enable only the features you use: this package needs `macros`, `rt`, `sync`, and `time`, plus `test-util` for tests. Servers usually add `rt-multi-thread`, `net`, and `signal`. `full` is convenient, but it compiles everything. Alternatives: [smol](https://crates.io/crates/smol) (small, composable), [embassy](https://embassy.dev) (embedded). async-std is discontinued. Helpers you will meet next: `tokio-util`'s `CancellationToken`, and [async-channel](https://crates.io/crates/async-channel) for multi-consumer queues.

### E07 · Capstone: `jobq`

A small job runner that combines everything: a JSON config (serde, with validation), a clap CLI (`check`, `run`, `--workers`, `--json`, `-v`), a thiserror `ConfigError` with anyhow context in the application layer, tracing spans per job, and a tokio worker pool with a bounded queue, per-job timeouts, and `spawn_blocking`. `main` stays thin. The library has everything testable. The integration tests drive the library with virtual time and also run the real binary. Exit codes:

| Code | Meaning |
| --- | --- |
| 0 | every job succeeded (or `check` passed) |
| 1 | at least one job failed or timed out |
| 2 | usage error (clap) |
| 66 | config file not found (`EX_NOINPUT`) |
| 78 | invalid config (`EX_CONFIG`) |

The tracing test lives in its own test binary, `tests/e07_tracing.rs`. tracing caches each callsite's interest process-wide. Parallel tests that hit the same callsites while another test installs a scoped subscriber can silently lose events. One test per process avoids that. `cargo nextest` does it for every test.

### E08 · Tooling workday

A guide with checkable tasks, in [TOOLING.md](TOOLING.md): `cargo doc` and doc tests, `cargo tree` and feature unification, clippy configuration through `[lints]`, macro debugging without cargo-expand, release profiles, honest timing with `Instant`, and reading a flamegraph from folded stacks. Optional tools are listed with install commands; none is required.

## Pinned versions

| Crate | Version | Features |
| --- | --- | --- |
| serde | 1.0.229 | `derive` |
| serde_json | 1.0.154 | default |
| thiserror | 2.0.21 | default |
| anyhow | 1.0.104 | default |
| clap | 4.6.7 | default + `derive` |
| tracing | 0.1.44 | default |
| tracing-subscriber | 0.3.23 | `fmt`, `std` only (no defaults) |
| tokio | 1.54.1 | `macros`, `rt`, `sync`, `time`; dev: `test-util` |
| proptest (dev) | 1.12.0 | `std` only (no defaults) |

Versions are pinned with `=` so that error messages, help text, and log formats match the tests. To upgrade a crate in a real project: read its changelog, run `cargo update -p NAME --precise VERSION`, check `cargo tree --duplicates`, and run the full test suite.

## Validation scope

The starters compile in debug and release with `--locked`; `cargo fmt --check` and `cargo clippy --all-targets` are clean. A complete reference solution was used to confirm that every test, including the predictions, passes in both profiles. It is not shipped. The [hints](HINTS.md) give two progressive hints per lesson and a model explanation.
