//! LESSON 07 — Capstone: a bounded record service
//! Run: cargo test --test chapter18 l07_
//!
//! PREDICT before running:
//! 1. What happens if an attacker declares an oversized payload?
//! 2. Can one frame contain arbitrary non-UTF-8 bytes?
//!
//! IMPLEMENT the tasks; keep the tests.
//! EXPLAIN: Explain framing, length validation before allocation, error propagation, and when the response is flushed.
//! COMPILER LAB (examples/ch18_scratch.rs): In the loopback lab, deliberately split the request header across two writes. Verify the service still handles it.

use std::io::{self, Read, Write};
/// A. Read one u16-LE length-prefixed frame. Reject lengths above max with
/// InvalidData before allocating/reading its body. Require all declared bytes.
fn receive<R: Read>(mut input: R, max: u16) -> io::Result<Vec<u8>> {
    todo!("A: implement the contract")
}
/// B. Receive one frame, reverse its bytes (not Unicode characters), send the
/// same framing, then flush. No output may be written if receiving fails.
fn handle_one<R: Read, W: Write>(input: R, mut output: W, max: u16) -> io::Result<()> {
    todo!("B: implement the contract")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a_receive_rejects_an_oversized_length_before_reading_the_body() {
        let mut input = &b"\x05\0hello"[..];
        assert_eq!(
            receive(&mut input, 4).unwrap_err().kind(),
            io::ErrorKind::InvalidData
        );
        assert_eq!(input, b"hello", "only the header may be consumed");
        let mut huge = &b"\xff\xff"[..];
        assert_eq!(
            receive(&mut huge, 100).unwrap_err().kind(),
            io::ErrorKind::InvalidData
        );
    }
    #[test]
    fn a_receive_requires_every_declared_byte() {
        assert_eq!(
            receive(&b"\x02\0hi"[..], 2).unwrap(),
            b"hi",
            "max is inclusive"
        );
        for bytes in [&b"\x03\0ab"[..], &b"\x03"[..], &b""[..]] {
            assert_eq!(
                receive(bytes, 8).unwrap_err().kind(),
                io::ErrorKind::UnexpectedEof,
                "{bytes:?}"
            );
        }
    }
    #[test]
    fn b_service_transforms_one_frame_and_retains_next() {
        let mut input = &b"\x03\0a\xffbTAIL"[..];
        let mut out = vec![];
        handle_one(&mut input, &mut out, 8).unwrap();
        assert_eq!(out, b"\x03\0b\xffa");
        assert_eq!(input, b"TAIL");
    }
    #[test]
    fn b_invalid_request_produces_no_response() {
        for (bytes, kind) in [
            (&b"\x05\0"[..], io::ErrorKind::InvalidData),
            (&b"\x02\0x"[..], io::ErrorKind::UnexpectedEof),
        ] {
            let mut out = vec![];
            assert_eq!(handle_one(bytes, &mut out, 4).unwrap_err().kind(), kind);
            assert!(out.is_empty());
        }
    }
    #[test]
    fn b_empty_frame_is_valid() {
        let mut out = vec![];
        handle_one(&b"\0\0"[..], &mut out, 0).unwrap();
        assert_eq!(out, b"\0\0");
    }
    #[test]
    fn b_response_survives_short_writes_and_is_flushed_once() {
        struct Narrow {
            bytes: Vec<u8>,
            flushed: usize,
        }
        impl Write for Narrow {
            fn write(&mut self, b: &[u8]) -> io::Result<usize> {
                let n = b.len().min(1);
                self.bytes.extend_from_slice(&b[..n]);
                Ok(n)
            }
            fn flush(&mut self) -> io::Result<()> {
                self.flushed += 1;
                Ok(())
            }
        }
        let mut w = Narrow {
            bytes: vec![],
            flushed: 0,
        };
        // "é!" is the three bytes C3 A9 21.
        handle_one(&b"\x03\0\xc3\xa9!"[..], &mut w, 8).unwrap();
        assert_eq!(
            w.bytes,
            [3, 0, b'!', 0xa9, 0xc3],
            "bytes reversed, not chars"
        );
        assert_eq!(w.flushed, 1);
    }
}
