# Chapter 17 hints

[Chapter guide](README.md) · [All hints](../../HINTS.md)

Try the chapter reference and compiler diagnostics first. Open one level at a time.

## 01 · Unicode scalar operations and encoding

<details><summary>Hint 1</summary>

char is a Unicode scalar value. UTF-8 uses variable byte lengths; classification is not restricted to ASCII.

</details>
<details><summary>Hint 2</summary>

Use char::from_u32 to reject surrogates and out-of-range code points. Collect to_uppercase because it is an iterator, and use to_digit for the requested radix.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

Case mapping is defined on whole strings of scalars, and some letters have no single-scalar counterpart: 'ß' uppercases to "SS", 'İ' lowercases to 'i' plus a combining dot, so `to_uppercase`/`to_lowercase` return iterators, not a char. A `char` is one Unicode scalar value (never a surrogate), not a user-perceived character: "e\u{301}" is two scalars that display as one. Reversing scalars would split such clusters and still wouldn't produce correct right-to-left display, because text is stored in logical order and the bidirectional algorithm decides visual order at render time. `chars()` does no normalization, segmentation, or bidi layout.

</details>

## 02 · Constructing and editing owned text

<details><summary>Hint 1</summary>

String methods preserve UTF-8, but byte-indexed edits require valid boundaries.

</details>
<details><summary>Hint 2</summary>

Use is_char_boundary before insert_str. For scalar truncation obtain the byte offset from char_indices; pop returns a whole scalar.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

Every String method keeps the buffer valid UTF-8, so any operation that takes a byte offset (`insert`, `insert_str`, `truncate`, `drain`, `replace_range`, slicing) panics if the offset falls inside a multi-byte scalar; check `is_char_boundary` or get offsets from `char_indices`. Capacity is measured in bytes, not characters. `push`, `push_str`, `insert`, `truncate`, `pop`, `remove`, and `clear` edit the existing allocation (growing it if needed); `+` reuses the left operand's buffer; `format!`, `to_uppercase`, `replace`, and `collect` build a new String. `pop` and `remove` always take a whole scalar.

</details>

## 03 · Search patterns, byte positions, and replacement

<details><summary>Hint 1</summary>

Search positions are byte offsets so they can index valid substrings at match boundaries.

</details>
<details><summary>Hint 2</summary>

Use find with the supplied char, match_indices for literal occurrences, and replacen for bounded replacement.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

A `&str` pattern matches a literal substring; a `char` matches one scalar; a `&[char]`, an array of chars, or a closure `FnMut(char) -> bool` matches any one scalar that satisfies it. All searches return *byte* offsets, which are always valid slice boundaries because a match starts and ends on scalar boundaries. Matches never overlap: "aaaa" contains "aa" at 0 and 2, not at 1. `replace` changes every match; `replacen(old, new, n)` lets you say how many, which matters when only the first few occurrences are meant to change.

</details>

## 04 · Text iteration, trimming, and casing

<details><summary>Hint 1</summary>

Use the iterator whose delimiters and ownership match the contract. Whole-string casing may use surrounding context.

</details>
<details><summary>Hint 2</summary>

Use lines for CRLF handling, trim each borrowed line, filter blanks, and to_uppercase on the complete string.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

Lowercasing both sides handles simple cases, but full Unicode case-insensitive matching needs case *folding* and often normalization: 'ß' vs "SS", final sigma 'ς' vs 'σ', and the Turkish dotted/dotless i all break naive lowercasing. `str::to_lowercase` even depends on context (Σ at the end of a word becomes ς), which per-char mapping misses. `bytes()` answers storage and protocol questions (length, ASCII parsing); `chars()` answers questions about scalars. `trim` and `lines` return borrowed `&str` slices of the original, so they never allocate.

</details>

## 05 · FromStr and textual conversion

<details><summary>Hint 1</summary>

FromStr describes a fallible owned conversion and cannot return a borrow tied to its input through the trait signature.

</details>
<details><summary>Hint 2</summary>

Use split_once on x, reject extra separators, parse each side as u16, and map parse failures to the documented error.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

A parser that truncates or guesses turns bad input into wrong data that surfaces far away, while a `Result` makes the failure visible where the input entered the program and lets the caller decide. `str::parse::<T>()` is a thin call to `T::from_str`, so implementing `FromStr` makes `parse` work for your type; `to_string` comes from any `Display` impl via the blanket `ToString`. Integer parsing is strict: no surrounding whitespace, but an optional '+' (and '-' only for signed types). Display is for humans and need not round-trip; when a format must round-trip, specify it and test it.

