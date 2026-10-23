//! LESSON 06 — Directory metadata
//! Run: cargo test --test chapter18 l06_
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider:
//! 1. Is read_dir order stable?
//! 2. Do metadata and symlink_metadata follow the same objects?
//!
//! IMPLEMENT the tasks; keep the tests.
//! EXPLAIN: Why should a listing specify symlink and ordering policies?
//! COMPILER LAB (examples/ch18_scratch.rs): Create an isolated temporary tree. Compare metadata and symlink_metadata on a symlink; remove only the tree you created.

use std::{ffi::OsString, fs, io, path::Path};
/// A. List names of immediate regular files, sorted by OsString ordering.
/// Skip directories and symlinks; use DirEntry::file_type (no link following).
/// Propagate all errors. Do not recurse.
fn regular_files(dir: &Path) -> io::Result<Vec<OsString>> {
    todo!("A: implement the contract")
}
/// B. Return false only for NotFound; propagate permission and other errors.
/// This check is observational, not permission to perform a later operation.
fn exists_result(path: &Path) -> io::Result<bool> {
    todo!("B: implement the contract")
}

#[cfg(test)]
mod tests {
    use super::*;
    use practice::guess;
    struct Temp(std::path::PathBuf);
    impl Drop for Temp {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }
    fn fixture() -> Temp {
        for index in 0..1000 {
            let path =
                std::env::temp_dir().join(format!("rust-ch18-{}-{index}", std::process::id()));
            match fs::create_dir(&path) {
                Ok(()) => return Temp(path),
                Err(e) if e.kind() == io::ErrorKind::AlreadyExists => continue,
                Err(e) => panic!("{e}"),
            }
        }
        panic!("no fixture name")
    }
    #[test]
    fn a_file_names_sorted_and_nonrecursive() {
        let d = fixture();
        fs::write(d.0.join("z"), b"z").unwrap();
        fs::write(d.0.join("a"), b"a").unwrap();
        fs::write(d.0.join("B"), b"b").unwrap();
        fs::create_dir(d.0.join("sub")).unwrap();
        fs::write(d.0.join("sub/nested"), b"x").unwrap();
        #[cfg(unix)]
        std::os::unix::fs::symlink(d.0.join("z"), d.0.join("link")).unwrap();
        assert_eq!(
            regular_files(&d.0).unwrap(),
            [
                OsString::from("B"),
                OsString::from("a"),
                OsString::from("z")
            ]
        );
        assert!(regular_files(&d.0.join("absent")).is_err());
        assert!(
            regular_files(&d.0.join("sub/nested")).is_err(),
            "not a directory"
        );
        let empty = d.0.join("empty");
        fs::create_dir(&empty).unwrap();
        assert!(regular_files(&empty).unwrap().is_empty());
    }
    #[test]
    fn b_exists_reports_only_not_found_as_false() {
        let d = fixture();
        fs::write(d.0.join("z"), b"z").unwrap();
        assert!(exists_result(&d.0.join("z")).unwrap());
        assert!(exists_result(&d.0).unwrap(), "directories exist too");
        assert!(!exists_result(&d.0.join("absent")).unwrap());
        // Looking *inside* a regular file fails with NotADirectory on Unix:
        // that is an error to propagate, not a "no".
        #[cfg(unix)]
        assert!(exists_result(&d.0.join("z/child")).is_err());
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    #[test]
    fn p_predictions() {
        let d = fixture();
        // P1. remove_file on a path that doesn't exist: which error kind?
        let p1: io::ErrorKind = guess();
        assert_eq!(
            p1,
            fs::remove_file(d.0.join("missing")).unwrap_err().kind(),
            "P1: removing nothing"
        );

        // P2. create_dir on an existing directory (error kind), and whether
        // create_dir_all on the same directory succeeds.
        let p2: (io::ErrorKind, bool) = guess();
        assert_eq!(
            p2,
            (
                fs::create_dir(&d.0).unwrap_err().kind(),
                fs::create_dir_all(&d.0).is_ok()
            ),
            "P2: creating what exists"
        );

        // P3. fs::write twice to one file: its length in bytes afterwards.
        let file = d.0.join("note.txt");
        fs::write(&file, "héllo").unwrap();
        fs::write(&file, "x").unwrap();
        let p3: u64 = guess();
        assert_eq!(
            p3,
            fs::metadata(&file).unwrap().len(),
            "P3: write truncates"
        );
    }
}
