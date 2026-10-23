//! MILESTONE m08 — Modules, a public API, and doc tests (Chapter 8)
//! Run: cargo test --test m08_modules   and   cargo test --doc
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider: which paths can a user of the crate
//! name, and which items does `pub use` make reachable twice?
//! IMPLEMENT `src/error.rs`: `Display`, `source`, and the `From` conversions for `Error`.
//! DOCUMENT: run `cargo test --doc` (the examples in `src/num.rs` and `src/scan.rs`
//! must pass) and add a runnable example to two more public functions you wrote.
//! Run `cargo doc --no-deps --open` and read your crate as a user would.
//! EXPLAIN afterward: what does `?` do with an error of a different type? Why does a
//! library expose one error enum instead of `Box<dyn Error>`?

mod common;

use common::guess;
use loglens::prelude::*;
use loglens::scan::parse_fields;
use loglens::{Error, ErrorKind, FilterError, ParseError};
use std::error::Error as _;
use std::io::{self, Read};

fn io_error() -> io::Error {
    io::Error::other("disk on fire")
}

fn filter_error() -> FilterError {
    FilterError {
        term: "level>=loud".to_string(),
        reason: "unknown level",
    }
}

#[test]
fn a_error_display() {
    assert_eq!(Error::Io(io_error()).to_string(), "I/O error: disk on fire");
    let open = Error::Open {
        path: "logs/app.log".into(),
        source: io::Error::new(io::ErrorKind::NotFound, "not found"),
    };
    assert_eq!(open.to_string(), "logs/app.log: not found");
    let parse = Error::Parse(ParseError::new(ErrorKind::EmptyKey, 4).at_line(2));
    assert_eq!(parse.to_string(), "line 2, column 4: empty key");
    assert_eq!(
        Error::Filter(filter_error()).to_string(),
        r#"invalid filter: term "level>=loud": unknown level"#
    );
    assert_eq!(
        Error::Usage("missing command".to_string()).to_string(),
        "missing command"
    );
}

#[test]
fn b_error_source_chain() {
    let io = Error::Io(io_error());
    assert_eq!(io.source().unwrap().to_string(), "disk on fire");
    let parse = Error::Parse(ParseError::new(ErrorKind::EmptyKey, 1));
    assert!(parse.source().unwrap().is::<ParseError>());
    let filter = Error::Filter(filter_error());
    assert!(filter.source().unwrap().is::<FilterError>());
    let open = Error::Open {
        path: "x".into(),
        source: io_error(),
    };
    assert!(open.source().unwrap().is::<io::Error>());
    assert!(Error::Usage("x".into()).source().is_none());
}

struct Broken;

impl Read for Broken {
    fn read(&mut self, _: &mut [u8]) -> io::Result<usize> {
        Err(io_error())
    }
}

fn read_all(mut reader: impl Read) -> loglens::Result<String> {
    let mut text = String::new();
    reader.read_to_string(&mut text)?;
    Ok(text)
}

fn first_key(line: &str) -> loglens::Result<String> {
    let fields = parse_fields(line, line)?;
    Ok(fields
        .first()
        .map(|f| f.key.to_string())
        .unwrap_or_default())
}

fn check(valid: bool) -> loglens::Result<()> {
    if !valid {
        Err(filter_error())?;
    }
    Ok(())
}

#[test]
fn c_question_mark_converts_into_error() {
    assert_eq!(read_all(&b"ok"[..]).unwrap(), "ok");
    assert!(matches!(read_all(Broken), Err(Error::Io(_))));
    assert_eq!(first_key("user=ada").unwrap(), "user");
    match first_key("user=ada oops") {
        Err(Error::Parse(e)) => assert_eq!((e.kind, e.column), (ErrorKind::MissingEquals, 10)),
        other => panic!("expected a parse error, got {other:?}"),
    }
    assert!(check(true).is_ok());
    assert!(matches!(check(false), Err(Error::Filter(_))));
}

fn assert_error_type<E: std::error::Error + Send + Sync + 'static>() {}

/// Compile-time checks: the public paths documented in `src/lib.rs` exist and
/// name the same items.
#[test]
fn d_public_api() {
    assert_error_type::<Error>();
    assert_error_type::<ParseError>();
    assert_error_type::<FilterError>();
    let parse: fn(&str) -> Result<Record<'_>, ParseError> = loglens::parse_record;
    let same: fn(&str) -> Result<Record<'_>, ParseError> = loglens::record::parse_record;
    assert!(std::ptr::fn_addr_eq(parse, same));
    let _: loglens::level::Level = Level::Info;
    let _: Option<loglens::Result<()>> = None;
    let _: fn() -> Filter = Filter::all;
    let _: Option<(&Field<'_>, &dyn Sink, Summary, Group, Tally)> = None;
}

mod inner {
    pub fn path() -> &'static str {
        module_path!()
    }
}

/// P. Predict BEFORE running: replace each `guess()` with your answer.
#[test]
fn p_predictions() {
    // P1. `module_path!()` inside `mod inner` of this test crate (`m08_modules`).
    let p1: &str = guess();
    assert_eq!(p1, inner::path(), "P1: module paths start at the crate");

    // P2. Are `loglens::Level` and `loglens::prelude::Level` the same type?
    use std::any::TypeId;
    let p2: bool = guess();
    assert_eq!(
        p2,
        TypeId::of::<loglens::Level>() == TypeId::of::<loglens::prelude::Level>(),
        "P2: `pub use` re-exports, it does not copy"
    );

    // P3. The full path the compiler reports for `Level`.
    let p3: &str = guess();
    assert_eq!(
        p3,
        std::any::type_name::<loglens::Level>(),
        "P3: type_name shows where an item is defined"
    );
}
