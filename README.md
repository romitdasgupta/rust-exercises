# Programming Rust — guided exercises

A chapter-by-chapter practice companion to [*Programming Rust*, 3rd edition](https://learning.oreilly.com/library/view/programming-rust-3rd/9781098176228/), covering Chapter 3 and every remaining technical chapter (4–24).

Chapters 3–24 contain **180 guided lessons, 21 spaced-review lessons, 22 code-review lessons, about 550 checked predictions, 217 compiler drills, and over 1,000 tests**. Every chapter includes a capstone and a map from the book sections to its exercises or labs. Two further tracks take you from the book to everyday professional Rust: a [production-crates track](ecosystem/README.md), and a project you build across the chapters (see [Beyond the book](#beyond-the-book)).

The exercises are original practice problems, not a reproduction of the book. Read the corresponding chapter alongside them. Each chapter has its own guide, lessons, compiler drills, progressive hints with model explanations, a notes workbook, a coverage map, and a capstone.

## Fresh clone setup

Install Git and [Rust through rustup](https://www.rust-lang.org/tools/install). The repository pins Rust **1.99.0** with rustfmt and clippy, and uses edition 2024. Rustup installs the pinned toolchain the first time you run Cargo in this directory.

```sh
git clone https://github.com/romitdasgupta/rust-exercises.git
cd rust-exercises
cargo test --locked --offline --all-targets --no-run
cargo test --locked --offline --release --all-targets --no-run
cargo progress
```

Run `cargo progress` once before you edit anything: it saves a copy of every untouched lesson so you can redo it later.

The build commands compile every exercise without running unfinished functions. The core needs no external crates or network access once Rust is installed. Run all commands from the repository root unless a lab says otherwise.

Every exercise starts unfinished: tests fail at `todo!()` and `guess()` until you fill them in. That's the starting state, not a broken setup.

## How you will learn

Reading about Rust and recognizing it later is not the same as writing it under pressure. These exercises are built on the techniques that make skills last: **predicting before you see the answer, retrieving from memory, getting immediate feedback, explaining in writing, and revisiting material on a spaced schedule.**

For each lesson:

1. **Read** the book section.
2. **Predict.** Each lesson ends with a `p_predictions` test. Replace each `guess()` with what you think the code produces, *before running anything*. Wrong guesses are the most valuable part: write down why in the chapter's `NOTES.md`. Once every prediction in a file is answered, the compiler warns that `guess` is an unused import; delete it from the `use` line.
3. **Implement** the `todo!()` bodies from their contracts and run only that lesson: `cargo test --test chapter04 l01_`.
4. **Check your technique.** Some tests count heap allocations or compare pointers, so a solution that clones everything fails even when its output is right. Run `cargo clippy --test chapter04` and fix its warnings in your code; clippy encodes the idioms that experienced Rust programmers use.
5. **Drill the compiler.** `cargo drill ch04` gives you short programs. Predict whether each compiles (and with which error code) or panics, see the verdict, then fix the program. Reading compiler errors fluently is most of day-to-day Rust.
6. **Explain** in `NOTES.md`, add an edge-case test, then compare with *Check your explanation* in the chapter's `HINTS.md`.

Each chapter also has:

- **R01, a review lesson.** It mixes earlier chapters into new problems, done from memory. Mixing topics feels harder than practicing one at a time, and that is why it sticks.
- **C01, a code-review lesson.** A teammate's code that compiles but has bugs, needless clones, and poor signatures. You review it, fix it, and clear the clippy warnings, as you would at work.
- **A capstone**, redone from its contract during spaced reviews.

### Your daily session

```sh
cargo progress          # where you are, the next lesson, and reviews due today
```

A good session is 45–60 minutes: clear any reviews that are due first, then continue with the next lesson. When a whole chapter passes, `cargo progress` schedules reviews 1, 3, 7, 21, and 60 days later. A review means:

1. Answer the chapter's mastery questions in `NOTES.md` from memory.
2. Run `cargo drill reset ch04` and predict the drills again.
3. Run `cargo progress redo ch04 l08` to archive your capstone and restore its starter, then solve it again from the contract.
4. Record it with `cargo progress reviewed ch04`.

Your progress history lives in `.progress/`, which Git ignores.

When you finish a chapter, also fill in its row in [DECISIONS.md](DECISIONS.md). It condenses the chapter into the judgment calls you make while writing real code: borrow or own, generic or `dyn`, `Mutex` or channel.

### Tools at a glance

| Command | What it does |
| --- | --- |
| `cargo test --test chapter04 l01_` | Run one lesson (use `r01_`, `c01_` for review and code review). |
| `cargo clippy --test chapter04` | Lint a chapter; fix the warnings in your code. |
| `cargo drill` / `cargo drill ch04` | Drill summary, or work through a chapter's compiler drills. |
| `cargo drill reset ch04` | Restore a chapter's drills to predict them again. |
| `cargo progress` | Lesson status, next step, and spaced reviews due. |
| `cargo progress redo ch04 l08` | Archive your solution and restore the lesson's starter. |
| `cargo progress reviewed ch04` | Record a completed review; schedules the next one. |
| `cargo run --example ch04_scratch` | Free-form experiments for a chapter. |

## Begin or continue

Start with [Chapter 3: Fundamental Types](exercises/chapter03/README.md), or with [Chapter 4: Ownership and Moves](exercises/chapter04/README.md) if you are comfortable with Rust's basic types. Chapter 4 shows every element described above.

For review with a study partner or tutor, share the lesson number, your predictions, your `NOTES.md`, and your code or test output. The [hint directory](HINTS.md) links to two progressive hints per lesson, and a model explanation to compare against your own.

## Chapters

Check a chapter off only after its lessons, predictions, drills, review, code review, notes, and capstone are complete.

| Done | Chapter | Guide | Run the chapter |
| --- | --- | --- | --- |
| [ ] | 03 | [Fundamental Types](exercises/chapter03/README.md) | `cargo test --test chapter03` |
| [ ] | 04 | [Ownership and Moves](exercises/chapter04/README.md) | `cargo test --test chapter04` |
| [ ] | 05 | [References](exercises/chapter05/README.md) | `cargo test --test chapter05` |
| [ ] | 06 | [Expressions](exercises/chapter06/README.md) | `cargo test --test chapter06` |
| [ ] | 07 | [Error Handling](exercises/chapter07/README.md) | `cargo test --test chapter07` |
| [ ] | 08 | [Crates and Modules](exercises/chapter08/README.md) | `cargo test --test chapter08` |
| [ ] | 09 | [Structs](exercises/chapter09/README.md) | `cargo test --test chapter09` |
| [ ] | 10 | [Enums and Patterns](exercises/chapter10/README.md) | `cargo test --test chapter10` |
| [ ] | 11 | [Traits and Generics](exercises/chapter11/README.md) | `cargo test --test chapter11` |
| [ ] | 12 | [Operator Overloading](exercises/chapter12/README.md) | `cargo test --test chapter12` |
| [ ] | 13 | [Utility Traits](exercises/chapter13/README.md) | `cargo test --test chapter13` |
| [ ] | 14 | [Closures](exercises/chapter14/README.md) | `cargo test --test chapter14` |
| [ ] | 15 | [Iterators](exercises/chapter15/README.md) | `cargo test --test chapter15` |
| [ ] | 16 | [Collections](exercises/chapter16/README.md) | `cargo test --test chapter16` |
| [ ] | 17 | [Strings and Text](exercises/chapter17/README.md) | `cargo test --test chapter17` |
| [ ] | 18 | [Input and Output](exercises/chapter18/README.md) | `cargo test --test chapter18` |
| [ ] | 19 | [Threads](exercises/chapter19/README.md) | `cargo test --test chapter19` |
| [ ] | 20 | [Asynchronous Programming](exercises/chapter20/README.md) | `cargo test --test chapter20` |
| [ ] | 21 | [Futures](exercises/chapter21/README.md) | `cargo test --test chapter21` |
| [ ] | 22 | [Macros](exercises/chapter22/README.md) | `cargo test --test chapter22` |
| [ ] | 23 | [Unsafe Code](exercises/chapter23/README.md) | `cargo test --test chapter23` |
| [ ] | 24 | [Foreign Functions](exercises/chapter24/README.md) | `cargo test --test chapter24` |

## Running the exercises

The runnable core uses the standard library and requires no external crates. Each chapter is a separate Cargo integration-test target, so an unfinished later chapter does not run when you select an earlier one. All lesson modules within the selected chapter must still compile.

```sh
# Check that all starters compile without running unfinished exercises.
cargo test --offline --all-targets --no-run

# List a chapter's tests without running them.
cargo test --test chapter04 -- --list

# Run one lesson, or one specific test using a longer name filter.
cargo test --test chapter04 l01_

# After completing a chapter, check both build profiles.
cargo test --test chapter04
cargo test --release --test chapter04

cargo clippy --test chapter04
cargo fmt --check
```

Unimplemented functions deliberately panic at `todo!()`, and unanswered predictions at `guess()`. A failing exercise is the starting state, not a broken installation. Do not run the whole book's tests expecting a green result before completing the exercises. Compiler labs deliberately include snippets that should fail to compile; keep those experiments in the scratch files or isolated lab projects and restore them afterward.

The supported toolchain is Rust 1.99.0, pinned in `rust-toolchain.toml`; the package uses edition 2024. Some lessons intentionally explore edition rules and recent features described in the third edition. Compiler diagnostics and opaque type names may differ across toolchains; test contracts focus on behavior rather than exact diagnostic wording.

## Your main.rs playground

`src/main.rs` is your personal experiment file. Run it with `cargo run --features playground`. The feature is off during the chapter tests, so an intentional compiler error in the playground does not block your exercise checks. Scratch examples are compiled by `--all-targets`, so repair intentional scratch errors before running that broad check.

## Beyond the book

| Track | When | What it practices |
| --- | --- | --- |
| [Production crates](ecosystem/README.md) | E01 after Chapter 7; the rest as their chapters are done | serde, thiserror and anyhow, clap, tracing, proptest, tokio, and everyday Cargo tooling, with "why this crate, and what are the alternatives" judgment for each. A standalone package with pinned dependencies (one download, then offline). |

## Labs that need another tool or crate

The chapter guides distinguish the runnable core from explicit labs for Cargo package structure, optional crates, async runtimes, system libraries, and build/linker behavior. Follow those labs in their own temporary Cargo projects. They have concrete implementation goals and expected observations; they are part of the coverage, not automated by the core tests.

In particular, Rayon, Tokio, regular expressions, compression, and libgit2 behavior cannot be demonstrated completely by a standard-library substitute. The relevant chapters identify these labs and their prerequisites. Package publishing is practiced locally; no exercise requires publishing a crate or changing an external service. Network exercises use local fixtures or loopback addresses.

Three extension packages are supplied directly in the repository:

| Package | What it practices | Guide |
| --- | --- | --- |
| Random elements | Seeded selection, shuffling, sampling, and robust random tests | [Chapter 16 random lab](exercises/chapter16/random-lab/README.md) |
| Regular expressions | Search, captures, replacement, and thread-safe lazy compilation | [Chapter 17 regex lab](exercises/chapter17/regex-lab/README.md) |
| Exported macros | A real external consumer, a renamed dependency, and public helper paths | [Chapter 22 export lab](exercises/chapter22/export_lab/README.md) |

These packages have their own manifests and commands in their guides. The random and regex packages pin their dependencies; they can run offline when those crates are cached.

For unsafe and FFI work, satisfy the supplied safety contracts and explain them before executing your implementation. Compiler rejection is an appropriate experiment; executing undefined behavior is not an exercise objective.

## How to judge mastery

A green test result checks the required behavior. It does not prove that you used the intended technique or can explain the design. Use each chapter's coverage table and mastery questions as a second checklist. Be ready to describe ownership, invariants, error behavior, and edge cases, then solve a changed version of the problem without copying your solution.

## Validation scope

A clean starter checkout should compile all 22 chapter targets (3–24) and their scratch examples offline in debug and release profiles using the fresh-clone commands above. Formatting is checked separately. The isolated random, regex, external macro-consumer, and production-crates packages have their own manifests and are not included in the root Cargo checks. The random, regex, and production-crates packages need a dependency download before offline builds work. The [starter CI workflow](.github/workflows/ci.yml) checks formatting and clippy, runs the coach's own tests, and compiles the core (including the playground) and every extension package in both profiles on Linux, macOS, and Windows; it deliberately does not execute unfinished exercises.

Running unfinished tests is expected to fail. Compilation checks establish that a learner can start; they do not establish that unfinished implementations are correct. No complete reference solution is shipped. Tests also do not prove that an unsafe abstraction is sound for every possible caller: its written safety argument is part of the exercise.

Compiler experiments and the assigned Cargo/process, Tokio, Mio, Rayon, compression, procedural-macro, generic gap-buffer, Miri, native C/libgit2, and platform-specific projects are manual practice. They are outside the automated core checks. Their prerequisites and acceptance criteria are stated in the chapter guides or lab instructions.

## License

Licensed under the [MIT License](LICENSE).
