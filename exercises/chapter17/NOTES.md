# Chapter 17 notes — Strings and Text

Your workbook. Write answers in your own words, *before* checking the hints. When `cargo progress` says this chapter is due for review, answer the mastery questions at the bottom again from memory, then compare with what you wrote here.

## l01 · Unicode scalar operations and encoding

**Predictions I got wrong, and why:**

- 

**Explain:** Why can case conversion yield multiple scalars? Why is reversing logical scalar order unsuitable as a text-direction algorithm?

> 

**Edge case I added:**

- 

## l02 · Constructing and editing owned text

**Predictions I got wrong, and why:**

- 

**Explain:** Why must insertion boundaries be checked? Which operations keep the original allocation versus return a new String?

> 

**Edge case I added:**

- 

## l03 · Search patterns, byte positions, and replacement

**Predictions I got wrong, and why:**

- 

**Explain:** How does a literal string pattern differ from a char or character predicate? Why does replacen need an explicit replacement count?

> 

**Edge case I added:**

- 

## l04 · Text iteration, trimming, and casing

**Predictions I got wrong, and why:**

- 

**Explain:** Why is lowercasing not a universal Unicode case-insensitive comparison? Why do chars and bytes answer different questions?

> 

**Edge case I added:**

- 

## l05 · FromStr and textual conversion

**Predictions I got wrong, and why:**

- 

**Explain:** Why should a parser return an error rather than quietly truncate? How do parse and to_string use traits?

> 

**Edge case I added:**

- 

## l06 · The formatting language

**Predictions I got wrong, and why:**

- 

**Explain:** How do named/indexed arguments and dynamic parameters make a formatter reusable? Why should pointer addresses never be hard-coded in tests?

> 

**Edge case I added:**

- 

## l07 · Display, fmt::Write, and format_args!

**Predictions I got wrong, and why:**

- 

**Explain:** Why is fmt::Error for formatting machinery failures rather than input validation? How does format_args! pass formatting without an intermediate String?

> 

**Edge case I added:**

- 

## l08 · UTF-8, borrowed text views, and delayed allocation

**Predictions I got wrong, and why:**

- 

**Explain:** Why should OS strings and paths not be forced through to_str().unwrap()? Which conversions guarantee UTF-8 and which preserve arbitrary bytes?

> 

**Edge case I added:**

- 

## l09 · Capstone: a Unicode-aware report parser

**Predictions I got wrong, and why:**

- 

**Explain:** Why are byte offsets safe at separator boundaries? What errors should a display-oriented parser preserve rather than ignore?

> 

**Edge case I added:**

- 

## Drills

Drills whose verdict surprised me, and the rule I learned:

- 

## Mastery check (answer from memory)

**Distinguish bytes, Unicode scalar values, grapheme clusters, display columns, and logical text order.**

> 

**Choose byte-safe slicing and identify when searching returns byte positions.**

> 

**Handle case expansion and invalid UTF-8 without losing ownership accidentally.**

> 

**Implement FromStr and Display with clear contracts and formatter error propagation.**

> 

**Build a regex once with LazyLock and choose is_match, find, or captures by required information.**

> 

**Redo the capstone from its contract after a few days without consulting your earlier implementation.**

> 

**Present your predictions, failed compiler experiments, and one new edge case for review.**

>
