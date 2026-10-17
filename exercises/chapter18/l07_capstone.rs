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
    fn service_transforms_one_frame_and_retains_next() {
        let mut input = &b"\x03\0a\xffbTAIL"[..];
        let mut out = vec![];
        handle_one(&mut input, &mut out, 8).unwrap();
        assert_eq!(out, b"\x03\0b\xffa");
        assert_eq!(input, b"TAIL");
    }
    #[test]
    fn invalid_request_produces_no_response() {
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
    fn empty_frame_is_valid() {
        let mut out = vec![];
        handle_one(&b"\0\0"[..], &mut out, 0).unwrap();
        assert_eq!(out, b"\0\0");
    }
}
