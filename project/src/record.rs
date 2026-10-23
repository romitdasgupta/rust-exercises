//! Milestones m09 (Chapters 9–10), m11 (Display), and m17 (text).
//!
//! A [`Record`] is one parsed line. It does not own any text: the timestamp,
//! target, and fields borrow from the line, so a record cannot outlive the
//! line's buffer. Code that must keep data longer (m16 aggregation) copies
//! exactly what it needs.

use crate::error::{ErrorKind, ParseError};
use crate::level::Level;
use crate::num::NumError;
use crate::scan::Field;
use std::{borrow::Cow, fmt};

/// One parsed log line.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Record<'a> {
    /// `YYYY-MM-DDTHH:MM:SSZ`, validated by [`is_timestamp`]. Timestamps in
    /// this exact format sort in time order when compared as strings.
    pub timestamp: &'a str,
    /// The level.
    pub level: Level,
    /// The component that wrote the line, such as `auth`.
    pub target: &'a str,
    /// The fields, in line order. Keys may repeat.
    pub fields: Vec<Field<'a>>,
}

/// m09 C. Whether `s` is exactly `YYYY-MM-DDTHH:MM:SSZ`: 20 ASCII bytes,
/// digits where the format has letters, with month `01`–`12`, day `01`–`31`,
/// hour `00`–`23`, minute `00`–`59`, and second `00`–`60` (leap second).
/// The calendar is not checked further: `2026-02-31T00:00:00Z` is accepted.
///
/// Use a slice pattern on `s.as_bytes()` to name the twenty positions.
pub fn is_timestamp(s: &str) -> bool {
    todo!("m09 C: match the timestamp's shape")
}

/// m09 D. Parse one line: `TIMESTAMP LEVEL TARGET FIELD...`.
///
/// Words are separated by whitespace; leading and trailing whitespace is
/// ignored. Errors, with the 1-based character column they point at:
///
/// | Problem | Kind | Column |
/// | --- | --- | --- |
/// | nothing but whitespace | `Empty` | 1 |
/// | first word fails [`is_timestamp`] | `BadTimestamp` | start of that word |
/// | no second word | `MissingLevel` | the end: one past the line's last character |
/// | second word is not a level ([`Level::from_name`]) | `UnknownLevel` | start of that word |
/// | no third word | `MissingTarget` | the end: one past the line's last character |
/// | third word contains `=` | `MissingTarget` | start of that word |
/// | a field error | from [`crate::scan::parse_fields`] | start of that field |
///
/// "Start of a word" and "the end" are both [`crate::scan::column_of`] of
/// the text that remains when you look for the word (trim the line's start
/// first). Use [`crate::scan::split_word`], `let ... else`, and `?`. The
/// record borrows everything from `line`.
pub fn parse_record(line: &str) -> Result<Record<'_>, ParseError> {
    todo!("m09 D: assemble a record")
}

impl<'a> Record<'a> {
    /// m09 E. The raw value of the first field named `key` (see
    /// [`crate::scan::find_field`]). The result borrows from the line, not
    /// from the record.
    pub fn get(&self, key: &str) -> Option<&'a str> {
        todo!("m09 E: look up a field")
    }

    /// m09 F. The first field named `key` as a number, read with
    /// [`crate::num::parse_u64`] under [`crate::num::Overflow::Saturate`].
    /// `Ok(None)` if there is no such field.
    pub fn number(&self, key: &str) -> Result<Option<u64>, NumError> {
        todo!("m09 F: look up and parse a numeric field")
    }

    /// m17 D. The decoded text of the first field named `key` (see
    /// [`Field::text`]), or `None` if there is no such field.
    pub fn text(&self, key: &str) -> Option<Cow<'a, str>> {
        todo!("m17 D: look up and decode a field")
    }
}

/// m11 D. The canonical line: `TIMESTAMP LEVEL TARGET` followed by
/// ` key=raw` for each bare field or ` key="raw"` for each quoted one, with
/// single spaces. `parse_record(&record.to_string())` gives back an equal
/// record. Write straight into the formatter: no intermediate `String`s.
impl fmt::Display for Record<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!("m11 D: write the canonical line")
    }
}
