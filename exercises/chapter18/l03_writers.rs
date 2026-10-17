//! LESSON 03 — Writers and flushing
//! Run: cargo test --test chapter18 l03_
//!
//! PREDICT before running:
//! 1. Does a successful write guarantee all bytes were written?
//! 2. Can buffered output errors appear only when flushing?
//!
//! IMPLEMENT the tasks; keep the tests.
//! EXPLAIN: Why should a function explicitly propagate flush failure?
//! COMPILER LAB (examples/ch18_scratch.rs): Use BufWriter<Vec<u8>>; compare buffer() with get_ref() before and after flush. Then try LineWriter.

use std::io::{self, Write};
/// A. Write every row followed by LF, even an empty row. Explicitly flush once
/// after all writes succeed, propagating any write or flush error.
fn write_rows<W: Write>(mut out: W, rows: &[&str]) -> io::Result<()> {
    todo!("A: implement the contract")
}
/// B. Encode a u16-length-prefixed byte payload (little endian). Reject lengths
/// above u16::MAX with InvalidInput before writing anything. Do not flush here.
fn write_frame<W: Write>(mut out: W, payload: &[u8]) -> io::Result<()> {
    todo!("B: implement the contract")
}

#[cfg(test)]
mod tests {
    use super::*;
    struct Short {
        bytes: Vec<u8>,
        flushed: usize,
        fail_flush: bool,
    }
    impl Write for Short {
        fn write(&mut self, b: &[u8]) -> io::Result<usize> {
            let n = b.len().min(1);
            self.bytes.extend_from_slice(&b[..n]);
            Ok(n)
        }
        fn flush(&mut self) -> io::Result<()> {
            self.flushed += 1;
            if self.fail_flush {
                Err(io::ErrorKind::BrokenPipe.into())
            } else {
                Ok(())
            }
        }
    }
    #[test]
    fn rows_handle_short_writes_and_flush_failure() {
        let mut w = Short {
            bytes: vec![],
            flushed: 0,
            fail_flush: false,
        };
        write_rows(&mut w, &["é", ""]).unwrap();
        assert_eq!(w.bytes, "é\n\n".as_bytes());
        assert_eq!(w.flushed, 1);
        w.fail_flush = true;
        assert_eq!(
            write_rows(&mut w, &[]).unwrap_err().kind(),
            io::ErrorKind::BrokenPipe
        );
    }
    #[test]
    fn frame_checks_length_before_output() {
        let mut out = vec![];
        write_frame(&mut out, b"hi").unwrap();
        assert_eq!(out, b"\x02\0hi");
        out.clear();
        assert_eq!(
            write_frame(&mut out, &vec![0; 65536]).unwrap_err().kind(),
            io::ErrorKind::InvalidInput
        );
        assert!(out.is_empty());
    }
}
