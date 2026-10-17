# I/O extension projects

These projects complement the automated in-memory tests. Record predictions, commands, results, and one limitation in your own notes.

## Files, buffering, seeking, and adapters

Create a fresh tree under `std::env::temp_dir()` with `create_dir` retrying on AlreadyExists; own it with a Drop cleanup guard. Write three framed records using `BufWriter<File>`, explicitly flush and close it, then read through `BufReader<File>`. Store frame offsets with `stream_position`, seek back to the middle frame, and compare it with the streamed result. Try `take`, `chain`, `io::empty`, `io::repeat`, and `io::sink` with a finite limit. Add a reader and writer that return Interrupted once and then make progress. Acceptance: short reads/writes, empty input, malformed frame, and a flush error have explicit outcomes; no file outside your temporary tree is modified.

## Compression and serialization (optional crates)

Create an isolated Cargo project under a temporary directory and add current `flate2`, `serde` (derive), and `serde_json` following their official docs. Define your own `Reading { station: String, values: Vec<i16> }`. Serialize several readings into a gzip stream, finish the encoder explicitly, and decode back. Test malformed JSON separately from corrupt/truncated compressed input, reject unexpected format versions, and impose a decompressed-size bound with `Read::take(limit + 1)` before accepting it. Compare the byte framing from l04 with schema-based serialization. Acceptance: round-trip, Unicode station name, empty readings, corrupt stream, over-limit expansion, and unsupported version. Explain why a successful gzip write is incomplete until finish/flush errors have been checked.

## Filesystem and platform observations

In your own temporary tree test create_new (collision), copy, rename, remove_file, create_dir_all, and read_dir. Compare Path::exists with fallible metadata; explain the time-of-check/time-of-use gap. On Unix inspect `PermissionsExt`/`MetadataExt` and create a symlink; on Windows inspect the corresponding Windows extension traits and document any required privileges. Use cfg gates. Compare canonicalize with lexical components; a lexical relative-path check is not a filesystem sandbox. Acceptance: sort directory entries before comparison, preserve non-Unicode names where supported, distinguish link metadata, and record platform-dependent behavior.

## Loopback TCP record server

Build a separate example using `TcpListener::bind(("127.0.0.1", 0))` and read the chosen port with local_addr. Spawn one server thread, accept exactly one client, and use l07's framing contract. Set read/write timeouts on both connected sockets so a broken protocol cannot hang a lab. Send the two-byte header in separate write_all calls, then a binary payload; read the complete response with read_exact. Join the server and propagate its Result. Test an empty payload, two consecutive frames, an oversized declared length, and half-closing the client write side. Acceptance: no public network or fixed ports, no sleeps, deterministic message boundaries, and shutdown on error. Explain why TCP may coalesce or split writes, and how this differs from UDP datagrams. Sketch a UDP variant with sender address handling and an explicit truncation policy.

References: [std::io](https://doc.rust-lang.org/std/io/), [std::fs](https://doc.rust-lang.org/std/fs/), [flate2](https://docs.rs/flate2/latest/flate2/), and [Serde](https://serde.rs/).
