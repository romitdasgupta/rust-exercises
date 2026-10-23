//! Production Rust: guided practice with the crates and tools used in daily work.
//!
//! Each `eNN_*` module is one lesson; [`e07_capstone`] is also driven by the `jobq`
//! binary and the `tests/e07_capstone.rs` integration tests. Start with `README.md`.

// Starter bodies are `todo!()`, so their parameters are unused until you implement them.
#![allow(dead_code, unused_variables)]

pub mod e01_errors;
pub mod e02_serde;
pub mod e03_clap;
pub mod e04_tracing;
// proptest is a dev-dependency, so this lesson exists only in test builds.
#[cfg(test)]
mod e05_proptest;
pub mod e06_tokio;
pub mod e07_capstone;
pub mod e08_tooling;

/// Placeholder for a prediction. Replace the call with your answer before
/// running the lesson, for example `let p1: usize = 5;`.
#[track_caller]
pub fn guess<T>() -> T {
    panic!("PREDICT: replace `guess()` with your written prediction, then run again")
}
