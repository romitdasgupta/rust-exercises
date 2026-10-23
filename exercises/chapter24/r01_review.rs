//! REVIEW 01 — Spaced retrieval: Chapters 18, 22, and 23 inside Chapter 24
//! Run: cargo test --test chapter24 r01_
//!
//! Do this WITHOUT reopening Chapters 18, 22, or 23 or their hints. Struggling
//! to recall is what makes the knowledge stick; look things up only after a
//! real attempt, and note in NOTES.md anything you had to look up.
//!
//! Each task mixes an earlier idea (uninitialized memory, macro control flow,
//! byte-level I/O) with this chapter's foreign-boundary rules.

use std::ffi::{c_char, c_int, c_uint};
use std::io::{self, Read, Write};

const AVAILABLE: usize = 5;

// Supplied ABI mock: writes min(cap, AVAILABLE) readings (10, 20, 30, ...) to
// the start of `out` and returns how many it wrote.
// # Safety: out is valid for writing `cap` c_ints. No pointer is retained.
unsafe extern "C" fn foreign_fill(out: *mut c_int, cap: usize) -> usize {
    let n = cap.min(AVAILABLE);
    for (i, value) in (0..n).zip((10..).step_by(10)) {
        // SAFETY: i < n <= cap, so the slot is inside the caller's buffer.
        unsafe { out.add(i).write(value) };
    }
    n
}

/// A. (Ch23 uninitialized memory + Ch24 out-buffers) Allocate ONE Vec with
/// capacity `max`, let `foreign_fill` write into its spare capacity, then set
/// the length to the reported count, after checking that it is <= max.
/// Never zero-fill and never expose an element the library did not write.
fn read_samples(max: usize) -> Vec<c_int> {
    todo!("A: Vec::with_capacity, spare_capacity_mut or as_mut_ptr, then set_len")
}

/// B. (Ch22 macro control flow + Ch24 status codes) Evaluate a c_int status
/// expression exactly once. If it is non-zero, `return Err(status)` from the
/// ENCLOSING function; otherwise the macro evaluates to `()`. Like `?` for C.
macro_rules! ffi_try {
    ($status:expr) => {{ todo!("B: bind the status once, then return early on failure") }};
}

#[repr(C)]
#[derive(Debug, PartialEq)]
struct Reading {
    kind: c_char,
    count: c_uint,
    value: c_int,
}

/// C. (Ch18 I/O + Ch24 layout) Write `r` as a fixed 9-byte little-endian
/// record: kind (1 byte), count (4), value (4). Do not dump the struct's
/// memory: it has padding and the target's byte order. Propagate I/O errors.
fn write_record(out: &mut impl Write, r: &Reading) -> io::Result<()> {
    todo!("C: to_le_bytes for each field, then write_all")
}

/// C. Read one 9-byte record written by `write_record`. A short input is an
/// error (UnexpectedEof), never a partial Reading.
fn read_record(input: &mut impl Read) -> io::Result<Reading> {
    todo!("C: read_exact into a [u8; 9], then from_le_bytes")
}

#[cfg(test)]
mod tests {
    use super::*;
    use practice::{alloc::count, guess};
    use std::cell::RefCell;
    use std::io::Cursor;

    #[test]
    fn a_samples_come_from_the_library() {
        assert_eq!(read_samples(3), [10, 20, 30]);
        assert_eq!(read_samples(8), [10, 20, 30, 40, 50]);
        assert!(read_samples(0).is_empty());
    }

    #[test]
    fn a_samples_use_one_allocation() {
        let (samples, allocations) = count(|| read_samples(64));
        assert_eq!(samples.len(), AVAILABLE);
        assert!(samples.capacity() >= 64);
        assert!(
            allocations <= 1,
            "one buffer; got {allocations} allocations"
        );
    }

    fn step(log: &RefCell<Vec<&'static str>>, name: &'static str, status: c_int) -> c_int {
        log.borrow_mut().push(name);
        status
    }

