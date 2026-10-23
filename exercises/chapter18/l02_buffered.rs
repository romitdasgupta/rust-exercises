//! LESSON 02 — Buffered text
//! Run: cargo test --test chapter18 l02_
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider:
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
    use practice::guess;
    use std::io::Cursor;
    #[test]
    fn a_text_preserves_spaces_and_accepts_unterminated_final_line() {
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
    fn a_only_a_leading_hash_byte_marks_a_comment() {
        assert_eq!(
            data_lines(&b" #kept\n#gone\r\n\r\n\t\n"[..]).unwrap(),
            [" #kept", "\t"]
        );
        assert_eq!(
            data_lines(&b"ok\n\xff\n"[..]).unwrap_err().kind(),
            io::ErrorKind::InvalidData,
            "a bad line after good ones is still an error"
        );
    }
    #[test]
    fn b_byte_records_need_not_be_unicode() {
        assert_eq!(
            nul_records(&b"a\0\0\xff\0"[..]).unwrap(),
            vec![b"a".to_vec(), vec![], vec![255]]
        );
        assert!(nul_records(&b""[..]).unwrap().is_empty());
        assert_eq!(
            nul_records(&b"\0a\0b"[..]).unwrap(),
            vec![vec![], b"a".to_vec(), b"b".to_vec()]
        );
        assert_eq!(
            nul_records(&b"line\none"[..]).unwrap(),
            vec![b"line\none".to_vec()],
            "newlines are ordinary bytes here"
        );
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    #[test]
    fn p_predictions() {
        // P1. read_line into a String that already holds "x": the count it
        // returns, and the String afterwards.
        let mut input = Cursor::new("ab\ncd");
        let mut line = String::from("x");
        let n = input.read_line(&mut line).unwrap();
        let p1: (usize, &str) = guess();
        assert_eq!(p1, (n, line.as_str()), "P1: read_line appends");

        // P2. The items lines() yields for "a\r\nb\n\n".
        let lines: Vec<String> = Cursor::new("a\r\nb\n\n")
            .lines()
            .collect::<io::Result<_>>()
            .unwrap();
        let p2: Vec<&str> = guess();
        assert_eq!(p2, lines, "P2: lines strips LF and CRLF");

        // P3. read_until twice on "a\r\n": both return values, and the buffer length.
        let mut input = Cursor::new(b"a\r\n".to_vec());
        let mut buf = Vec::new();
        let first = input.read_until(b'\n', &mut buf).unwrap();
        let second = input.read_until(b'\n', &mut buf).unwrap();
        let p3: (usize, usize, usize) = guess();
        assert_eq!(
            p3,
            (first, second, buf.len()),
            "P3: read_until keeps the delimiter"
        );
    }
}
