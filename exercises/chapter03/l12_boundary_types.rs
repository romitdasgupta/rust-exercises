//! LESSON 12 — Pick the representation at a system boundary
//! Run: cargo test --test chapter03 l12_
//!
//! PREDICT: Give the types of b"OK", b'O', and c"status". Which of the
//! byte and C-string literals contains an implicit terminal zero byte?
//! Can an OS filename contain bytes that aren't UTF-8? Can a C string
//! contain an interior zero byte? What does a type alias change?
//!
//! EXPLAIN: Choose types for user text, arbitrary file contents, filesystem
//! paths, environment-variable names, and C API arguments. Distinguish owned
//! and borrowed forms. Why doesn't an alias enforce a new validation rule?
//!
//! COMPILER LAB: try a non-ASCII character directly inside a byte literal.
//! Try c"a\0b". Declare another alias for Vec<u8> and assign between the two
//! aliases. Which examples fail, and which are interchangeable?

use std::ffi::{CStr, OsStr, OsString};
use std::path::{Path, PathBuf};

// This is a name for an existing type, not a distinct type. Use it in task A.
type PacketBytes = Vec<u8>;

/// A. Return owned bytes [0x89, b'R', b'U', b'S', b'T'] using a byte string
/// literal and copying its bytes into the aliased vector type.
fn packet_header() -> PacketBytes {
    todo!("A: arbitrary bytes need not be valid UTF-8")
}

/// B. Make an owned path from a borrowed path, preserving its representation.
/// This task does not read, write, or otherwise access the filesystem.
fn owned_path(path: &Path) -> PathBuf {
    todo!("B: use the owned form of a path")
}

/// C. Borrow an OS string as OsStr without converting through UTF-8.
fn borrowed_os_name(name: &OsString) -> &OsStr {
    todo!("C: preserve platform-native string data")
}

/// D. Return a borrowed C-string literal for the command status.
/// Use c"..." syntax. Its storage includes a terminating zero byte.
fn c_command() -> &'static CStr {
    todo!("D: return a C string with its representation invariant intact")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_alias_is_still_an_ordinary_byte_vector() {
        let bytes: Vec<u8> = packet_header();
        assert_eq!(bytes, [0x89, b'R', b'U', b'S', b'T']);
        assert!(std::str::from_utf8(&bytes).is_err());
    }

    #[test]
    fn b_owned_path_preserves_the_path() {
        let path = Path::new("logs").join("測定.txt");
        let owned = owned_path(&path);
        assert_eq!(owned.as_path(), path.as_path());
        assert_eq!(owned.file_name(), Some(OsStr::new("測定.txt")));
        assert_eq!(owned_path(Path::new("")), PathBuf::new());
    }

    #[test]
    fn c_os_string_can_be_borrowed_without_a_copy() {
        let name = OsString::from("sensor_β");
        assert_eq!(borrowed_os_name(&name), OsStr::new("sensor_β"));
        assert!(std::ptr::eq(borrowed_os_name(&name), name.as_os_str()));
    }

    #[cfg(unix)]
    #[test]
    fn c_os_string_does_not_require_valid_utf8() {
        use std::os::unix::ffi::{OsStrExt, OsStringExt};
        let name = OsString::from_vec(vec![b'x', 0xff]);
        let borrowed = borrowed_os_name(&name);
        assert_eq!(borrowed.as_bytes(), &[b'x', 0xff]);
        assert!(borrowed.to_str().is_none());
    }

    #[test]
    fn d_c_string_has_one_terminal_zero() {
        assert_eq!(c_command().to_bytes(), b"status");
        assert_eq!(c_command().to_bytes_with_nul(), b"status\0");
    }
}
