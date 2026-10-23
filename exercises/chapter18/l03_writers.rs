//! LESSON 03 — Writers and flushing
//! Run: cargo test --test chapter18 l03_
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider:
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
    use practice::guess;
    use std::io::{BufWriter, LineWriter};
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
    fn a_rows_handle_short_writes_and_flush_failure() {
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
    fn a_rows_stop_at_a_write_error_without_flushing() {
        struct Refuses {
            flushed: usize,
        }
        impl Write for Refuses {
            fn write(&mut self, _: &[u8]) -> io::Result<usize> {
                Err(io::ErrorKind::PermissionDenied.into())
            }
            fn flush(&mut self) -> io::Result<()> {
                self.flushed += 1;
                Ok(())
            }
        }
        let mut w = Refuses { flushed: 0 };
        assert_eq!(
            write_rows(&mut w, &["x"]).unwrap_err().kind(),
            io::ErrorKind::PermissionDenied
        );
        assert_eq!(w.flushed, 0, "flush only after every write succeeded");
    }
    #[test]
    fn b_frame_checks_length_before_output() {
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
    #[test]
    fn b_frame_accepts_the_maximum_and_does_not_flush() {
        let mut out = vec![];
        write_frame(&mut out, &vec![7; 65535]).unwrap();
        assert_eq!((out.len(), &out[..3]), (65537, &[0xff, 0xff, 7][..]));
        let mut w = Short {
            bytes: vec![],
            flushed: 0,
            fail_flush: false,
        };
        write_frame(&mut w, "é".as_bytes()).unwrap();
        assert_eq!(
            w.bytes, b"\x02\0\xc3\xa9",
            "length counts bytes; short writes retried"
        );
        assert_eq!(w.flushed, 0, "the caller decides when to flush");
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    #[test]
    fn p_predictions() {
        // P1. Bytes in BufWriter's inner Vec before and after flush.
        let mut w = BufWriter::new(Vec::new());
        w.write_all(b"hi").unwrap();
        let before = w.get_ref().len();
        w.flush().unwrap();
        let p1: (usize, usize) = guess();
        assert_eq!(p1, (before, w.get_ref().len()), "P1: buffering");

        // P2. Writing 5 bytes into a 3-byte slice: what write returns, and the
        // error kind from write_all.
        let mut buf = [0u8; 3];
        let n = (&mut buf[..]).write(b"hello").unwrap();
        let kind = (&mut buf[..]).write_all(b"hello").unwrap_err().kind();
        let p2: (usize, io::ErrorKind) = guess();
        assert_eq!(p2, (n, kind), "P2: short writes");

        // P3. LineWriter after write_all(b"a\nb"): what has reached the Vec?
        let mut w = LineWriter::new(Vec::new());
        w.write_all(b"a\nb").unwrap();
        let p3: &[u8] = guess();
        assert_eq!(p3, w.get_ref().as_slice(), "P3: LineWriter");
    }
}
