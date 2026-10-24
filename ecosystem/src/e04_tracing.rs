//! LESSON E04 — Structured logging with tracing (after Chapter 18)
//! Run: cargo test --manifest-path ecosystem/Cargo.toml e04_
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider: Who decides where an event goes: the
//! code that emits it, or the subscriber? Is an event's message formatted when its
//! level is disabled?
//! IMPLEMENT the tasks below. Preserve the tests and add an edge case.
//! EXPLAIN afterward: Explain the difference between a span and an event, and why a
//! library should emit tracing data but never install a global subscriber.
//! JUDGMENT: `tracing` is the default for services, especially async ones, because
//! spans carry context across `.await`. The `log` crate is simpler and still common in
//! libraries; tracing-subscriber can collect `log` records too. Plain `eprintln!` is
//! fine for small CLIs.

use std::io;
use std::sync::{Arc, Mutex};
use tracing::Level;
use tracing_subscriber::fmt::MakeWriter;

/// A. A cloneable in-memory sink for formatted log output. Every clone appends to
/// the same buffer, so the test keeps one clone and gives another to the subscriber.
#[derive(Clone, Default)]
pub struct Capture {
    buf: Arc<Mutex<Vec<u8>>>,
}

impl Capture {
    pub fn new() -> Capture {
        Capture::default()
    }

    /// A. Everything written so far, as text (lossy UTF-8 is fine).
    pub fn contents(&self) -> String {
        todo!("Implement this task")
    }
}

/// A. Writes append to the shared buffer.
impl io::Write for Capture {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        todo!("Implement this task")
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

/// A. The fmt subscriber asks its `MakeWriter` for a fresh writer for each event.
impl<'a> MakeWriter<'a> for Capture {
    type Writer = Capture;

    fn make_writer(&'a self) -> Capture {
        todo!("Implement this task")
    }
}

/// B. Run `f` with a fmt subscriber that writes to a fresh `Capture` and return
/// `(f's result, captured text)`. Format: no ANSI colors, no timestamps, no targets;
/// only events at `max_level` or more severe. The subscriber must be active only on
/// this thread and only while `f` runs: use `tracing::subscriber::with_default`, never
/// a global default (tests run in parallel in one process).
pub fn capture<R>(max_level: Level, f: impl FnOnce() -> R) -> (R, String) {
    todo!("Implement this task")
}

/// C. Check credentials (the correct password is `"hunter2"`). Contract:
/// - runs inside a span named `login` with exactly one field, `user`, recorded with
///   Display (`login{user=alice}`); the password must never appear in any output;
/// - emits INFO `login attempt`, and on failure WARN `login failed`.
pub fn check_login(username: &str, password: &str) -> bool {
    todo!("Implement this task")
}

/// D. Sum the non-negative items of a batch inside a span named `batch` with fields
/// `id` and `total`, where `total` is empty at first and recorded once at the end.
/// - Each negative item: WARN `skipping negative item` with field `item`.
/// - Each other item: DEBUG `item ok` with field `item`.
/// - Finally record `total` on the span, then INFO `batch done`. Return the total.
pub fn process_batch(id: u32, items: &[i64]) -> i64 {
    todo!("Implement this task")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::guess;
    use std::io::Write as _;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[test]
    fn a_clones_share_one_buffer() {
        let sink = Capture::new();
        let mut a = sink.clone();
        let mut b = sink.make_writer();
        a.write_all(b"one ").unwrap();
        b.write_all(b"two").unwrap();
        assert_eq!(sink.contents(), "one two");
        assert_eq!(Capture::new().contents(), "");
    }

    #[test]
    fn b_capture_formats_events_without_noise() {
        let (value, out) = capture(Level::INFO, || {
            tracing::info!(user = "ann", "hello");
            tracing::debug!("hidden");
            7
        });
        assert_eq!(value, 7);
        assert_eq!(out, " INFO hello user=\"ann\"\n");
    }

    #[test]
    fn b_capture_is_scoped() {
        let (_, out) = capture(Level::TRACE, || tracing::trace!("inside"));
        assert!(out.contains("inside"));
        tracing::error!("outside: nobody is listening");
        let (_, out) = capture(Level::ERROR, || tracing::warn!("too quiet"));
        assert_eq!(out, "");
    }

    #[test]
    fn c_login_span_has_user_and_no_secret() {
        let (ok, out) = capture(Level::INFO, || check_login("alice", "s3cr3t-value"));
        assert!(!ok);
        assert!(
            out.contains("INFO login{user=alice}: login attempt"),
            "{out}"
        );
        assert!(
            out.contains("WARN login{user=alice}: login failed"),
            "{out}"
        );
        assert!(!out.contains("s3cr3t"), "password leaked: {out}");
        let (ok, out) = capture(Level::INFO, || check_login("bob", "hunter2"));
        assert!(ok);
        assert_eq!(out.lines().count(), 1, "{out}");
        assert!(!out.contains("hunter2"), "password leaked: {out}");
    }

    #[test]
    fn d_batch_records_fields_and_respects_levels() {
        let (total, out) = capture(Level::INFO, || process_batch(7, &[2, -3, 3]));
        assert_eq!(total, 5);
        let lines: Vec<&str> = out.lines().collect();
        assert_eq!(
            lines,
            [
                " WARN batch{id=7}: skipping negative item item=-3",
                " INFO batch{id=7 total=5}: batch done",
            ]
        );
        let (_, out) = capture(Level::DEBUG, || process_batch(8, &[1]));
        assert!(out.contains("DEBUG batch{id=8}: item ok item=1"), "{out}");
        let (total, out) = capture(Level::INFO, || process_batch(9, &[]));
        assert_eq!(total, 0);
        assert!(out.contains("batch{id=9 total=0}: batch done"), "{out}");
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    /// These use their own subscriber, so they work before your tasks are done.
    #[test]
    fn p_predictions() {
        let quiet = || {
            tracing_subscriber::fmt()
                .with_max_level(Level::WARN)
                .with_writer(io::sink)
                .finish()
        };

        // P1. A scoped subscriber is active on this thread. Is WARN enabled on a
        // thread spawned inside the scope?
        let on_spawned = tracing::subscriber::with_default(quiet(), || {
            std::thread::spawn(|| tracing::enabled!(Level::WARN))
                .join()
                .unwrap()
        });
        let p1: bool = guess();
        assert_eq!(p1, on_spawned, "P1: with_default is thread-local");

        // P2. With max level WARN, how many times is `expensive()` called?
        let calls = AtomicUsize::new(0);
        let expensive = || {
            calls.fetch_add(1, Ordering::SeqCst);
            "details"
        };
        tracing::subscriber::with_default(quiet(), || {
            tracing::info!("state: {}", expensive());
            tracing::info!(detail = expensive(), "state");
        });
        let p2: usize = guess();
        assert_eq!(
            p2,
            calls.load(Ordering::SeqCst),
            "P2: disabled events skip their arguments"
        );

        // P3. In tracing, is `Level::ERROR > Level::WARN`?
        let p3: bool = guess();
        assert_eq!(
            p3,
            Level::ERROR > Level::WARN,
            "P3: levels compare by verbosity"
        );
    }
}
