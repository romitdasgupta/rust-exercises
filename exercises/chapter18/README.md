# Chapter 18 — Input and Output

[All chapters](../../README.md) · [Previous chapter](../chapter17/README.md) · [Next chapter](../chapter19/README.md)

Original exercises accompanying [Programming Rust, 3rd edition, chapter 18](https://learning.oreilly.com/library/view/programming-rust-3rd/9781098176228/ch18.html). Read the corresponding section, then practice without copying the book's examples.

## Working route

Predict on paper first. Implement one lesson's TODOs, run its filtered tests, do its scratch experiment, and explain the result in your own words. Add one edge case of your own. Open [HINTS.md](HINTS.md) only after an attempt. Passing tests is one part of mastery; the explanation and labs matter too.

The starter compiles; unfinished TODOs intentionally fail when reached. Keep the supplied tests. Each lesson is independent, including the capstone, so later unfinished tasks do not block an earlier lesson.

| Done | Lesson | Coding tasks |
| --- | --- | --- |
| [ ] | [l01 · Streaming bytes](l01_readers.rs) | 2 |
| [ ] | [l02 · Buffered text](l02_buffered.rs) | 2 |
| [ ] | [l03 · Writers and flushing](l03_writers.rs) | 2 |
| [ ] | [l04 · Seeking and binary formats](l04_seek_binary.rs) | 2 |
| [ ] | [l05 · Paths preserve OS data](l05_paths.rs) | 2 |
| [ ] | [l06 · Directory metadata](l06_directories.rs) | 2 |
| [ ] | [l07 · Capstone: a bounded record service](l07_capstone.rs) | 2 |

```sh
cargo test --test chapter18 l01_
cargo test --test chapter18 -- --list
# After completing all lessons:
cargo test --test chapter18
cargo test --release --test chapter18
cargo run --example ch18_scratch
```

Use [the scratch example](../../examples/ch18_scratch.rs) for experiments and restore it to a compiling state afterward. Core lessons use only the standard library. Dependency or platform labs have their own instructions and are outside the normal test target.

## Coverage map

The section labels below follow the book's contents. A mapping to a lab means a concrete manual project, not automated coverage of an external library.

| Book sections | Practice |
| --- | --- |
| Readers and Writers; Readers; Other Reader and Writer Types | l01 generic byte copy; Cursor and adapter experiments |
| Buffered Readers; Reading Lines; Collecting Lines | l02 borrowed buffering and line ownership |
| Writers; Files; Seeking | l03 short writes and l04 seekable binary records; filesystem lab |
| Binary Data, Compression, and Serialization | l04 byte order; LABS.md framed binary and compression/Serde projects |
| Files and Directories; OsStr and Path; Path and PathBuf Methods | l05 path transformation without Unicode conversion |
| Filesystem Access Functions; Reading Directories; Platform-Specific Features | l06 directory metadata plus LABS.md temporary-tree/platform lab |
| Networking | l07 protocol capstone and LABS.md loopback TCP project |

## Extension labs

- [LABS.md](LABS.md)

## Mastery check

1. What may a successful Read::read or Write::write return before the buffer is exhausted?
2. When does read_line append, and how do lines() and split() differ?
3. How do Cursor, BufReader, Take, Chain, and Seek compose?
4. Why must paths and file names not be forced through String?
5. Which error kinds should retry, propagate, or end a stream?
6. How do you frame messages on TCP, and why does a single read not correspond to a whole message?

Redo the capstone from its contract in a later session. Explain one design tradeoff and create a case the supplied tests do not exercise.

## Validation scope

Use the repository's pinned Rust 1.99.0 toolchain and run commands from the repository root (see [setup instructions](../../README.md#fresh-clone-setup)). The dependency-free starter target can be checked without running TODOs using `cargo test --offline --test chapter18 --no-run`. No complete reference solution is shipped. Extension projects are manual practice and are outside the core compilation checks.
