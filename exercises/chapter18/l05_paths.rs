//! LESSON 05 — Paths preserve OS data
//! Run: cargo test --test chapter18 l05_
//!
//! PREDICT before running:
//! 1. Can every Path be borrowed as str?
//! 2. Does Path::join ignore the base when the child is absolute?
//!
//! IMPLEMENT the tasks; keep the tests.
//! EXPLAIN: Distinguish lexical path processing from filesystem canonicalization and security containment.
//! COMPILER LAB (examples/ch18_scratch.rs): On Unix construct OsString from invalid UTF-8 bytes; round-trip it through PathBuf. Also compare file_name, file_stem, extension, components.

use std::path::{Component, Path, PathBuf};
/// A. Change the final extension to "bak" without converting through Unicode.
/// Return None if the path has no normal final file name (e.g. "/" or "..").
fn backup_path(path: &Path) -> Option<PathBuf> {
    todo!("A: implement the contract")
}
/// B. Inspect normalized Path::components: accept nonempty, purely normal
/// relative components (interior a/./b normalizes to a/b). No '.', '..', root,
/// or platform prefix. Reject an empty path. This is lexical validation only;
/// it DOES NOT defend against symlinks or filesystem races.
fn normal_relative(path: &Path) -> bool {
    todo!("B: implement the contract")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn backup_changes_only_final_extension() {
        assert_eq!(
            backup_path(Path::new("reports/one.csv")),
            Some(PathBuf::from("reports/one.bak"))
        );
        assert_eq!(
            backup_path(Path::new("notes")),
            Some(PathBuf::from("notes.bak"))
        );
        assert_eq!(backup_path(Path::new("..")), None);
    }
    #[test]
    fn normal_components_are_lexical() {
        assert!(normal_relative(Path::new("a/b")));
        for p in ["", "../a", "a/../b", "./a", "/a"] {
            assert!(!normal_relative(Path::new(p)), "{p}");
        }
        assert!(
            normal_relative(Path::new("a/./b")),
            "components normalizes interior dot"
        );
    }
    #[cfg(unix)]
    #[test]
    fn backup_retains_non_unicode_file_name() {
        use std::os::unix::ffi::OsStringExt;
        let p = PathBuf::from(std::ffi::OsString::from_vec(vec![255, b'.', b'x']));
        let out = backup_path(&p).unwrap();
        assert_eq!(
            out.into_os_string().into_vec(),
            vec![255, b'.', b'b', b'a', b'k']
        );
    }
}
