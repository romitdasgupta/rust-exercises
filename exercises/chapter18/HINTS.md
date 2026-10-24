# Chapter 18 hints

[Chapter guide](README.md) · [All hints](../../HINTS.md)

Try hint 1 before opening hint 2.

## l01 · Streaming bytes

<details><summary>Hint 1</summary>

Do not assume one read fills your buffer. A zero-length successful read ends this copy.

</details>

<details><summary>Hint 2</summary>

Match Interrupted explicitly, copy only the initialized prefix, and increment a u64 count. Read::take borrows the reader when called on &mut R.

</details>

<details><summary>Check your explanation (after writing your own)</summary>

Taking `R: Read` means the same function works for files, sockets, byte slices, `Cursor`s, and test doubles that fail on purpose; requiring `File` would tie it to the filesystem and make the edge cases hard to test. A successful `read` may return fewer bytes than the buffer holds at any time, so callers must loop. `Ok(0)` means end of stream (for a nonempty buffer). `ErrorKind::Interrupted` means a signal interrupted the call before any data moved: nothing is lost, so retry; every other error is real and should propagate. `read_exact`, `write_all`, `read_to_end`, and `io::copy` already contain these loops.

</details>

## l02 · Buffered text

<details><summary>Hint 1</summary>

lines() yields Result<String>, so do not discard read errors while filtering.

</details>

<details><summary>Hint 2</summary>

Collect an iterator of Result values into Result<Vec<_>>. split(0) produces byte vectors and owns each record.

</details>

<details><summary>Check your explanation (after writing your own)</summary>

`lines()` and `read_line` decode each line as UTF-8 and return `InvalidData` for bytes that aren't, because their result is a `String`. Byte-oriented APIs such as `read_until` and `split` return `Vec<u8>` and never look at the encoding, so they accept any data. `read_line` *appends* to its String (clear it yourself in a loop) and keeps the line ending; `lines()` strips "\n" and "\r\n". Both report a final line that has no terminator, and `lines()` does not invent an empty line after a final newline.

</details>

## l03 · Writers and flushing

<details><summary>Hint 1</summary>

write_all handles short writes; flush is a separate fallible operation.

</details>

<details><summary>Hint 2</summary>

Convert the length before the first write. Use to_le_bytes rather than memory casts.

</details>

<details><summary>Check your explanation (after writing your own)</summary>

`write` may accept only part of the buffer and still return `Ok`, so use `write_all` unless you handle the count yourself. Buffered writers hold data in memory: an error from the underlying device may only surface when the buffer is flushed, and `BufWriter` flushes on drop *ignoring* any error. So a function that promises its output is complete must call `flush()` and propagate its result, and should do it once, after all writes succeeded. A framing function like `write_frame` should not flush: batching several frames before one flush is the caller's decision.

</details>

## l04 · Seeking and binary formats

<details><summary>Hint 1</summary>

A file position is measured in bytes. Reject multiplication overflow before seeking.

</details>

<details><summary>Hint 2</summary>

Use read_exact on fixed headers, then allocate the declared payload length. Decode with from_le_bytes.

</details>

<details><summary>Check your explanation (after writing your own)</summary>

A file is a sequence of bytes, so a format must say how many bytes each field occupies and in which order: a `u32` written on a little-endian machine reads back differently on a big-endian one unless both sides agree. `to_le_bytes`/`from_le_bytes` make that explicit and portable. Transmuting a struct also depends on padding, alignment, and field order, none of which are stable. Seeking only moves the position; on a `Cursor<Vec<u8>>`, writing after seeking past the end fills the gap with zeros. Offsets are in bytes, so compute `index * size` with `checked_mul` before seeking.

</details>

## l05 · Paths preserve OS data

<details><summary>Hint 1</summary>

Path components have platform semantics, and components() normalizes some interior dot components.

</details>

<details><summary>Hint 2</summary>

file_name rejects parent-only paths. Work with PathBuf::set_extension and Component::Normal; never call to_str().unwrap().

</details>

<details><summary>Check your explanation (after writing your own)</summary>

Lexical processing looks only at the text of a path: `components()`, `join`, `file_name`, `extension`. It never touches the disk, so it can't know about symlinks, mounts, or case-insensitive file systems. `canonicalize` asks the filesystem and resolves symlinks and `..`, but only for paths that exist, and the answer can change right after you get it. Neither is a security boundary: a path that is lexically "inside" a directory can still escape through a symlink or a race. Containment needs OS support (opening relative to a directory handle, sandboxing) rather than string checks.

</details>

## l06 · Directory metadata

<details><summary>Hint 1</summary>

DirEntry::file_type describes the entry without following a symlink. Sorting is an explicit API choice.

</details>

<details><summary>Hint 2</summary>

Use ? at both read_dir and each yielded entry. Match only ErrorKind::NotFound in the existence helper.

</details>

<details><summary>Check your explanation (after writing your own)</summary>

`read_dir` order is whatever the filesystem returns; it differs between platforms and can change as files are added, so a listing must sort explicitly to be deterministic. Symlinks need a stated policy: `metadata` follows a link to its target, while `symlink_metadata` and `DirEntry::file_type` describe the link itself, so "list regular files" without following links won't report a link to a file. Existence checks are observations: the answer can be stale by the next call (time of check versus time of use), so code should attempt the operation and handle its error instead. Only `NotFound` means "no"; permission and other errors must propagate.

