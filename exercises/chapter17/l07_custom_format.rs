//! LESSON 07 — Display, fmt::Write, and format_args!
//! Run: cargo test --test chapter17 l07_
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider: Can Display::fmt allocate freely? Must it propagate formatter
//! failures? Why does write! work with both String and I/O writers under different traits?
//!
//! IMPLEMENT the documented tasks, then keep the tests and add an edge case.
//! EXPLAIN afterward: Why is fmt::Error for formatting machinery failures rather than input
//! validation? How does format_args! pass formatting without an intermediate String?
//!
//! COMPILER / MANUAL LAB (use examples/ch17_scratch.rs):
//! Implement a wrapper for fmt::Write that always returns Err(fmt::Error), then call your
//! write_title function and verify propagation. Try storing format_args! with temporary
//! arguments beyond their lifetime; repair by using it in the call expression.
//!
//! TRANSFER: invent a new edge case, predict it, then add a check. A lesson is
//! complete only when its tests, predictions, lab, and explanation are done.

use std::fmt::{self, Write};
struct Progress {
    done: u32,
    total: u32,
}
impl fmt::Display for Progress {
    /// A. Render exactly "DONE/TOTAL"; alternate format renders "DONE of TOTAL".
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!("Task 1: implement the documented contract")
    }
}
/// B. Write "[" + supplied formatting arguments + "]" into any fmt::Write.
/// Propagate failure; do not allocate an intermediate String.
fn write_title<W: Write + ?Sized>(out: &mut W, args: fmt::Arguments<'_>) -> fmt::Result {
    todo!("Task 2: implement the documented contract")
}
#[cfg(test)]
mod tests {
    use super::*;
    use practice::{alloc::count, guess};
    #[test]
    fn a_display_supports_its_documented_alternate_form() {
        let p = Progress { done: 3, total: 8 };
        assert_eq!(format!("{p}"), "3/8");
        assert_eq!(format!("{p:#}"), "3 of 8");
        assert_eq!(p.to_string(), "3/8");
        let none = Progress { done: 0, total: 0 };
        assert_eq!(format!("{none}|{none:#}"), "0/0|0 of 0");
    }
    #[test]
    fn b_format_args_forwarding_uses_the_target_writer() {
        let mut text = String::from("prefix");
        write_title(&mut text, format_args!("{}={:.1}", "temp", 3.5)).unwrap();
        assert_eq!(text, "prefix[temp=3.5]");
    }
    #[test]
    fn b_write_errors_propagate() {
        struct Fail;
        impl Write for Fail {
            fn write_str(&mut self, _: &str) -> fmt::Result {
                Err(fmt::Error)
            }
        }
        assert!(write_title(&mut Fail, format_args!("x")).is_err());
    }

    /// Counts bytes and never allocates.
    struct Tally(usize);
    impl Write for Tally {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.0 += s.len();
            Ok(())
        }
    }

    #[test]
    fn b_write_title_does_not_build_a_string() {
        let mut out = Tally(0);
        let (result, allocations) =
            count(|| write_title(&mut out, format_args!("{}-{:>4}", "é", 7)));
        assert!(result.is_ok());
        assert_eq!(out.0, "[é-   7]".len());
        assert_eq!(
            allocations, 0,
            "forward the arguments; don't format! them first"
        );
    }

    struct Id(u32);
    impl fmt::Display for Id {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "#{}", self.0)
        }
    }
    struct Padded;
    impl fmt::Display for Padded {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.pad("ok")
        }
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    #[test]
    fn p_predictions() {
        // P1. `Id` writes with write!. Is the requested width applied?
        let p1: String = guess();
        assert_eq!(p1, format!("[{:>5}]", Id(7)), "P1: write! ignores width");

        // P2. `Padded` uses Formatter::pad instead.
        let p2: String = guess();
        assert_eq!(p2, format!("[{:>4}]", Padded), "P2: pad honors width");

        // P3. How many chars long is the Debug form of "a\tb"?
        let p3: usize = guess();
        assert_eq!(
            p3,
            format!("{:?}", "a\tb").chars().count(),
            "P3: Debug quotes and escapes"
        );
    }
}
