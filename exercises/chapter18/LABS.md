# I/O extension projects

These projects complement the automated in-memory tests. Record predictions, commands, results, and one limitation in your own notes.

Each lab is a small standalone Cargo project. Create it outside this repository, for example with `cargo new --lib io-lab` under a temporary directory, so nothing here changes. Labs 1, 3, and 4 need only the standard library; lab 2 adds three crates.

Every lab follows the same rhythm: do a step, check the *you should now see* line, and only then move on. Finish by ticking every acceptance box.

## 1. Files, buffering, seeking, and adapters

**Dependencies:** none (standard library only).

```toml
[package]
name = "io-files-lab"
version = "0.1.0"
edition = "2024"

[dependencies]
```

**Steps**

1. **Temporary tree.** Create a fresh directory under `std::env::temp_dir()` with `fs::create_dir`, retrying with a new name on `AlreadyExists`. Wrap its path in a guard type whose `Drop` calls `fs::remove_dir_all`.
   *You should now see* the directory appear while your test runs and disappear afterward, even when the test panics.
2. **Write framed records.** Open a file in the tree and wrap it in `BufWriter<File>`. Write three records using the l04 framing (u16 little-endian length, then the payload). Call `flush()` explicitly, check its result, then drop the writer to close the file.
   *You should now see* a file whose length is the sum of the three payloads plus 6 header bytes.
3. **Read them back.** Open the file through `BufReader<File>` and read all three frames in order.
   *You should now see* exactly the three payloads you wrote, and then a clean end of file.
4. **Seek to the middle frame.** While reading in step 3, record each frame's start offset with `stream_position`. Seek back to the second offset and read one frame.
   *You should now see* the same bytes as the second frame from the streamed read.
5. **Adapters.** Try `take`, `chain`, `io::empty`, `io::repeat`, and `io::sink`. Always bound `io::repeat` with `take` (a finite limit).
   *You should now see*, for example, `io::repeat(b'x').take(3)` produce exactly `xxx`, `io::empty()` report end of file at once, and `io::sink()` accept every byte.
6. **Interrupted once.** Write a reader and a writer that return `ErrorKind::Interrupted` on their first call and then make progress. Run your l01 copy loop through them.
   *You should now see* the copy succeed with every byte intact.
7. **Edge cases.** Feed in short reads and short writes, empty input, a malformed frame (a header that declares more bytes than follow), and a writer whose `flush` fails.

**Acceptance criteria**

- [ ] Short reads and short writes, empty input, a malformed frame, and a flush error each have an explicit, tested outcome.
- [ ] The middle frame read after seeking equals the streamed one.
- [ ] No file outside your temporary tree is created, changed, or removed.

## 2. Compression and serialization (optional crates)

**Dependencies:** current versions of `flate2`, `serde` (with `derive`), and `serde_json`. Follow their official docs.

```toml
[package]
name = "io-serde-lab"
version = "0.1.0"
edition = "2024"

[dependencies]
flate2 = "1"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
```

**Steps**

1. **Isolated project.** Create the project under a temporary directory and add the dependencies above.
   *You should now see* `cargo build` fetch and compile the three crates.
2. **Data type.** Define your own `Reading { station: String, values: Vec<i16> }` and derive `Serialize` and `Deserialize`. Add a format version number to what you write.
3. **Encode.** Serialize several readings as JSON into a gzip stream (`flate2::write::GzEncoder`). Call `finish()` explicitly and check its result.
   *You should now see* a compressed byte vector that starts with the gzip magic bytes `1f 8b`.
4. **Decode.** Decompress and deserialize back.
   *You should now see* readings equal to the originals, including a Unicode station name and a reading with empty `values`.
5. **Separate failure modes.** Test malformed JSON inside a valid gzip stream separately from corrupt or truncated compressed input. Reject an unexpected format version.
   *You should now see* three different errors, each reported as the kind of failure it is.
6. **Bound the expansion.** Before accepting decompressed data, read through `Read::take(limit + 1)`; if you got more than `limit` bytes, reject the input.
   *You should now see* a small compressed input that expands past the limit rejected without decompressing all of it.
7. **Compare.** Compare the byte framing from l04 with schema-based serialization: what each makes easy, and what each makes hard to change later.

**Acceptance criteria**

