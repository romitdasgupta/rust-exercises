//! LESSON 06 — Directory metadata
//! Run: cargo test --test chapter18 l06_
//!
//! PREDICT before running:
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
    fn file_names_sorted_and_nonrecursive() {
        let d = fixture();
        fs::write(d.0.join("z"), b"z").unwrap();
        fs::write(d.0.join("a"), b"a").unwrap();
        fs::create_dir(d.0.join("sub")).unwrap();
        fs::write(d.0.join("sub/nested"), b"x").unwrap();
        #[cfg(unix)]
        std::os::unix::fs::symlink(d.0.join("z"), d.0.join("link")).unwrap();
        assert_eq!(
            regular_files(&d.0).unwrap(),
            [OsString::from("a"), OsString::from("z")]
        );
        assert!(exists_result(&d.0.join("z")).unwrap());
        assert!(!exists_result(&d.0.join("absent")).unwrap());
        assert!(regular_files(&d.0.join("absent")).is_err());
    }
}
