//! Milestone m14 (Chapter 14): a filter language compiled to closures.
//!
//! A filter is a list of terms separated by whitespace. A record matches
//! when it matches *every* term; an empty filter matches everything.
//!
//! ```text
//! level>=warn target=auth user!=bob latency_ms>100 msg~timeout ts>=2026-10-23T17:00:00Z
//! ```
//!
//! A term is `KEY OP VALUE`. The operator is found at the first byte of the
//! term that is one of `= ! < > ~`; the operator is then `!=`, `<=`, `>=`
//! (two bytes) or `=`, `<`, `>`, `~` (one byte). The key is everything before
//! it; the value is everything after it and may be empty (`user=`).
//!
//! | Key | Operators | Meaning |
//! | --- | --- | --- |
//! | `level` | `= != < <= > >=` | compare with a level name (any case, via `FromStr`) |
//! | `target` | `= != ~` | compare with the target; `~` means "contains" |
//! | `ts` | `= != < <= > >=` | compare timestamps as strings; the value must pass `is_timestamp` |
//! | any other key | `= !=` | the first field with that key equals / does not equal the value; a missing field never equals |
//! | any other key | `~` | the field exists and contains the value |
//! | any other key | `< <= > >=` | the field's `number()` is `Ok(Some(n))` and `n OP value`; the value must be a number (`parse_u64`, `Overflow::Reject`) |
//!
//! So `user!=bob` matches records without a `user` field, and
//! `latency_ms>100` never matches a record whose latency is missing or not a
//! number.
//!
//! Parse once, match many times: [`Filter::parse`] validates every term and
//! turns it into a boxed closure that owns the values it needs. Matching
//! performs no parsing of the filter and no allocation.

use crate::level::Level;
use crate::num::{Overflow, parse_u64};
use crate::record::{Record, is_timestamp};
use std::{fmt, str::FromStr};

/// One compiled term. `Send + Sync` lets threads share a filter (m19).
type Predicate = Box<dyn Fn(&Record<'_>) -> bool + Send + Sync>;

/// A compiled filter.
pub struct Filter {
    source: String,
    predicates: Vec<Predicate>,
}

/// Why a filter term was rejected.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FilterError {
    /// The whole offending term, such as `"level>=loud"`.
    pub term: String,
    /// One of the reasons listed on [`Filter::parse`].
    pub reason: &'static str,
}

impl fmt::Display for FilterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "term {:?}: {}", self.term, self.reason)
    }
}

impl std::error::Error for FilterError {}

impl Filter {
    /// m14 A. Compile a filter (grammar in the module documentation).
    ///
    /// Reject the first invalid term with one of these exact reasons:
    ///
    /// | Problem | `reason` |
    /// | --- | --- |
    /// | no operator, or `!` not followed by `=` | `"expected KEY OP VALUE"` |
    /// | the key is empty | `"empty key"` |
    /// | `level` value is not a level | `"unknown level"` |
    /// | `ts` value fails `is_timestamp` | `"expected a timestamp"` |
    /// | `<`, `<=`, `>`, `>=` value of a field term is not a number | `"expected a number"` |
    /// | operator not allowed for this key (e.g. `level~x`, `target>a`) | `"unsupported operator"` |
    ///
    /// Check in this order: operator, key, operator allowed for the key, value.
    /// So `level~loud` is an unsupported operator, not an unknown level.
    ///
    /// Each term becomes one boxed predicate: a `move` closure that captures
    /// owned copies of what it compares against (a `Level`, a `String`, a
    /// `u64`). Write a small private helper per key kind rather than one
    /// giant closure.
    pub fn parse(source: &str) -> Result<Filter, FilterError> {
        todo!("m14 A: compile terms to closures")
    }

    /// m14 B. Whether `record` satisfies every term.
    pub fn matches(&self, record: &Record<'_>) -> bool {
        todo!("m14 B: run the closures")
    }

    /// The text the filter was parsed from, unchanged.
    pub fn source(&self) -> &str {
        &self.source
    }

    /// A filter that matches every record.
    pub fn all() -> Filter {
        Filter {
            source: String::new(),
            predicates: Vec::new(),
        }
    }
}

impl Default for Filter {
    fn default() -> Self {
        Filter::all()
    }
}

impl fmt::Debug for Filter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Filter")
            .field("source", &self.source)
            .field("terms", &self.predicates.len())
            .finish()
    }
}

impl FromStr for Filter {
    type Err = FilterError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Filter::parse(s)
    }
}
