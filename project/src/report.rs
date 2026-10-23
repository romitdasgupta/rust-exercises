//! Milestone m16 (Chapters 12 and 16): aggregation and the text report.
//!
//! A [`Summary`] keeps counters, not records: per level, and per *group*
//! (the record's target, its level, or the value of one field). It owns its
//! group names, so it can outlive the lines it was built from.

use crate::level::Level;
use crate::record::Record;
use crate::sink::Sink;
use std::collections::{BTreeMap, BinaryHeap, HashMap};
use std::{cmp::Reverse, fmt, io, ops::AddAssign};

/// The numeric field whose values the report aggregates.
pub const LATENCY_KEY: &str = "latency_ms";

/// What the report groups records by.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Group {
    /// The record's target.
    Target,
    /// The record's level (shown as its canonical name).
    Level,
    /// The raw value of the first field with this key; `-` when missing.
    Field(String),
}

impl Group {
    /// m16 A. The group name of `record`: its target, its level's
    /// [`Level::as_str`], or the raw field value (`"-"` if the field is
    /// missing). The name borrows from the line (or is `'static`).
    pub fn key_of<'a>(&self, record: &Record<'a>) -> &'a str {
        todo!("m16 A: name the record's group")
    }
}

/// m16 B. `"target"` is [`Group::Target`], `"level"` is [`Group::Level`],
/// anything else is a field name.
impl From<&str> for Group {
    fn from(name: &str) -> Self {
        todo!("m16 B: choose a grouping")
    }
}

impl fmt::Display for Group {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Group::Target => f.write_str("target"),
            Group::Level => f.write_str("level"),
            Group::Field(key) => f.write_str(key),
        }
    }
}

/// Counters for one group.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct GroupStats {
    /// Records in the group.
    pub count: u64,
    /// Records at [`Level::Error`].
    pub errors: u64,
    /// Saturating sum of the valid [`LATENCY_KEY`] values.
    pub latency_sum: u64,
    /// Number of valid [`LATENCY_KEY`] values.
    pub latency_count: u64,
    /// The largest valid [`LATENCY_KEY`] value.
    pub latency_max: Option<u64>,
}

impl GroupStats {
    /// m16 C. The mean latency ([`crate::num::mean`]), `None` without values.
    pub fn mean_latency(&self) -> Option<f64> {
        todo!("m16 C: mean of the latencies")
    }
}

/// m16 D. Merge another group's counters into this one (sums saturate).
impl AddAssign<&GroupStats> for GroupStats {
    fn add_assign(&mut self, other: &GroupStats) {
        todo!("m16 D: merge counters")
    }
}

/// Aggregated counters for many records.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Summary {
    /// What the groups are.
    pub group: Group,
    /// Records added.
    pub records: u64,
    /// Records per level. Only levels that occurred have an entry.
    pub levels: BTreeMap<Level, u64>,
    /// Counters per group name.
    pub groups: HashMap<String, GroupStats>,
}

impl Summary {
    /// An empty summary.
    pub fn new(group: Group) -> Self {
        Summary {
            group,
            records: 0,
            levels: BTreeMap::new(),
            groups: HashMap::new(),
        }
    }

    /// m16 E. Count one record: `records`, its level, and its group's
    /// `count`, `errors`, and latency (a [`LATENCY_KEY`] value that is
    /// missing or not a number is ignored).
    ///
    /// Allocate the group name only the first time a group appears: look it
    /// up by `&str` before inserting an owned `String`.
    pub fn add(&mut self, record: &Record<'_>) {
        todo!("m16 E: count one record")
    }

    /// m16 F. The `k` largest groups by `count`, largest first; equal counts
    /// in ascending name order. Fewer than `k` if there are fewer groups.
    ///
    /// Use a [`BinaryHeap`] that never holds more than `k` entries (with
    /// [`Reverse`] it is a min-heap: pop the worst when it grows past `k`),
    /// so the cost is O(n log k), not a full sort.
    pub fn top(&self, k: usize) -> Vec<(&str, &GroupStats)> {
        todo!("m16 F: top k with a bounded heap")
    }
}

/// m16 G. Merge another summary into this one, as if all of its records had
/// been added here. Both must have the same `group` (`debug_assert_eq!`).
impl AddAssign for Summary {
    fn add_assign(&mut self, other: Summary) {
        todo!("m16 G: merge summaries")
    }
}

/// m16 H. A summary is a sink: `accept` calls [`Summary::add`].
impl Sink for Summary {
    fn accept(&mut self, record: &Record<'_>) -> io::Result<()> {
        todo!("m16 H: add the record")
    }
}

/// m16 I. The report printed by `loglens report`. Every line ends in `\n`:
///
/// ```text
/// records: 8
/// levels: TRACE=0 DEBUG=1 INFO=4 WARN=1 ERROR=2
/// top 10 by target:
///   auth    count=4 errors=1 mean_latency_ms=122.0 max_latency_ms=310
///   db      count=2 errors=1 mean_latency_ms=2508.0 max_latency_ms=5012
///   billing count=1 errors=0 mean_latency_ms=- max_latency_ms=-
/// ```
///
/// - `levels:` lists all five levels in order, including zeros.
/// - The heading shows the requested `top` and the group ([`Group`]'s
///   `Display`), even when fewer groups exist.
/// - Rows come from [`Summary::top`]. Names are left-aligned and padded to
///   the longest name among the rows shown. The mean has one decimal; a
///   missing mean or maximum is `-`.
/// - With no rows, the heading is followed by the line `  (none)`.
pub fn render(summary: &Summary, top: usize) -> String {
    todo!("m16 I: format the report")
}
