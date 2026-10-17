//! LESSON 07 — Dependency requirements and lockfiles
//! Run: cargo test --test chapter08 l07_
//!
//! PREDICT before running: Does a dependency requirement select one exact resolved version? Does a workspace have one lockfile or one per member?
//! IMPLEMENT the tasks below. Preserve the tests and add an edge case.
//! EXPLAIN afterward: Explain compatible version requirements, the pre-1.0 exception, lockfile reproducibility, and why path dependencies are useful for offline experiments.
//! COMPILER / TRANSFER LAB: Create /tmp/guided-deps with two local crates codec and app. Add codec by a relative path dependency, call its public function, and inspect cargo tree and Cargo.lock. Set codec version to 0.2.3 and a path+version requirement of "0.2" in app; then change codec to 0.3.0 and observe rejection. Restore, use cargo build --locked, and inspect the difference after a manifest change. For a cached external crate compare "1.2", "=1.2.3", and ">=1.2, <2" using cargo metadata; this last experiment is optional if no cached crate is available.
//! Use examples/ch08_scratch.rs; repair intentional failures before committing.

/// A. Model the stable numeric requirement ^1.2.3 (no prereleases/build tags):
/// accepts >=1.2.3 and <2.0.0. Input is (major,minor,patch).
fn accepts_1_2_3(version: (u32, u32, u32)) -> bool {
    todo!("Implement this task")
}
/// B. Model the stable numeric requirement ^0.2.3: >=0.2.3 and <0.3.0.
/// This is a limited exercise, NOT a general Cargo version resolver.
fn accepts_0_2_3(version: (u32, u32, u32)) -> bool {
    todo!("Implement this task")
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a_nonzero_major_compatibility() {
        assert!(!accepts_1_2_3((1, 2, 2)));
        assert!(accepts_1_2_3((1, 2, 3)));
        assert!(accepts_1_2_3((1, 9, 0)));
        assert!(!accepts_1_2_3((2, 0, 0)));
        assert!(!accepts_1_2_3((0, 99, 99)));
    }
    #[test]
    fn b_zero_major_has_narrower_compatibility() {
        assert!(!accepts_0_2_3((0, 2, 2)));
        assert!(accepts_0_2_3((0, 2, 3)));
        assert!(accepts_0_2_3((0, 2, 99)));
        assert!(!accepts_0_2_3((0, 3, 0)));
        assert!(!accepts_0_2_3((1, 2, 3)));
    }
}
