# Chapter 17 lab — Regular expressions and lazy construction

This isolated Cargo package completes **Regular Expressions**, **Basic Regex Use**, and **Building Regex Values Lazily**. It is part of chapter mastery; run it when `regex` is available to fetch or cached. The core chapter target stays standard-library-only.

Read the official [regex API](https://docs.rs/regex/1.13.1/regex/) and [LazyLock API](https://doc.rust-lang.org/std/sync/struct.LazyLock.html). The package pins regex 1.13.1. The exercises intentionally use `LazyLock<Regex>` to practice the book's construction/lifetime approach.

Predict before coding: does `find` return character or byte offsets? Does `[0-9]` mean the same thing as Unicode-aware `\d`? When is the static regex compiled? Can multiple callers trigger multiple successful initializations?

Implement the four contracts in `src/lib.rs`, keeping the supplied LazyLock and counter scaffold. Run from the repository root:

```sh
cargo test --manifest-path exercises/chapter17/regex-lab/Cargo.toml
cargo test --release --manifest-path exercises/chapter17/regex-lab/Cargo.toml
```

Append `--offline` if crates are cached. The starter compiles; its TODOs fail tests. The initializer's TODO can poison LazyLock, causing later tests to report initialization poisoning until you implement the pattern and rerun the process.

1. **Build:** compile a pattern for ASCII lowercase key, `=`, then one or more ASCII decimal digits. Find substrings rather than anchoring the whole haystack. Name the captures `key` and `value`.
2. **Find:** return each whole-match substring and its byte range. Verify the first range starts at byte 3 after `"é "`.
3. **Capture:** parse each captured value as u32, skipping overflow. The `find` task still reports the overflow token: matching shape and parsing range are separate concerns.
4. **Replace:** transform every match to `key:value` with captured-group replacement while leaving surrounding text unchanged.
5. **Lazy construction:** the supplied test initializes from several threads and checks the initializer ran once. Explain the lifetime and synchronization responsibilities of LazyLock. Do not reset the global counter between tests.
6. **Manual comparison:** compare `Regex::new(r"\d+")` with `Regex::new(r"[0-9]+")` on `"٧ 7"`; the former sees both numeral runs, the latter only ASCII `7`. Attempt `Regex::new("[")` and handle its Err. Choose is_match when only existence matters, find for positions, and captures when groups are needed.
7. **Transfer:** add a regex for a whole record and explain how anchors change matching; distinguish syntactic date recognition from actual date validation.

<details><summary>Hint 1</summary>

Use raw literals for the pattern. Searching and matching shape do not validate an integer's range. Byte offsets of successful UTF-8 regex matches are valid slicing boundaries.

</details>
<details><summary>Hint 2</summary>

Use named groups `(?P<key>...)` and `(?P<value>...)`, `find_iter`, `captures_iter`, `parse::<u32>().ok()`, and `replace_all` with `${key}:${value}`. Return owned strings where the signatures require them.

</details>
