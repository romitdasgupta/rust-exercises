//! Milestone m17 (Chapter 17): quoted values and Unicode-safe truncation.
//!
//! Each function returns `Cow<str>`: it borrows its input when no change is
//! needed (the common case) and allocates only when it must build new text.

use std::borrow::Cow;

/// m17 A. Decode the escapes in a quoted value's raw text.
///
/// `\"` is `"`, `\\` is `\`, `\n` is a newline, `\r` a carriage return,
/// `\t` a tab. A backslash before any other character is kept together with
/// that character (`\x` stays `\x`), and a backslash at the very end is kept.
/// Without any backslash, return `Cow::Borrowed(raw)`.
pub fn unescape(raw: &str) -> Cow<'_, str> {
    todo!("m17 A: decode escapes")
}

/// m17 B. Write `value` so that it scans back as one field value.
///
/// Return `Cow::Borrowed(value)` when it is non-empty and contains no
/// whitespace (`char::is_whitespace`, which includes Unicode spaces), `"`,
/// `\`, or `=`. Otherwise return it in double quotes, escaping `\` as `\\`,
/// `"` as `\"`, newline as `\n`, carriage return as `\r`, and tab as `\t`.
///
/// For every string `s`, scanning `key=` followed by `quote(s)` gives a field
/// whose `text()` is `s`.
pub fn quote(value: &str) -> Cow<'_, str> {
    todo!("m17 B: quote when needed")
}

/// m17 E. Shorten `s` to at most `max_chars` characters (`char`s, not
/// bytes).
///
/// If `s` fits, return it borrowed. Otherwise keep the first `max_chars - 1`
/// characters and append `…` (U+2026), so the result has exactly
/// `max_chars` characters. `max_chars == 0` gives `""`. Never split a
/// character; a `char` is a code point, so a letter written with a combining
/// accent counts as two.
pub fn truncate(s: &str, max_chars: usize) -> Cow<'_, str> {
    todo!("m17 E: cut on a character boundary")
}
