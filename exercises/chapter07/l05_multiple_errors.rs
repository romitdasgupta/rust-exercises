//! LESSON 05 — Multiple error types at a boundary
//! Run: cargo test --test chapter07 l05_
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider: How can one Result carry both an I/O error and an integer parse error? What static recovery information does Box<dyn Error> hide?
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
    use practice::{alloc::count, guess};
    use std::io::{self, ErrorKind};
    use std::num::ParseIntError;
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
        assert_eq!(read_count("4294967295\n".as_bytes()).unwrap(), u32::MAX);
        assert!(
            read_count("4294967296".as_bytes())
                .unwrap_err()
                .is::<ParseIntError>()
        );
    }
    #[test]
    fn a_io_error_keeps_its_kind() {
        let err = read_count(Broken).unwrap_err();
        let io = err.downcast_ref::<io::Error>().expect("an io::Error");
        assert_eq!(io.kind(), ErrorKind::PermissionDenied);
        let invalid_utf8: &[u8] = &[b'1', 0xff];
        assert!(read_count(invalid_utf8).unwrap_err().is::<io::Error>());
    }
    #[test]
    fn b_downcast_identifies_error_families() {
        assert_eq!(category(&Ok(1)), "ok");
        assert_eq!(category(&read_count(Broken)), "io");
        assert_eq!(category(&read_count("bad".as_bytes())), "parse");
        let other: AnyResult<u32> = Err("custom boundary failure".into());
        assert_eq!(category(&other), "other");
        let invalid_utf8: &[u8] = &[0xc3];
        assert_eq!(category(&read_count(invalid_utf8)), "io");
        let float: AnyResult<u32> = Err(Box::new("x".parse::<f64>().unwrap_err()));
        assert_eq!(category(&float), "other");
    }

    fn boxed(text: &str) -> AnyResult<u32> {
        Ok(text.parse::<u32>()?)
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    #[test]
    fn p_predictions() {
        // P1. How many machine words does a Box<dyn Error> take on the stack?
        let p1: usize = guess();
        assert_eq!(
            p1 * size_of::<usize>(),
            size_of::<Box<dyn Error>>(),
            "P1: a trait-object pointer is fat"
        );

        // P2. Heap allocations made by `boxed` on success, and on a parse error.
        let (good, ok_allocations) = count(|| boxed("12"));
        let (bad, err_allocations) = count(|| boxed("x"));
        assert!(good.is_ok() && bad.is_err());
        let p2: (usize, usize) = guess();
        assert_eq!(
            p2,
            (ok_allocations, err_allocations),
            "P2: what ? into Box<dyn Error> costs"
        );
    }
}
