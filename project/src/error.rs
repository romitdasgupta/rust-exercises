//! Milestones m07 (Chapter 7) and m08 (Chapter 8): errors.
//!
//! [`ParseError`] says what is wrong with one line and where. [`Error`] is
//! the crate-wide error type that the command-line program reports; the
//! `From` conversions let `?` turn every lower-level error into it.

use crate::filter::FilterError;
use std::{fmt, io, path::PathBuf};

/// What is wrong with a line.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ErrorKind {
    /// The line is blank where a record or field was required.
    Empty,
    /// The first word is not a `YYYY-MM-DDTHH:MM:SSZ` timestamp.
    BadTimestamp,
    /// The line ends after the timestamp.
    MissingLevel,
    /// The second word is not a level name.
    UnknownLevel,
    /// The line ends after the level, or the third word contains `=`.
    MissingTarget,
    /// A field has no `=` before the next whitespace.
    MissingEquals,
    /// A field starts with `=`.
    EmptyKey,
    /// A quoted value has no closing quote.
    UnterminatedQuote,
    /// Something other than whitespace follows a closing quote.
    TextAfterQuote,
    /// The line's bytes are not valid UTF-8 (reported by the readers in m18).
    InvalidUtf8,
}

/// m07 D. Display each kind as a short lowercase message:
///
/// | Kind | Message |
/// | --- | --- |
/// | `Empty` | `empty line` |
/// | `BadTimestamp` | `invalid timestamp` |
/// | `MissingLevel` | `missing level` |
/// | `UnknownLevel` | `unknown level` |
/// | `MissingTarget` | `missing target` |
/// | `MissingEquals` | `expected key=value` |
/// | `EmptyKey` | `empty key` |
/// | `UnterminatedQuote` | `unterminated quoted value` |
/// | `TextAfterQuote` | `unexpected text after closing quote` |
/// | `InvalidUtf8` | `line is not valid UTF-8` |
impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!("m07 D: one message per kind")
    }
}

impl std::error::Error for ErrorKind {}

/// A problem in one line: what, at which column, and (once known) which line.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError {
    /// What is wrong.
    pub kind: ErrorKind,
    /// 1-based character column where the problem starts.
    pub column: usize,
    /// 1-based line number, filled in by code that knows it (m15).
    pub line: Option<u64>,
}

impl ParseError {
    /// An error at `column` of a line whose number is not known yet.
    pub fn new(kind: ErrorKind, column: usize) -> Self {
        ParseError {
            kind,
            column,
            line: None,
        }
    }

    /// The same error, now known to be on line `line`.
    pub fn at_line(self, line: u64) -> Self {
        ParseError {
            line: Some(line),
            ..self
        }
    }
}

/// m07 E. `line 3, column 17: unterminated quoted value`, or without a line
/// number `column 17: unterminated quoted value`.
impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!("m07 E: describe the position and the kind")
    }
}

impl std::error::Error for ParseError {}

/// Everything that can go wrong in loglens, for callers who just want to
/// report it.
#[derive(Debug)]
pub enum Error {
    /// Reading or writing failed.
    Io(io::Error),
    /// A file could not be opened.
    Open {
        /// The file.
        path: PathBuf,
        /// Why it could not be opened.
        source: io::Error,
    },
    /// A line could not be parsed.
    Parse(ParseError),
    /// A filter expression is invalid.
    Filter(FilterError),
    /// The command line is invalid.
    Usage(String),
}

/// m08 A. Display:
///
/// | Variant | Text |
/// | --- | --- |
/// | `Io(e)` | `I/O error: {e}` |
/// | `Open { path, source }` | `{path}: {source}` (use `path.display()`) |
/// | `Parse(e)` | `{e}` |
/// | `Filter(e)` | `invalid filter: {e}` |
/// | `Usage(message)` | `{message}` |
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!("m08 A: one line per variant")
    }
}

/// m08 B. `source` returns the wrapped error for `Io`, `Open`, `Parse`, and
/// `Filter`, and `None` for `Usage`, so that callers can walk the chain.
impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        todo!("m08 B: expose the wrapped error")
    }
}

/// m08 C. Wrap an I/O error in [`Error::Io`].
impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        todo!("m08 C: wrap")
    }
}

/// m08 C. Wrap a parse error in [`Error::Parse`].
impl From<ParseError> for Error {
    fn from(error: ParseError) -> Self {
        todo!("m08 C: wrap")
    }
}

/// m08 C. Wrap a filter error in [`Error::Filter`].
impl From<FilterError> for Error {
    fn from(error: FilterError) -> Self {
        todo!("m08 C: wrap")
    }
}

/// The crate's result type: `Result<T>` means `Result<T, loglens::Error>`.
pub type Result<T, E = Error> = std::result::Result<T, E>;
