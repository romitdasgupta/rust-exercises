//! LESSON 05 — Multiple error types at a boundary
//! Run: cargo test --test chapter07 l05_
//!
//! PREDICT before running: How can one Result carry both an I/O error and an integer parse error? What static recovery information does Box<dyn Error> hide?
//! IMPLEMENT the tasks below. Preserve the tests and add an edge case.
//! EXPLAIN afterward: Explain ? conversion into a boxed error and compare a concrete error enum with a universal application-boundary error.
//! COMPILER / TRANSFER LAB: In a separate optional package, add anyhow and write a main returning anyhow::Result. Use Context on read/parse failures and inspect the chain; compare with Box<dyn Error>. This lab needs a cached crate or network; core tasks do not. Do not add dependencies to this learner package.
//! Use examples/ch07_scratch.rs; repair intentional failures before committing.

use std::error::Error;
use std::io::Read;
type AnyResult<T> = Result<T, Box<dyn Error>>;
/// A. Read all UTF-8 text from reader and parse a trimmed u32. Propagate I/O
/// and parse errors with ? into Box<dyn Error>. Do not stringify the errors.
fn read_count(mut reader: impl Read) -> AnyResult<u32> {
    todo!("Implement this task")
}
/// B. For either success or error, return a stable category: "ok", "io",
/// "parse", or "other", recognizing concrete boxed error types by downcast.
fn category(result: &AnyResult<u32>) -> &'static str {
    todo!("Implement this task")
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{self, ErrorKind};
    struct Broken;
    impl Read for Broken {
        fn read(&mut self, _: &mut [u8]) -> io::Result<usize> {
            Err(io::Error::new(ErrorKind::PermissionDenied, "blocked"))
        }
    }
    #[test]
    fn a_boundary_preserves_sources() {
        assert_eq!(read_count(" 17 ".as_bytes()).unwrap(), 17);
        assert!(
            read_count("bad".as_bytes())
                .unwrap_err()
                .is::<std::num::ParseIntError>()
        );
        assert!(read_count(Broken).unwrap_err().is::<io::Error>());
    }
    #[test]
    fn b_downcast_identifies_error_families() {
        assert_eq!(category(&Ok(1)), "ok");
        assert_eq!(category(&read_count(Broken)), "io");
        assert_eq!(category(&read_count("bad".as_bytes())), "parse");
        let other: AnyResult<u32> = Err("custom boundary failure".into());
        assert_eq!(category(&other), "other");
    }
}
