//! LESSON 09 — Capstone: a Unicode-aware report parser
//! Run: cargo test --test chapter17 l09_
//!
//! PREDICT before running: Predict parsing, rejection, byte/scalar counts, and output for
//! non-ASCII names. Decide whether a user-perceived character limit is in scope.
//!
//! IMPLEMENT the documented tasks, then keep the tests and add an edge case.
//! EXPLAIN afterward: Why are byte offsets safe at separator boundaries? What errors should
//! a display-oriented parser preserve rather than ignore?
//!
//! COMPILER / MANUAL LAB (use examples/ch17_scratch.rs):
//! Extend the format with escaped separators and compare a small parser to the isolated
//! regex lab. Explain why this task does not implement Unicode normalization, grapheme
//! segmentation, or bidirectional layout.
//!
//! TRANSFER: invent a new edge case, predict it, then add a check. A lesson is
//! complete only when its tests, predictions, lab, and explanation are done.

#[derive(Debug, PartialEq, Eq)]
struct Record {
    name: String,
    value: u16,
}
/// A. Parse trimmed NAME=VALUE; name must be nonblank, value valid u16.
/// Reject extra '='. Return "invalid record" for every malformed input.
fn parse_record(line: &str) -> Result<Record, &'static str> {
    todo!("Task 1: implement the documented contract")
}
/// B. Read nonblank lines, ignoring lines whose trimmed text starts '#'.
/// Preserve valid record order; record 1-based source line numbers for rejects.
fn read_report(text: &str) -> (Vec<Record>, Vec<usize>) {
    todo!("Task 2: implement the documented contract")
}
/// C. Render each row as "NAME: VALUE (N scalars)\n". Empty report is empty text.
fn render(records: &[Record]) -> String {
    todo!("Task 3: implement the documented contract")
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a_parser_preserves_unicode_and_validates_entire_record() {
        assert_eq!(
            parse_record(" é = 42 "),
            Ok(Record {
                name: "é".into(),
                value: 42
            })
        );
        for input in [
            "=1",
            "a=65536",
            "a=-1",
            "a=1=2",
            "missing",
            "a=",
            " \u{2003}=1",
        ] {
            assert_eq!(parse_record(input), Err("invalid record"), "{input}");
        }
    }
    #[test]
    fn b_batch_parser_reports_original_lines() {
        let (rows, bad) = read_report("# title\n é=2\n\ninvalid\n🦀=3\na=1=2\n");
        assert_eq!(
            rows,
            vec![
                Record {
                    name: "é".into(),
                    value: 2
                },
                Record {
                    name: "🦀".into(),
                    value: 3
                }
            ]
        );
        assert_eq!(bad, [4, 6]);
        let (rows, bad) = read_report("  # indented comment\r\nx = 1\r\n\r\n#=2\n");
        assert_eq!(
            rows,
            [Record {
                name: "x".into(),
                value: 1
            }]
        );
        assert!(bad.is_empty());
        assert_eq!(read_report(""), (vec![], vec![]));
    }
    #[test]
    fn c_output_uses_scalar_counts_and_exact_line_endings() {
        assert_eq!(
            render(&[
                Record {
                    name: "e\u{301}".into(),
                    value: 5
                },
                Record {
                    name: "🦀".into(),
                    value: 0
                }
            ]),
            "e\u{301}: 5 (2 scalars)\n🦀: 0 (1 scalars)\n"
        );
        assert_eq!(render(&[]), "");
    }
}
