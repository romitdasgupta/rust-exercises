//! # loglens
//!
//! A structured-log analyzer that grows milestone by milestone alongside
//! *Programming Rust*. The route, the milestone table, and the commands are
//! in `README.md`. This crate is the library; `src/main.rs` is the
//! `loglens` command-line program built on top of it.
//!
//! A log line has a timestamp, a level, a target (the component that wrote
//! it), and any number of `key=value` fields:
//!
//! ```text
//! 2026-10-23T17:03:00Z INFO auth user=ada latency_ms=42 msg="login ok"
//! ```
//!
//! This example runs as a doc test (`cargo test --doc`) once milestones
//! m03–m14 are done:
//!
//! ```
//! use loglens::prelude::*;
//!
//! let line = r#"2026-10-23T17:03:00Z INFO auth user=ada latency_ms=42 msg="login ok""#;
//! let record = parse_record(line)?;
//! assert_eq!(record.level, Level::Info);
//! assert_eq!(record.get("user"), Some("ada"));
//!
//! let filter = Filter::parse("level>=info latency_ms<100")?;
//! assert!(filter.matches(&record));
//! # Ok::<(), loglens::Error>(())
//! ```
//!
//! | Module | Milestone | Responsibility |
//! | --- | --- | --- |
//! | [`num`] | m03 | numeric fields and the overflow policy |
//! | [`scan`] | m04, m07 | zero-copy tokenizing of one line |
//! | [`error`] | m07, m08 | parse errors with columns; the crate error type |
//! | [`level`], [`record`] | m09, m11 | the `Level` enum and the `Record` struct |
//! | [`sink`] | m11 | where matching records go |
//! | [`filter`] | m14 | the filter language, compiled to closures |
//! | [`pipeline`] | m15, m18 | iterating and processing whole inputs |
//! | [`report`] | m16 | aggregation and the text report |
//! | [`text`] | m17 | quoting, unescaping, and truncation |
//! | [`follow`] | m18 | reading a file that is still being written |
//! | [`cli`] | m18 | argument parsing and the program's behavior |
//! | [`parallel`] | m19 | processing many files on worker threads |
//! | [`aio`] | m20 | an optional std-only async reader and executor |
//! | [`macros`] | m22 | `field!` and `log_line!` |
//! | [`fast`] | m23 | an optional unsafe byte search behind a safe API |
#![warn(missing_docs)]
// Starter scaffolding: the `todo!()` bodies leave parameters and imports
// unused. Delete the attribute below once every milestone is done, then fix
// what the compiler reports.
#![allow(unused_variables, unused_imports)]

pub mod aio;
pub mod cli;
pub mod error;
pub mod fast;
pub mod filter;
pub mod follow;
pub mod level;
pub mod macros;
pub mod num;
pub mod parallel;
pub mod pipeline;
pub mod record;
pub mod report;
pub mod scan;
pub mod sink;
pub mod text;

pub use error::{Error, ErrorKind, ParseError, Result};
pub use filter::{Filter, FilterError};
pub use level::Level;
pub use pipeline::Tally;
pub use record::{Record, parse_record};
pub use report::{Group, Summary};
pub use scan::Field;
pub use sink::Sink;

/// The names most programs need: `use loglens::prelude::*;`.
pub mod prelude {
    pub use crate::filter::Filter;
    pub use crate::level::Level;
    pub use crate::pipeline::Tally;
    pub use crate::record::{Record, parse_record};
    pub use crate::report::{Group, Summary};
    pub use crate::scan::Field;
    pub use crate::sink::Sink;
}
