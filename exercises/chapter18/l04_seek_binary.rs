//! LESSON 04 — Seeking and binary formats
//! Run: cargo test --test chapter18 l04_
//!
//! PREDICT before running:
//! 1. Does seek change a Cursor backing vector length immediately?
//! 2. Is transmuting a struct a portable file format?
//!
//! IMPLEMENT the tasks; keep the tests.
//! EXPLAIN: Explain why field sizes and byte order belong to a format contract.
//! COMPILER LAB (examples/ch18_scratch.rs): Use Cursor<Vec<u8>> and SeekFrom::End, Current, Start. Then wrap it in BufReader and compare stream_position.

use std::io::{self, Read, Seek, SeekFrom};
/// A. Read a u32 little-endian record at index * 4. Seek from the start,
/// checked-multiply the offset, and propagate EOF/seek/read errors.
fn read_record<R: Read + Seek>(input: &mut R, index: u64) -> io::Result<u32> {
    todo!("A: implement the contract")
}
/// B. Read a single length-prefixed frame; a truncated header or payload is
/// UnexpectedEof. Leave any following frame unread.
fn read_frame<R: Read>(mut input: R) -> io::Result<Vec<u8>> {
    todo!("B: implement the contract")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn random_access_has_explicit_endianness() {
        let mut c = std::io::Cursor::new([1, 0, 0, 0, 0x78, 0x56, 0x34, 0x12]);
        assert_eq!(read_record(&mut c, 1).unwrap(), 0x12345678);
        assert_eq!(read_record(&mut c, 0).unwrap(), 1);
        assert_eq!(
            read_record(&mut c, 2).unwrap_err().kind(),
            io::ErrorKind::UnexpectedEof
        );
        assert_eq!(
            read_record(&mut c, u64::MAX).unwrap_err().kind(),
            io::ErrorKind::InvalidInput
        );
    }
    #[test]
    fn framed_read_does_not_eat_next_message() {
        let mut input = &b"\x02\0hi\0\0"[..];
        assert_eq!(read_frame(&mut input).unwrap(), b"hi");
        assert!(read_frame(&mut input).unwrap().is_empty());
        assert_eq!(
            read_frame(&b"\x02\0x"[..]).unwrap_err().kind(),
            io::ErrorKind::UnexpectedEof
        );
    }
}
