//! Milestones m04 (Chapters 4–5) and m07 (Chapters 6–7): zero-copy scanning.
//!
//! Every slice these functions return borrows from the line you pass in.
//! Nothing is copied: a [`Field`] is three words of pointers and lengths into
//! the original text. The m04 tests check this with pointer ranges.
//!
//! The grammar of the part of a line after the target:
//!
//! ```text
//! fields   = field { whitespace field }
//! field    = key "=" value
//! key      = one or more bytes other than ASCII whitespace and "="
//! value    = quoted | bare
//! bare     = zero or more bytes other than ASCII whitespace
//! quoted   = '"' { escape | any byte other than '"' and '\' } '"'
//! escape   = '\' any character
//! ```
//!
//! "Whitespace" always means ASCII whitespace (`u8::is_ascii_whitespace`:
//! space, tab, `\n`, `\r`, form feed).

use crate::error::{ErrorKind, ParseError};
use std::borrow::Cow;

/// One `key=value` field, borrowed from the line it was scanned from.
///
/// For `msg="login ok"` the key is `msg`, `raw` is `login ok` (without the
/// quotes, escapes still encoded) and `quoted` is `true`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Field<'a> {
    /// The key, before the `=`.
    pub key: &'a str,
    /// The value as written: without surrounding quotes, escapes not decoded.
    pub raw: &'a str,
    /// Whether the value was written in double quotes.
    pub quoted: bool,
}

impl<'a> Field<'a> {
    /// m17 C. The decoded value.
    ///
    /// A bare value is returned as `Cow::Borrowed(self.raw)`. A quoted value
    /// is decoded with [`crate::text::unescape`]. The result borrows from the
    /// line (lifetime `'a`), not from this `Field`.
    pub fn text(&self) -> Cow<'a, str> {
        todo!("m17 C: decode the value")
    }
}

/// m04 A. Split off the first word.
///
/// Skip leading whitespace, then return the word (up to the next whitespace
/// byte or the end) and the rest of the text after it, with the rest's
/// leading whitespace removed. Return `None` if `s` is empty or only
/// whitespace. Both slices borrow from `s`.
///
/// ```
/// use loglens::scan::split_word;
///
/// assert_eq!(split_word("  INFO   auth x=1"), Some(("INFO", "auth x=1")));
/// assert_eq!(split_word("auth"), Some(("auth", "")));
/// assert_eq!(split_word(" \t"), None);
/// ```
pub fn split_word(s: &str) -> Option<(&str, &str)> {
    todo!("m04 A: borrow a word and the rest")
}

/// m04 B. Split off the first field.
///
/// Skip leading whitespace, then scan one field (see the module grammar).
/// Return the field and the rest of `s` after it, with the rest's leading
/// whitespace removed. Return `None` if `s` does not start with a valid field:
/// it is empty, has no `=` before the next whitespace, has an empty key, has
/// an unterminated quoted value, or has anything other than whitespace or the
/// end right after a closing quote.
///
/// In a quoted value, a backslash escapes the next character, so `\"` does
/// not close the value. Bare values may be empty (`key=`).
///
/// Refactor in m07: once [`scan_field`] exists, make this function a one-line
/// wrapper around it so there is only one scanner.
pub fn split_field(s: &str) -> Option<(Field<'_>, &str)> {
    todo!("m04 B: borrow one key=value field")
}

/// m04 C. The raw value of the first field named `key`.
///
/// Note the signature: the result lives as long as the *line* (`'a`), not as
/// long as the `fields` slice or the `key`. Callers may drop the vector of
/// fields and keep the value.
pub fn find_field<'a>(fields: &[Field<'a>], key: &str) -> Option<&'a str> {
    todo!("m04 C: search the fields")
}

/// m07 A. Like [`split_field`], but say what is wrong.
///
/// | Input (after leading whitespace) | Error |
/// | --- | --- |
/// | empty | [`ErrorKind::Empty`] |
/// | whitespace or end before any `=` | [`ErrorKind::MissingEquals`] |
/// | `=` is the first byte | [`ErrorKind::EmptyKey`] |
/// | no closing quote | [`ErrorKind::UnterminatedQuote`] |
/// | a byte other than whitespace right after the closing quote | [`ErrorKind::TextAfterQuote`] |
///
/// ```
/// use loglens::{ErrorKind, scan::scan_field};
///
/// let (field, rest) = scan_field(r#"msg="a \"b\"" user=ada"#)?;
/// assert_eq!((field.key, field.raw, field.quoted), ("msg", r#"a \"b\""#, true));
/// assert_eq!(rest, "user=ada");
/// assert_eq!(scan_field("user"), Err(ErrorKind::MissingEquals));
/// # Ok::<(), ErrorKind>(())
/// ```
pub fn scan_field(s: &str) -> Result<(Field<'_>, &str), ErrorKind> {
    todo!("m07 A: scan one field or say why not")
}

/// m07 B. The 1-based *character* column at which `suffix` starts in `line`.
///
/// `suffix` must be a tail of `line`, obtained by slicing it (as every
/// `rest` returned by this module is). `column_of(line, line)` is 1 and
/// `column_of(line, "")` is one past the last character. Count characters,
/// not bytes: in `"é x"`, the `x` is in column 3.
pub fn column_of(line: &str, suffix: &str) -> usize {
    todo!("m07 B: count characters before the suffix")
}

/// m07 C. Scan every field in `rest`, a tail of `line`.
///
/// Repeat [`scan_field`] until `rest` is empty (or only whitespace). Turn the
/// first failure into a [`ParseError`] whose column is where the failing field
/// starts (after whitespace), using [`column_of`] and the `?` operator. The
/// returned fields borrow from `rest`; `line` is only needed for columns,
/// which is why it has its own, unrelated lifetime.
pub fn parse_fields<'a>(line: &str, rest: &'a str) -> Result<Vec<Field<'a>>, ParseError> {
    todo!("m07 C: scan all fields, with columns")
}
