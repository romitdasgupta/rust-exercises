//! LESSON 04 — Seeking and binary formats
//! Run: cargo test --test chapter18 l04_
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider:
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
    use practice::guess;
    use std::io::{Cursor, Write};
    #[test]
    fn a_random_access_has_explicit_endianness() {
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
    fn a_partial_and_distant_records_are_eof() {
        let mut c = Cursor::new(vec![1, 0, 0, 0, 9, 9]);
        assert_eq!(
            read_record(&mut c, 1).unwrap_err().kind(),
            io::ErrorKind::UnexpectedEof,
            "half a record is not a record"
        );
        assert_eq!(
            read_record(&mut c, 1 << 40).unwrap_err().kind(),
            io::ErrorKind::UnexpectedEof
        );
        assert_eq!(read_record(&mut c, 0).unwrap(), 1, "seek from the start");
    }
    #[test]
    fn b_framed_read_does_not_eat_next_message() {
        let mut input = &b"\x02\0hi\0\0"[..];
        assert_eq!(read_frame(&mut input).unwrap(), b"hi");
        assert!(read_frame(&mut input).unwrap().is_empty());
        assert_eq!(
            read_frame(&b"\x02\0x"[..]).unwrap_err().kind(),
            io::ErrorKind::UnexpectedEof
        );
    }
    #[test]
    fn b_truncated_headers_and_little_endian_lengths() {
        assert_eq!(
            read_frame(&b"\x02"[..]).unwrap_err().kind(),
            io::ErrorKind::UnexpectedEof
        );
        let mut frame = vec![0x01, 0x01];
        frame.extend(std::iter::repeat_n(b'z', 257));
        frame.push(b'!');
        let mut input = &frame[..];
        assert_eq!(
            read_frame(&mut input).unwrap().len(),
            257,
            "0x0101 little endian"
        );
        assert_eq!(input, b"!");
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    /// A wrong prediction is useful: read the failure, then explain it in NOTES.md.
    #[test]
    fn p_predictions() {
        // P1. SeekFrom::End(-1): the new position, then the byte read there.
        let mut c = Cursor::new(vec![1u8, 2, 3]);
        let pos = c.seek(SeekFrom::End(-1)).unwrap();
        let mut b = [0];
        c.read_exact(&mut b).unwrap();
        let p1: (u64, u8) = guess();
        assert_eq!(p1, (pos, b[0]), "P1: seeking from the end");

        // P2. Seek past the end of a Cursor<Vec<u8>>, then write one byte: the
        // position, the length before the write, and the length after.
        let mut c = Cursor::new(vec![1u8, 2, 3]);
        let pos = c.seek(SeekFrom::Start(10)).unwrap();
        let before = c.get_ref().len();
        c.write_all(&[9]).unwrap();
        let p2: (u64, usize, usize) = guess();
        assert_eq!(
            p2,
            (pos, before, c.get_ref().len()),
            "P2: seeking past the end"
        );

        // P3. Seeking before the start: which error kind?
        let mut c = Cursor::new(vec![1u8]);
        let p3: io::ErrorKind = guess();
        assert_eq!(
            p3,
            c.seek(SeekFrom::Current(-5)).unwrap_err().kind(),
            "P3: negative positions"
        );

        // P4. Byte order: decode two little-endian bytes; encode big-endian.
        let p4: (u16, [u8; 2]) = guess();
        assert_eq!(
            p4,
            (u16::from_le_bytes([0x34, 0x12]), 0x1234u16.to_be_bytes()),
            "P4: from_le_bytes / to_be_bytes"
        );
    }
}