</details>

## 06 · The formatting language

<details><summary>Hint 1</summary>

Width is a minimum, while precision has type-dependent meaning. Keep field alignment and numeric rounding separate.

</details>
<details><summary>Hint 2</summary>

Use named dynamic width/precision in format!, alternative hex with zero padding, and Debug rather than Display for the vector.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

Named arguments and `width$`/`precision$` parameters let one format string serve every row, with the numbers supplied at run time. Width is a *minimum*, counted in Unicode scalars, so long text is never truncated and wide characters (emoji, CJK) can still misalign columns in a terminal. Precision rounds floats but truncates strings. `{:#06x}` counts the "0x" prefix inside the width and zero-pads after it. `{:p}` prints an address chosen by the allocator and by ASLR, so it differs between runs and must never appear in an expected value.

</details>

## 07 · Display, fmt::Write, and format_args!

<details><summary>Hint 1</summary>

Custom Display implementations write through the formatter. An API can accept fmt::Arguments and forward them.

</details>
<details><summary>Hint 2</summary>

Use write!(f,...) in Display, fmt::Write::write_fmt for the arguments API, and ? on each fallible output operation.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

`fmt::Error` only signals that the destination refused the output (a full buffer, a closed pipe); `Display` should never return it for "bad data", because callers like `to_string` panic on it. Validate in constructors or `FromStr` instead. A `Display` impl should write through the formatter with `write!` and propagate each `?`, and use `f.pad` when it wants to honor width and alignment. `format_args!` builds an `fmt::Arguments` value that borrows its arguments and describes the formatting without performing it; `write_fmt` streams the pieces straight into the target writer, so no intermediate String is allocated.

</details>

## 08 · UTF-8, borrowed text views, and delayed allocation

<details><summary>Hint 1</summary>

Use strict conversion when invalid bytes should be rejected; use lossy conversion only with an explicit replacement policy.

</details>
<details><summary>Hint 2</summary>

Call String::from_utf8 and recover err.into_bytes() on failure. Cow distinguishes valid borrowed input from a repaired owned result. Join borrowed words instead of cloning each first.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

OS strings and paths may hold bytes (Unix) or unpaired UTF-16 (Windows) that are not valid Unicode, so `to_str().unwrap()` panics on real file names; keep them as `OsStr`/`Path` and convert only for display (`display()`, `to_string_lossy`). `String::from_utf8` and `str::from_utf8` validate and guarantee UTF-8; `from_utf8_lossy` guarantees UTF-8 by replacing bad sequences with U+FFFD and borrows when nothing needed replacing. `into_bytes`, `FromUtf8Error::into_bytes`, and `OsString` keep arbitrary bytes and the original allocation. `Cow` delays allocation until a change is actually needed.

</details>

## 09 · Capstone: a Unicode-aware report parser

<details><summary>Hint 1</summary>

Parse shape and value separately, then format only validated records. A typed error can retain the failing source line.

</details>
<details><summary>Hint 2</summary>

Use split_once, trim, reject extra separators, and parse u16. Use char counts for the explicitly requested scalar metric and write! into the output String.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

The separators '=' and '\n' are ASCII, and in UTF-8 an ASCII byte never appears inside a multi-byte scalar, so the byte offsets that `split_once` and `lines` find are always valid boundaries. The parser should keep enough context to act on errors: which line failed (1-based, counted before skipping comments) and, ideally, why. Rendering counts scalars because the contract asks for scalars; "e\u{301}" is 2 scalars but one user-perceived character, and that gap is deliberately out of scope here, along with normalization and display width.

</details>

## R01 · Spaced review of Chapters 3, 7, 10, 15, 16

<details><summary>Hint 1</summary>

An anagram key is the word's chars, sorted, collected back into a String. Initials are "first char of each word", mapped to uppercase, flattened. For commands, peel off the first word, then let a `match` on it decide how to read the rest; implement `From` so `?` can convert parse errors.

</details>
<details><summary>Hint 2</summary>

