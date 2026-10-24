//! LESSON 08 — Capstone: fallible batch report
//! Run: cargo test --test chapter07 l08_
//!
//! PREDICT before running: What should a caller receive for syntax errors, zero values, and arithmetic overflow? Should a partial sum count as success?
//! IMPLEMENT the tasks below. Preserve the tests and add an edge case.
//! EXPLAIN afterward: Explain how typed errors support recovery, why the first failed row wins, and how a CLI main should convert this API into an exit status.
//! COMPILER / TRANSFER LAB: Create a standalone main returning Result<(), Box<dyn Error>>, invoke your report on command-line inputs, and observe stderr and exit status for good and bad inputs. Then return ExitCode explicitly for a stable user message. Keep process::exit out of chapter tests.
//! Use examples/ch07_scratch.rs; repair intentional failures before committing.

#[derive(Debug, PartialEq)]
enum ReportError {
    Invalid { row: usize },
    Zero { row: usize },
    Overflow { row: usize },
}
/// A. Parse a positive u16 for one row. Use supplied row number in the error.
fn parse_row(text: &str, row: usize) -> Result<u16, ReportError> {
    todo!("Implement this task")
}
/// B. Return (row_count,total) only if EVERY row is valid and total fits u16.
/// Stop at the first error; rows are one-based. Empty input succeeds (0,0).
fn report(rows: &[&str]) -> Result<(usize, u16), ReportError> {
    todo!("Implement this task")
}
#[cfg(test)]
mod tests {
    use super::*;
    use practice::alloc::count;
    #[test]
    fn a_row_errors_are_actionable() {
        assert_eq!(parse_row(" 4 ", 7), Ok(4));
        assert_eq!(parse_row("0", 7), Err(ReportError::Zero { row: 7 }));
        assert_eq!(parse_row("bad", 8), Err(ReportError::Invalid { row: 8 }));
        assert_eq!(parse_row("65535", 1), Ok(65535));
        assert_eq!(parse_row("65536", 2), Err(ReportError::Invalid { row: 2 }));
        assert_eq!(parse_row("-1", 3), Err(ReportError::Invalid { row: 3 }));
        assert_eq!(parse_row("", 4), Err(ReportError::Invalid { row: 4 }));
    }
    #[test]
    fn b_report_never_returns_partial_success() {
        assert_eq!(report(&["2", "3"]), Ok((2, 5)));
        assert_eq!(report(&[]), Ok((0, 0)));
        assert_eq!(
            report(&["2", "0", "bad"]),
            Err(ReportError::Zero { row: 2 })
        );
        assert_eq!(
            report(&["65535", "1"]),
            Err(ReportError::Overflow { row: 2 })
        );
        assert_eq!(report(&["1", "bad"]), Err(ReportError::Invalid { row: 2 }));
    }
    #[test]
    fn b_first_failure_wins_in_row_order() {
        assert_eq!(
            report(&["bad", "65535", "1"]),
            Err(ReportError::Invalid { row: 1 })
        );
        assert_eq!(
            report(&["65000", "500", "100", "0"]),
            Err(ReportError::Overflow { row: 3 })
        );
        assert_eq!(
            report(&["65535", "1", "bad"]),
            Err(ReportError::Overflow { row: 2 })
        );
        assert_eq!(report(&["65535"]), Ok((1, 65535)));
    }
    #[test]
    fn a_b_report_does_not_allocate() {
        let rows = ["1"; 300];
        let (result, allocations) = count(|| report(&rows));
        assert_eq!(result, Ok((300, 300)));
        assert_eq!(allocations, 0, "a one-pass report needs no buffer");
        let (result, allocations) = count(|| report(&["1", "2", "x"]));
        assert_eq!(result, Err(ReportError::Invalid { row: 3 }));
        assert_eq!(allocations, 0, "typed errors need no String");
    }
}
