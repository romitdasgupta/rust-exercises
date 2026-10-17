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

## l02 · Buffered text

<details><summary>Hint 1</summary>

lines() yields Result<String>, so do not discard read errors while filtering.

</details>

<details><summary>Hint 2</summary>

Collect an iterator of Result values into Result<Vec<_>>. split(0) produces byte vectors and owns each record.

</details>

## l03 · Writers and flushing

<details><summary>Hint 1</summary>

write_all handles short writes; flush is a separate fallible operation.

</details>

<details><summary>Hint 2</summary>

Convert the length before the first write. Use to_le_bytes rather than memory casts.

</details>

## l04 · Seeking and binary formats

<details><summary>Hint 1</summary>

A file position is measured in bytes. Reject multiplication overflow before seeking.

</details>

<details><summary>Hint 2</summary>

Use read_exact on fixed headers, then allocate the declared payload length. Decode with from_le_bytes.

</details>

## l05 · Paths preserve OS data

<details><summary>Hint 1</summary>

Path components have platform semantics, and components() normalizes some interior dot components.

</details>

<details><summary>Hint 2</summary>

file_name rejects parent-only paths. Work with PathBuf::set_extension and Component::Normal; never call to_str().unwrap().

</details>

## l06 · Directory metadata

<details><summary>Hint 1</summary>

DirEntry::file_type describes the entry without following a symlink. Sorting is an explicit API choice.

</details>

<details><summary>Hint 2</summary>

Use ? at both read_dir and each yielded entry. Match only ErrorKind::NotFound in the existence helper.

</details>

## l07 · Capstone: a bounded record service

<details><summary>Hint 1</summary>

Validate the header before allocating. The format length measures bytes, not characters.

</details>

<details><summary>Hint 2</summary>

Compose receive with reverse, write_all for header/body, and flush. The input reference may point to more than one frame.

</details>