`let mut cs: Vec<char> = w.chars().collect(); cs.sort_unstable();` then `map.entry(cs.into_iter().collect()).or_default().push(w)`. `split_whitespace().filter_map(|w| w.chars().next()).flat_map(char::to_uppercase).collect()`. `line.split_once(char::is_whitespace).unwrap_or((line, ""))`, then `let [dx, dy] = args[..] else { ... }` and `dx.parse()?`.

</details>

## C01 · Code review

<details><summary>Hint 1</summary>

There are two bugs, and both confuse ASCII or bytes with Unicode scalars. Most allocations in this file can go: count them per character, per call, and per pair.

</details>
<details><summary>Hint 2</summary>

Use `is_alphanumeric`, push into one String, and only emit a pending '-' before the next kept scalar. Find cut points with `char_indices().nth(..)` and return `Cow<'_, str>` so text that fits is borrowed. `text.split(';').filter_map(|s| s.split_once('='))` returns `Vec<(&str, &str)>` with no allocation per pair.

</details>
<details><summary>Check your review</summary>

`slugify` keeps only ASCII alphanumerics, so it deletes "é", "ü", Greek, and every other non-ASCII letter; it also allocates a new String for every character (`out = out + &c.to_string()` copies the whole slug each time), allocates again for the trimmed result, and takes `&String` where `&str` will do. `truncate_display` measures and slices bytes: it truncates too early on non-ASCII text, panics when `max - 1` lands inside a scalar, and returns an owned copy even when nothing changed. `parse_pairs` works but builds a `Vec` per segment and two Strings per pair; `split_once` says "first '='" directly, and returning borrowed slices needs no allocation at all. Clippy flags the `&String` parameter and the `out = out + ...` assignments.

</details>

## Drills

Run `cargo drill ch17`. The compiler output explains *what* happened; these notes explain *why*. Read one only after the drill's verdict.

<details><summary>d01 · Indexing a String</summary>E0277: `String` doesn't implement `Index<usize>`, because a byte index would not give a character and a character index would be O(n). Use `word.chars().next()` for the first scalar, or `word.as_bytes()[0]` when you really want a byte.</details>
<details><summary>d02 · Slicing inside a character</summary>Panics at run time: 'é' occupies bytes 1..3, so byte 2 is not a char boundary. Range indices are run-time values, so the compiler can't check them. Use `&word[..3]`, or compute the end with `char_indices().nth(2)`.</details>
<details><summary>d03 · Adding two &str</summary>E0369: `+` is implemented for `String + &str` (it appends into the left operand's buffer), not for two borrowed slices. Use `greeting.to_owned() + name` or `format!("{greeting}{name}")`.</details>
<details><summary>d04 · String + &str consumes the left side</summary>E0382: `Add for String` takes `self` by value, reuses its buffer, and returns it, so `base` is moved into `file`. Use `format!("{base}.txt")` (or `base.clone() + ".txt"`) when you still need `base`.</details>
<details><summary>d05 · push versus push_str</summary>E0308: `push` takes a `char` ('/'), `push_str` takes a `&str` ("/"). Write `path.push('/')`.</details>
<details><summary>d06 · Returning a borrow of a temporary</summary>E0515: `to_lowercase` creates a new String owned by the function; it is dropped at the end of the function, so a `&str` into it would dangle. Return the owned `String` (the KEEP line compares fine with a `&str`). A plain `trim()` *could* return `&str`, because it borrows from `input`.</details>
<details><summary>d07 · to_string without Display</summary>E0599: `to_string` comes from the blanket `impl<T: Display> ToString for T`. Implement `fmt::Display for Celsius` (`write!(f, "{}°C", self.0)`) and `to_string` appears.</details>
<details><summary>d08 · Comparing String with &str</summary>Compiles and runs. The standard library implements `PartialEq` between `String`, `str`, `&str`, and `Cow<str>` in both directions, so no conversion is needed to compare them.</details>
<details><summary>d09 · parse without a target type</summary>E0284 (type annotations needed): `parse` is generic over any `FromStr` type, and `+ 2` doesn't pin it down because many types implement `Add`. Write `"40".parse::<i32>()`, or annotate a binding: `let n: i32 = "40".parse().unwrap();`.</details>
<details><summary>d10 · Width counts what?</summary>Panics: `{:<5}` pads to 5 *scalars*, so the cell is "é    |", 6 scalars but 7 bytes ('é' is 2 bytes). Measure with `cell.chars().count()`. Neither count is a terminal column width: emoji and CJK characters often take two columns.</details>
