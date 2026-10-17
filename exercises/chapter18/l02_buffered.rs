//! LESSON 02 — Buffered text
//! Run: cargo test --test chapter18 l02_
//!
//! PREDICT before running:
//! 1. Does read_line clear its String argument?
//! 2. What do lines() and read_line do to CRLF?
//!
//! IMPLEMENT the tasks; keep the tests.
//! EXPLAIN: Why can byte records succeed where text lines return InvalidData?
//! COMPILER LAB (examples/ch18_scratch.rs): Read the same Cursor with read_line, lines, and read_until. Include CRLF, no final newline, and invalid UTF-8.

use std::io::{self, BufRead};
/// A. Collect UTF-8 lines, discarding empty lines and lines whose first byte
/// is '#'. Keep all other whitespace. BufRead::lines removes LF/CRLF.
fn data_lines<R: BufRead>(input: R) -> io::Result<Vec<String>> {
    todo!("A: implement the contract")
}
/// B. Collect byte records split on zero. Keep empty records between delimiters;
/// omit a phantom record after a final delimiter (BufRead::split behavior).
fn nul_records<R: BufRead>(input: R) -> io::Result<Vec<Vec<u8>>> {
    todo!("B: implement the contract")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn text_preserves_spaces_and_accepts_unterminated_final_line() {
        assert_eq!(
            data_lines(&b"#meta\r\n\n a \r\nlast"[..]).unwrap(),
            [" a ", "last"]
        );
        assert!(data_lines(&b""[..]).unwrap().is_empty());
        assert_eq!(
            data_lines(&b"\xff\n"[..]).unwrap_err().kind(),
            io::ErrorKind::InvalidData
        );
    }
    #[test]
    fn byte_records_need_not_be_unicode() {
        assert_eq!(
            nul_records(&b"a\0\0\xff\0"[..]).unwrap(),
            vec![b"a".to_vec(), vec![], vec![255]]
        );
        assert!(nul_records(&b""[..]).unwrap().is_empty());
    }
}
