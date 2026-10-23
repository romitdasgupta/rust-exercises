//! Milestone m22 (Chapter 22): `field!` and `log_line!`.
//!
//! Both macros are exported at the crate root (`loglens::field!`,
//! `loglens::log_line!`). The *patterns* (what callers may write) are given
//! so the tests compile from the start; you write the *expansions*. Each
//! starter expansion is a typed placeholder that panics. (It mentions the
//! arguments inside a closure that is never called, only so that the call
//! sites compile without unused-variable warnings.)
//!
//! Expansion rules for macros used outside this crate:
//!
//! - Refer to this crate's items through `$crate::...`, never by a bare
//!   name, so a caller's own `quote` or `text` cannot capture them.
//! - Refer to std through absolute paths (`::std::...`).
//! - Evaluate each argument exactly once.

/// The placeholder used by the starter expansions.
#[doc(hidden)]
#[track_caller]
pub fn unfinished<T>() -> T {
    todo!("m22: write this macro's expansion")
}

/// m22 A. Read a field from a [`crate::Record`] (or a reference to one).
///
/// - `field!(record, user)` is `record.get("user")`: an `Option<&str>`.
/// - `field!(record, "content-type")` is the same for keys that are not
///   identifiers.
/// - `field!(record, latency_ms as u64)` is the number, or `None` if the
///   field is missing or not a number: an `Option<u64>`.
///
/// Use `stringify!` to turn the identifier into the key, and a method call
/// on the record expression so both `record` and `&record` work.
#[macro_export]
macro_rules! field {
    ($record:expr, $key:ident as u64) => {{
        let _never_called = || {
            let _ = &$record;
        };
        $crate::macros::unfinished::<::core::option::Option<u64>>()
    }};
    ($record:expr, $key:ident) => {{
        let _never_called = || {
            let _ = &$record;
        };
        $crate::macros::unfinished::<::core::option::Option<&str>>()
    }};
    ($record:expr, $key:literal) => {{
        let _never_called = || {
            let _ = &$record;
        };
        $crate::macros::unfinished::<::core::option::Option<&str>>()
    }};
}

/// m22 B. Build a log line as a `String`, for tests and examples.
///
/// ```text
/// log_line!("2026-10-23T17:03:00Z", Info, "auth", user = "ada", latency_ms = 42)
/// ```
///
/// gives `2026-10-23T17:03:00Z INFO auth user=ada latency_ms=42`. The
/// timestamp and target are any `Display` expressions; the level is a
/// [`crate::Level`] variant name. Each value is any `Display` expression,
/// formatted and then passed through [`crate::text::quote`]. A trailing
/// comma is allowed. Hint: build a `String` with `write!` inside a block
/// expression; a `$( ... )*` repetition emits one statement per field.
#[macro_export]
macro_rules! log_line {
    ($ts:expr, $level:ident, $target:expr $(, $key:ident = $value:expr)* $(,)?) => {{
        let _never_called = || {
            let _ = (&$ts, &$target $(, &$value)*);
        };
        $crate::macros::unfinished::<::std::string::String>()
    }};
}
