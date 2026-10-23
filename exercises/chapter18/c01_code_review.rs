//! CODE REVIEW 01 — A teammate's pull request
//! Run: cargo test --test chapter18 c01_
//! Then: cargo clippy --test chapter18
//!
//! This is the kind of code you will be asked to review at work. It compiles,
//! and some behavior tests pass. Your job:
//! 1. Before running anything, read the code and write a review in NOTES.md:
//!    list every bug, every error that panics instead of propagating, every
//!    assumption that one call moves all the bytes, and every needless
//!    buffer or allocation.
//! 2. Run the tests. Compare what they found with your review.
//! 3. Fix it. You MAY change parameter types where the doc comment says so;
//!    the tests are written to accept the better signature.
//! 4. Run clippy and resolve its warnings for this file.

use std::io::{self, Read, Write};
use std::path::PathBuf;

/// Count the lines in a byte stream: every b'\n' ends a line, and a final
/// line without one also counts; empty input has 0 lines. The input need not
/// be UTF-8. Propagate read errors. Keep this signature.
fn count_lines<R: Read>(mut input: R) -> io::Result<usize> {
    let mut text = String::new();
    input.read_to_string(&mut text).unwrap();
    Ok(text.split('\n').count())
}

/// Write each row as "NAME: VALUE\n", flush once at the end, and return the
/// number of bytes written. Propagate errors. Keep this signature.
fn write_report<W: Write>(mut out: W, rows: &[(&str, u32)]) -> io::Result<usize> {
    let mut total = 0;
    for (name, value) in rows {
        let line = format!("{}: {}\n", name, value);
        total += out.write(line.as_bytes())?;
    }
    out.flush()?;
    Ok(total)
}

/// The path of a backup copy: the full file name plus ".bak" in the same
/// directory ("dir/a.txt" -> "dir/a.txt.bak"). File names need not be UTF-8.
/// You may change the parameter type.
fn backup_name(path: &PathBuf) -> PathBuf {
    let s = path.to_str().unwrap();
    PathBuf::from(format!("{}.bak", s))
}

#[cfg(test)]
mod tests {
    use super::*;
    use practice::alloc::count;

    /// Produces `n` copies of "line\n" without telling anyone its size.
    struct Lines(usize);
    impl Read for Lines {
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            let mut n = 0;
            while self.0 > 0 && buf.len() - n >= 5 {
                buf[n..n + 5].copy_from_slice(b"line\n");
                n += 5;
                self.0 -= 1;
            }
            Ok(n)
        }
    }

    #[test]
    fn a_count_lines_counts_terminated_and_final_lines() {
        assert_eq!(count_lines(&b"a\nb"[..]).unwrap(), 2);
        assert_eq!(count_lines(&b"a\nb\n"[..]).unwrap(), 2);
        assert_eq!(count_lines(&b"\n\n"[..]).unwrap(), 2);
        assert_eq!(count_lines(&b""[..]).unwrap(), 0);
    }

    #[test]
    fn a_count_lines_accepts_bytes_and_propagates_errors() {
        assert_eq!(count_lines(&b"\xff\n\xfe"[..]).unwrap(), 2);
        struct Broken;
        impl Read for Broken {
            fn read(&mut self, _: &mut [u8]) -> io::Result<usize> {
                Err(io::ErrorKind::ConnectionReset.into())
            }
        }
        assert_eq!(
            count_lines(Broken).unwrap_err().kind(),
            io::ErrorKind::ConnectionReset,
            "return the error; don't panic"
        );
    }

    #[test]
    fn a_count_lines_streams_instead_of_buffering_everything() {
        let (lines, allocations) = count(|| count_lines(Lines(100_000)).unwrap());
        assert!(
            allocations <= 4,
            "500 kB of input cost {allocations} allocations"
        );
        assert_eq!(lines, 100_000);
    }

    #[test]
    fn b_report_is_complete_despite_short_writes() {
        struct Narrow {
            bytes: Vec<u8>,
            flushed: usize,
        }
        impl Write for Narrow {
            fn write(&mut self, b: &[u8]) -> io::Result<usize> {
                let n = b.len().min(3);
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
        let total = write_report(&mut w, &[("disk", 93), ("é", 7)]).unwrap();
        assert_eq!(w.bytes, "disk: 93\né: 7\n".as_bytes());
        assert_eq!(total, w.bytes.len());
        assert_eq!(w.flushed, 1);
        assert_eq!(write_report(Vec::new(), &[]).unwrap(), 0);
    }

    #[test]
    fn b_report_formats_straight_into_the_writer() {
        let rows: Vec<(&str, u32)> = (0..100).map(|i| ("row", i)).collect();
        let (total, allocations) = count(|| write_report(io::sink(), &rows).unwrap());
        assert_eq!(total, 10 * 7 + 90 * 8);
        assert!(allocations <= 2, "100 rows cost {allocations} allocations");
    }

    #[test]
    fn c_backup_name_appends_to_the_full_name() {
        assert_eq!(
            backup_name(&PathBuf::from("dir/a.txt")),
            PathBuf::from("dir/a.txt.bak")
        );
        assert_eq!(
            backup_name(&PathBuf::from("notes")),
            PathBuf::from("notes.bak")
        );
    }

    #[cfg(unix)]
    #[test]
    fn c_backup_name_keeps_non_utf8_names() {
        use std::ffi::OsString;
        use std::os::unix::ffi::{OsStrExt, OsStringExt};
        let path = PathBuf::from(OsString::from_vec(vec![b'r', 0xff]));
        let backup = backup_name(&path);
        assert_eq!(backup.as_os_str().as_bytes(), b"r\xff.bak");
    }
}