- [ ] Round-trip of several readings.
- [ ] Unicode station name.
- [ ] Empty readings.
- [ ] Corrupt or truncated compressed stream, reported separately from malformed JSON.
- [ ] Over-limit expansion rejected with `take(limit + 1)`.
- [ ] Unsupported format version rejected.
- [ ] Notes explain why a successful gzip write is incomplete until the `finish`/`flush` errors have been checked.

## 3. Filesystem and platform observations

**Dependencies:** none (standard library only). Use `#[cfg(unix)]` and `#[cfg(windows)]` gates for platform code.

```toml
[package]
name = "io-fs-lab"
version = "0.1.0"
edition = "2024"

[dependencies]
```

**Steps**

1. **Your own tree.** Reuse the guard from lab 1, so every operation stays inside a temporary directory you created.
2. **Basic operations.** Test `OpenOptions::create_new` (once to create, and again to observe the collision), `fs::copy`, `fs::rename`, `fs::remove_file`, `fs::create_dir_all`, and `fs::read_dir`.
   *You should now see* the second `create_new` fail with `AlreadyExists`, and `read_dir` list exactly the entries you created.
3. **Existence.** Compare `Path::exists` with a fallible `fs::metadata` call on a missing path, and on a path you are not allowed to read (where your platform lets you create one).
   *You should now see* `exists` return `false` in both cases, while `metadata` tells them apart. In your notes, explain the time-of-check/time-of-use gap.
4. **Platform metadata.** On Unix, inspect `PermissionsExt` and `MetadataExt`, and create a symlink. On Windows, inspect the corresponding Windows extension traits and document any privileges required to create a symlink.
   *You should now see* `metadata` and `symlink_metadata` disagree about the link.
5. **Canonical versus lexical.** Compare `fs::canonicalize` with lexical `components()` on a path that contains `..` and on one that passes through your symlink.
   *You should now see* the two give different answers for the symlinked path. Note why a lexical relative-path check is not a filesystem sandbox.
6. **Non-Unicode names.** Where your platform supports it (most Unix file systems), create a file whose name is not valid UTF-8 and list it.

**Acceptance criteria**

- [ ] Directory entries are sorted before any comparison.
- [ ] Non-Unicode names are preserved where the platform supports them.
- [ ] Link metadata is distinguished from target metadata.
- [ ] Platform-dependent behavior is recorded in your notes, with `cfg` gates in the code.

## 4. Loopback TCP record server

**Dependencies:** none (standard library only). Build it as a separate example: its own Cargo project, or [the scratch example](../../examples/ch18_scratch.rs), restored afterward.

```toml
[package]
name = "io-tcp-lab"
version = "0.1.0"
edition = "2024"

[dependencies]
```

**Steps**

1. **Bind to loopback.** Call `TcpListener::bind(("127.0.0.1", 0))` and read the port the OS chose with `local_addr`.
   *You should now see* a different, nonzero port on each run. Never use a fixed port.
2. **One server thread.** Spawn one thread that accepts exactly one client and serves it with l07's framing contract (your `handle_one`).
3. **Timeouts.** Set read and write timeouts on both connected sockets (the server's accepted stream and the client's), so a broken protocol cannot hang the lab.
   *You should now see* a deliberately silent peer produce a `WouldBlock` or `TimedOut` error instead of a hang.
4. **Split header.** From the client, send the two header bytes in two separate `write_all` calls, then a binary payload. Read the complete response with `read_exact`.
   *You should now see* the reversed payload with the same framing, even though the header arrived in pieces.
5. **Join.** Join the server thread and propagate its `Result`, so a server error fails the run.
6. **Edge cases.** Send an empty payload, two consecutive frames, an oversized declared length, and half-close the client's write side (`shutdown(Shutdown::Write)`) before or in the middle of a frame.
7. **Explain.** In your notes, explain why TCP may coalesce or split writes, and how that differs from UDP datagrams. Sketch a UDP variant that handles the sender address and has an explicit truncation policy for datagrams larger than the buffer.

**Acceptance criteria**

- [ ] No public network and no fixed ports.
- [ ] No sleeps: timeouts and joins only.
- [ ] Message boundaries are deterministic, even with a split header.
- [ ] Both sides shut down on error.
- [ ] Empty payload, two consecutive frames, oversized length, and half-close are all tested.

References: [std::io](https://doc.rust-lang.org/std/io/), [std::fs](https://doc.rust-lang.org/std/fs/), [flate2](https://docs.rs/flate2/latest/flate2/), and [Serde](https://serde.rs/).