</details>

## l07 · Capstone: a bounded record service

<details><summary>Hint 1</summary>

Validate the header before allocating. The format length measures bytes, not characters.

</details>

<details><summary>Hint 2</summary>

Compose receive with reverse, write_all for header/body, and flush. The input reference may point to more than one frame.

</details>

<details><summary>Check your explanation (after writing your own)</summary>

TCP and other byte streams have no message boundaries, so a protocol frames each message, here with a 2-byte little-endian length. The receiver reads the header with `read_exact`, validates the length *before* allocating or reading the body (otherwise a peer could declare a huge length to exhaust memory), and then insists on every declared byte, treating a short body as `UnexpectedEof`. Errors propagate unchanged, and no response is written until the whole request has been received and validated. The response is written with `write_all` and flushed once at the end, so a buffered transport actually sends it.

</details>

## R01 · Spaced review of Chapters 3, 7, 11, 15–17

<details><summary>Hint 1</summary>

Compare lines by `chars().count()`, not `len()`, and replace the best only on a strictly larger count so ties keep the earliest. For the tally, a map entry plus `checked_add` turns overflow into an error instead of a wrap or a panic. For loading, `?` uses your `From<io::Error>` impl.

</details>

<details><summary>Hint 2</summary>

`for (i, line) in input.lines().enumerate() { let line = line?; ... }`. Take fields with `split_whitespace()` and require exactly two (`let (Some(name), Some(n), None) = (f.next(), f.next(), f.next()) else { ... }`). `*e = e.checked_add(n).ok_or(...)?`. `source()` returns `Some(e)` for both variants; `line.parse().map_err(|source| LoadError::Parse { line: i + 1, source })?`.

</details>

## C01 · Code review

<details><summary>Hint 1</summary>

There are two bugs: one miscounts lines at the edges, one assumes a single `write` sends everything. Two `unwrap()` calls turn ordinary input into a panic.

</details>

<details><summary>Hint 2</summary>

Count `b'\n'` bytes through a `BufReader` (`fill_buf`/`consume`, or `read_until`) and add one if the last byte wasn't a newline. Use `write!(out, ...)` (which calls `write_all` internally) and add up the lengths yourself, or wrap the writer in a small counting adapter. Build the backup name from `path.as_os_str().to_owned()` and `OsString::push`.

</details>

<details><summary>Check your review</summary>

`count_lines` splits on '\n' and counts pieces, so a trailing newline adds a phantom line and empty input counts as 1; it reads the whole stream into one String (memory grows with the input), rejects non-UTF-8 input that line counting never needed to decode, and `unwrap()` panics on read errors instead of returning them. `write_report` uses `write`, which may accept only part of a line, so output is silently truncated while the byte count looks plausible; it also allocates a String per row when `write!` can format straight into the writer. `backup_name` panics on non-UTF-8 file names via `to_str().unwrap()` and allocates through `format!`; `&PathBuf` should be `&Path`, which clippy flags.

</details>

## Drills

Run `cargo drill ch18`. The compiler output explains *what* happened; these notes explain *why*. Read one only after the drill's verdict.

<details><summary>d01 · Read methods need the trait in scope</summary>E0599: `read_to_string` is a method of the `Read` trait, and trait methods are only callable when the trait is in scope. Add `use std::io::Read;` (or `use std::io::prelude::*;`).</details>
<details><summary>d02 · What lines() yields</summary>E0308: `lines()` yields `io::Result<String>`, because reading or decoding any line can fail. Handle the error: `.unwrap()` it here, or use `?` in a function that returns `io::Result`.</details>
<details><summary>d03 · read_exact on a short input</summary>Panics: `read_exact` must fill the whole buffer, and three bytes can't fill four, so it returns `UnexpectedEof`. Use a 3-byte buffer, or `read_to_end` when the length isn't fixed.</details>
<details><summary>d04 · lines() consumes the reader</summary>E0382: `BufRead::lines` takes `self`, so the `BufReader` is moved into the iterator. Iterate over `(&mut reader).lines()` instead: `&mut B` implements `BufRead` too, so the reader survives (make the binding `mut`).</details>
<details><summary>d05 · Printing a Path</summary>E0277: `Path` doesn't implement `Display`, because a path may not be valid Unicode and printing it means choosing a lossy conversion. `path.display()` makes that choice explicit.</details>
<details><summary>d06 · A byte slice is a reader</summary>Compiles and runs. `&[u8]` implements `Read` by copying from the front and then advancing the slice itself, so after reading two bytes `input` is `b"c"`. That's why the binding must be `mut`.</details>
<details><summary>d07 · Where are the bytes?</summary>Panics: the five bytes are still in `BufWriter`'s buffer, so the inner Vec is empty. Call `out.flush()` (or `into_inner()`, which flushes and returns the Vec) before inspecting it.</details>
<details><summary>d08 · A file name is not a str</summary>E0308: `file_name` returns `Option<&OsStr>`, because OS file names need not be UTF-8. Convert deliberately: `.to_str()` returns `Option<&str>` (None for non-Unicode names), and `.to_string_lossy()` replaces invalid data.</details>
<details><summary>d09 · Reading needs a mutable reader</summary>E0596: `read_exact` takes `&mut self` because reading advances the reader's position. Declare the parameter `mut input: R` (callers can still pass `&mut reader`, since `&mut R` is also `Read`).</details>
