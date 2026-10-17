//! LESSON 01 — Crates, editions and profiles
//! Run: cargo test --test chapter08 l01_
//!
//! PREDICT before running: Does edition select a compiler version? Does an integration test compile as a separate crate? Which profile enables debug_assertions by default?
//! IMPLEMENT the tasks below. Preserve the tests and add an edge case.
//! EXPLAIN afterward: Distinguish package, crate, target, compiler version, edition, rust-version, and build profile.
//! COMPILER / TRANSFER LAB: In /tmp create a new package with cargo new, inspect cargo metadata --no-deps, rustc --version, cargo --version, and rustup show. Add rust-version to its manifest; explain MSRV versus installed toolchain. Compare cargo check, build, test, and build --release outputs; inspect target directory. Keep the real workspace manifest intact.
//! Use examples/ch08_scratch.rs; repair intentional failures before committing.

/// A. Return "checked" when debug_assertions are enabled, "optimized" when
/// they are disabled. This label follows cfg, not a claim about every optimizer.
fn profile_label() -> &'static str {
    todo!("Implement this task")
}
/// B. Return the current package name and version as compile-time static text.
fn package_identity() -> (&'static str, &'static str) {
    todo!("Implement this task")
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a_cfg_tracks_the_selected_profile() {
        assert_eq!(
            profile_label(),
            if cfg!(debug_assertions) {
                "checked"
            } else {
                "optimized"
            }
        );
    }
    #[test]
    fn b_metadata_belongs_to_the_compiling_package() {
        assert_eq!(
            package_identity(),
            (env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
        );
        assert!(!package_identity().0.is_empty());
    }
}
