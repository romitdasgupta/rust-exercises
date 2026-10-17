//! LESSON 01 — Streaming bytes
//! Run: cargo test --test chapter18 l01_
//!
//! PREDICT before running:
//! 1. Can read return fewer bytes than requested before EOF?
//! 2. Does read_to_string accept arbitrary bytes?
//!
//! IMPLEMENT the tasks; keep the tests.
//! EXPLAIN: Why use Read rather than File in the contract? Explain EOF versus Interrupted.
//! COMPILER LAB (examples/ch18_scratch.rs): Implement a Read wrapper that returns one byte per call. Compare read, read_exact, take, and chain.

use std::io::{self, Read, Write};
/// A. Copy all bytes with a fixed 4-byte scratch buffer. Retry Interrupted;
/// propagate all other errors. Use write_all, not one write per chunk.
fn copy_stream<R: Read, W: Write>(mut input: R, mut output: W) -> io::Result<u64> {
    todo!("A: implement the contract")
}
/// B. Read at most limit bytes without consuming the next byte.
fn prefix<R: Read>(input: &mut R, limit: u64) -> io::Result<Vec<u8>> {
    todo!("B: implement the contract")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn copy_handles_binary_short_reads_and_interruptions() {
        struct Source {
            first: bool,
            rest: &'static [u8],
        }
        impl Read for Source {
            fn read(&mut self, b: &mut [u8]) -> io::Result<usize> {
                if self.first {
                    self.first = false;
                    return Err(io::ErrorKind::Interrupted.into());
                }
                if self.rest.is_empty() {
                    return Ok(0);
                }
                b[0] = self.rest[0];
                self.rest = &self.rest[1..];
                Ok(1)
            }
        }
        let mut out = Vec::new();
        assert_eq!(
            copy_stream(
                Source {
                    first: true,
                    rest: b"a\0\xffz"
                },
                &mut out
            )
            .unwrap(),
            4
        );
        assert_eq!(out, b"a\0\xffz");
        assert_eq!(copy_stream(&b""[..], Vec::new()).unwrap(), 0);
        struct Broken;
        impl Read for Broken {
            fn read(&mut self, _: &mut [u8]) -> io::Result<usize> {
                Err(io::ErrorKind::PermissionDenied.into())
            }
        }
        assert_eq!(
            copy_stream(Broken, Vec::new()).unwrap_err().kind(),
            io::ErrorKind::PermissionDenied
        );
    }
    #[test]
    fn prefix_leaves_remainder() {
        let mut r = &b"abcdef"[..];
        assert_eq!(prefix(&mut r, 2).unwrap(), b"ab");
        assert_eq!(r, b"cdef");
        assert_eq!(prefix(&mut r, 0).unwrap(), b"");
    }
}
