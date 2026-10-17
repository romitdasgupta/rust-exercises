# Programming Rust — guided exercises

A chapter-by-chapter practice companion to [*Programming Rust*, 3rd edition](https://learning.oreilly.com/library/view/programming-rust-3rd/9781098176228/), covering Chapter 3 and every remaining technical chapter (4–24).

Chapters 4–24 add **167 guided lessons and 396 core implementation tasks**, plus **7 implementation tasks in the isolated random and regex packages**. Every chapter includes a capstone and a map from the book sections to its exercises or labs.

The exercises are original practice problems, not a reproduction of the book. Read the corresponding chapter alongside them. Each chapter has its own guide, lesson files, progressive hints, tests, compiler experiments, coverage map, and capstone.

## Fresh clone setup

Install Git and [Rust through rustup](https://www.rust-lang.org/tools/install). The repository pins Rust **1.99.0** and uses edition 2024. Rustup installs the pinned toolchain when you first run Cargo in this directory. Install the formatting component explicitly if your rustup profile does not include it.

Replace `<owner>` and `<repository>` with the GitHub repository's actual owner and name:

```sh
git clone https://github.com/<owner>/<repository>.git
cd <repository>
rustup toolchain install 1.99.0 --profile minimal --component rustfmt
cargo test --locked --offline --all-targets --no-run
cargo test --locked --offline --release --all-targets --no-run
cargo fmt --all -- --check
cargo test --test chapter03 -- --list
```

The build commands compile the exercises without executing unfinished functions. No external crates or network access are needed for these core checks once Rust is installed. Run all commands in this guide and the chapter guides from the repository root unless a lab says otherwise.

This is a practice repository with partially completed Chapter 3 work and unfinished exercises throughout. Some tests already pass; others intentionally fail at `todo!`. Start by listing tests, then work through one lesson at a time.

## Begin or continue

Start with [Chapter 3: Fundamental Types](exercises/chapter03/README.md), or continue with [Chapter 4: Ownership and Moves](exercises/chapter04/README.md).

1. Open a chapter guide and follow its lesson order.
2. Write your predictions before running an example or looking up an answer.
3. Replace the lesson's `todo!()` bodies, keeping the test contracts intact.
4. Run only that lesson while you work:

   ```sh
   cargo test --test chapter04 l01_
   ```

5. Use its compiler lab to explore the boundary between accepted and rejected programs. There is a scratch example for each chapter: `cargo run --example ch04_scratch`.
6. Answer the explanation questions and add an edge case of your own before checking the lesson off.
7. Finish the capstone, then repeat it later from the contract without consulting your previous implementation.

For review with a study partner or tutor, share the lesson number, your predictions, and your code or test output. The [hint directory](HINTS.md) links to two progressive hints per lesson.

## Chapters

Check a chapter off only after its code, labs, explanation prompts, and capstone are complete.

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

cargo fmt --check
```

Unimplemented functions deliberately panic at `todo!()`. A failing exercise is the starting state, not a broken installation. Do not run the whole book's tests expecting a green result before completing the exercises. Compiler labs deliberately include snippets that should fail to compile; keep those experiments in the scratch files or isolated lab projects and restore them afterward.

The supported toolchain is Rust 1.99.0, pinned in `rust-toolchain.toml`; the package uses edition 2024. Some lessons intentionally explore edition rules and recent features described in the third edition. Compiler diagnostics and opaque type names may differ across toolchains; test contracts focus on behavior rather than exact diagnostic wording.

## Your main.rs playground

`src/main.rs` is your personal experiment file. Run it with `cargo run --features playground`. The feature is off during the chapter tests, so an intentional compiler error in the playground does not block your exercise checks. Scratch examples are compiled by `--all-targets`, so repair intentional scratch errors before running that broad check.

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

A clean starter checkout should compile all 22 chapter targets (3–24) and their scratch examples offline in debug and release profiles using the fresh-clone commands above. Formatting is checked separately. The isolated random, regex, and external macro-consumer packages have their own manifests and are not included in the root Cargo checks. The first two need a dependency download before offline builds work. The [starter CI workflow](.github/workflows/ci.yml) checks formatting and compiles the core (including the playground) and all three extension packages in both profiles on Linux, macOS, and Windows; it deliberately does not execute unfinished exercises.

Running unfinished tests is expected to fail. Compilation checks establish that a learner can start; they do not establish that unfinished implementations are correct. No complete reference solution is shipped. Tests also do not prove that an unsafe abstraction is sound for every possible caller: its written safety argument is part of the exercise.

Compiler experiments and the assigned Cargo/process, Tokio, Mio, Rayon, compression, procedural-macro, generic gap-buffer, Miri, native C/libgit2, and platform-specific projects are manual practice. They are outside the automated core checks. Their prerequisites and acceptance criteria are stated in the chapter guides or lab instructions.

## License

Licensed under the [MIT License](LICENSE).
