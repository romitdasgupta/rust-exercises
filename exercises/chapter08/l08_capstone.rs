//! LESSON 08 — Capstone: library facade and workspace
//! Run: cargo test --test chapter08 l08_
//!
//! PREDICT before running: Which code belongs in a reusable library and which belongs in a CLI binary? How can a private implementation module remain refactorable?
//! IMPLEMENT the tasks below. Preserve the tests and add an edge case.
//! EXPLAIN afterward: Explain the public facade and the external test that proves its privacy. Audit exactly what a package archive contains.
//! COMPILER / TRANSFER LAB: Create a disposable workspace with a library label_core and a CLI label_cli using a path dependency. Move the facade API there, split validation into its own file, add a second binary under src/bin, a public integration test, and a doc-test. Run cargo test --workspace, cargo fmt --check, cargo clippy --workspace --all-targets, cargo doc --workspace --no-deps, then cargo package --list and cargo package --allow-dirty --no-verify for the LIBRARY only; inspect the .crate archive locally. Add package metadata/readme/license before packaging. Do NOT run cargo publish or upload anything. Explain excluded files, Cargo.lock policy, workspace resolver/edition, and the public API compatibility promise.
//! Use examples/ch08_scratch.rs; repair intentional failures before committing.

mod core {
    fn validate(name: &str) -> Option<&str> {
        let name = name.trim();
        if name.is_empty() { None } else { Some(name) }
    }
    /// A. Return "tag:NAME" after private validation; reject empty labels.
    pub fn tag(name: &str) -> Option<String> {
        todo!("Implement this task")
    }
}
pub use core::tag;
/// B. Apply the public API to all inputs in order. Return Err(zero_based_index)
/// at the first invalid name; no partial successful vector on error.
fn tag_batch(names: &[&str]) -> Result<Vec<String>, usize> {
    todo!("Implement this task")
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a_facade_exposes_behavior() {
        assert_eq!(tag(" 港 "), Some("tag:港".into()));
        assert_eq!(tag(" \n"), None);
    }
    #[test]
    fn b_library_api_is_independent_of_cli() {
        assert_eq!(
            tag_batch(&[" a ", "b"]),
            Ok(vec!["tag:a".into(), "tag:b".into()])
        );
        assert_eq!(tag_batch(&["a", "", "b"]), Err(1));
        assert_eq!(tag_batch(&[]), Ok(vec![]));
    }
}