    fn transaction(
        log: &RefCell<Vec<&'static str>>,
        write_status: c_int,
    ) -> Result<&'static str, c_int> {
        ffi_try!(step(log, "open", 0));
        ffi_try!(step(log, "write", write_status));
        ffi_try!(step(log, "close", 0));
        Ok("done")
    }

    #[test]
    fn b_success_runs_every_step_once() {
        let log = RefCell::new(Vec::new());
        assert_eq!(transaction(&log, 0), Ok("done"));
        assert_eq!(*log.borrow(), ["open", "write", "close"]);
    }

    #[test]
    fn b_failure_returns_early_from_the_caller() {
        let log = RefCell::new(Vec::new());
        assert_eq!(transaction(&log, -5), Err(-5));
        assert_eq!(*log.borrow(), ["open", "write"], "close never runs");
        let log = RefCell::new(Vec::new());
        assert_eq!(transaction(&log, 3), Err(3), "any non-zero status fails");
    }

    #[test]
    fn c_record_bytes_are_explicit_little_endian() {
        let r = Reading {
            kind: 0x41,
            count: 0x0102_0304,
            value: -2,
        };
        let mut bytes = Vec::new();
        write_record(&mut bytes, &r).unwrap();
        assert_eq!(bytes, [0x41, 4, 3, 2, 1, 0xfe, 0xff, 0xff, 0xff]);
        assert_ne!(bytes.len(), size_of::<Reading>(), "no padding on the wire");
        assert_eq!(read_record(&mut Cursor::new(bytes)).unwrap(), r);
    }

    #[test]
    fn c_errors_propagate_in_both_directions() {
        let short = read_record(&mut Cursor::new([1_u8, 2, 3])).unwrap_err();
        assert_eq!(short.kind(), io::ErrorKind::UnexpectedEof);
        let r = Reading {
            kind: 0,
            count: 0,
            value: 0,
        };
        let mut tiny = [0_u8; 4];
        let full = write_record(&mut &mut tiny[..], &r).unwrap_err();
        assert_eq!(full.kind(), io::ErrorKind::WriteZero);
    }

    #[test]
    fn c_writing_into_a_reserved_buffer_does_not_allocate() {
        let r = Reading {
            kind: 1,
            count: 2,
            value: 3,
        };
        let mut out = Vec::with_capacity(64);
        let (result, allocations) = count(|| write_record(&mut out, &r));
        assert!(result.is_ok());
        assert_eq!(out.len(), 9);
        assert_eq!(allocations, 0, "no temporary buffers");
    }

    /// P. Chapters 18–23 recall: predict each value BEFORE running.
    #[test]
    fn p_predictions() {
        // P1. How many times is "kept" dropped by the end of this block?
        let log = RefCell::new(Vec::new());
        struct Noisy<'a>(&'a RefCell<Vec<&'static str>>);
        impl Drop for Noisy<'_> {
            fn drop(&mut self) {
                self.0.borrow_mut().push("kept");
            }
        }
        {
            let _kept = std::mem::ManuallyDrop::new(Noisy(&log));
        }
        let p1: usize = guess();
        assert_eq!(p1, log.borrow().len(), "P1: ManuallyDrop");

        // P2. How many token trees are in `f(a, b) [c]`?
        macro_rules! count_tts {
            ($($t:tt)*) => { 0 $(+ { let _ = stringify!($t); 1 })* };
        }
        let p2: i32 = guess();
        assert_eq!(p2, count_tts!(f(a, b)[c]), "P2: token trees");

        // P3. Write "abc", seek back to 1, write "Z". What does the buffer hold?
        let mut cursor = Cursor::new(Vec::new());
        cursor.write_all(b"abc").unwrap();
        cursor.set_position(1);
        cursor.write_all(b"Z").unwrap();
        let p3: &[u8] = guess();
        assert_eq!(p3, cursor.get_ref().as_slice(), "P3: Cursor overwrites");
    }
}
