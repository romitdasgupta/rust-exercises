//! LESSON 07 — Display, fmt::Write, and format_args!
//! Run: cargo test --test chapter17 l07_
//!
//! PREDICT before executing: Can Display::fmt allocate freely? Must it propagate formatter
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
    #[test]
    fn a_display_supports_its_documented_alternate_form() {
        let p = Progress { done: 3, total: 8 };
        assert_eq!(format!("{p}"), "3/8");
        assert_eq!(format!("{p:#}"), "3 of 8");
        assert_eq!(p.to_string(), "3/8");
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
}
