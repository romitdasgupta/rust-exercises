//! LESSON 05 — Paths preserve OS data
//! Run: cargo test --test chapter18 l05_
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider:
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
    use practice::guess;
    use std::ffi::OsStr;
    #[test]
    fn a_backup_changes_only_final_extension() {
        assert_eq!(
            backup_path(Path::new("reports/one.csv")),
            Some(PathBuf::from("reports/one.bak"))
        );
        assert_eq!(
            backup_path(Path::new("notes")),
            Some(PathBuf::from("notes.bak"))
        );
        assert_eq!(backup_path(Path::new("..")), None);
        assert_eq!(
            backup_path(Path::new("archive.tar.gz")),
            Some(PathBuf::from("archive.tar.bak"))
        );
        assert_eq!(
            backup_path(Path::new(".bashrc")),
            Some(PathBuf::from(".bashrc.bak")),
            "a leading dot is not an extension"
        );
        assert_eq!(backup_path(Path::new("/")), None);
        assert_eq!(backup_path(Path::new("")), None);
    }
    #[test]
    fn b_normal_components_are_lexical() {
        assert!(normal_relative(Path::new("a/b")));
        for p in ["", "../a", "a/../b", "./a", "/a"] {
            assert!(!normal_relative(Path::new(p)), "{p}");
        }
        assert!(
            normal_relative(Path::new("a/./b")),
            "components normalizes interior dot"
        );
        assert!(normal_relative(Path::new("a/")), "a trailing slash is fine");
        assert!(normal_relative(Path::new("é/🦀.txt")));
        for p in [".", "a/..", "../"] {
            assert!(!normal_relative(Path::new(p)), "{p}");
        }
    }
    #[cfg(unix)]
    #[test]
    fn a_backup_retains_non_unicode_file_name() {
        use std::os::unix::ffi::OsStringExt;
        let p = PathBuf::from(std::ffi::OsString::from_vec(vec![255, b'.', b'x']));
        let out = backup_path(&p).unwrap();
        assert_eq!(
            out.into_os_string().into_vec(),
            vec![255, b'.', b'b', b'a', b'k']
        );
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    #[test]
    fn p_predictions() {
        // P1. Joining an absolute path onto a base.
        #[allow(clippy::join_absolute_paths)] // this surprise is the point of P1
        let joined = Path::new("/etc").join("/tmp");
        let p1: PathBuf = guess();
        assert_eq!(p1, joined, "P1: join with a rooted path");

        // P2. file_stem and extension of "archive.tar.gz".
        let path = Path::new("archive.tar.gz");
        let p2: (Option<&str>, Option<&str>) = guess();
        assert_eq!(
            p2,
            (
                path.file_stem().and_then(OsStr::to_str),
                path.extension().and_then(OsStr::to_str)
            ),
            "P2: stem and extension"
        );

        // P3. How many components does "a/./b/../c" have?
        let p3: usize = guess();
        assert_eq!(
            p3,
            Path::new("a/./b/../c").components().count(),
            "P3: components are lexical"
        );

        // P4. starts_with compares whole components.
        let path = Path::new("dir/file.txt");
        let p4: (bool, bool) = guess();
        assert_eq!(
            p4,
            (path.starts_with("dir"), path.starts_with("di")),
            "P4: Path::starts_with"
        );
    }
}
