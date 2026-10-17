//! LESSON 05 — Attributes, cfg and included files
//! Run: cargo test --test chapter08 l05_
//!
//! PREDICT before running: Is code behind #[cfg(FALSE)] type-checked? Does cfg!(FALSE) remove the unselected branch from type checking?
//! IMPLEMENT the tasks below. Preserve the tests and add an edge case.
//! EXPLAIN afterward: Explain conditional compilation versus runtime branching, environment capture at compilation, and file inclusion paths.
//! COMPILER / TRANSFER LAB: Compare #[cfg(any())] on a function with if cfg!(any()) around an invalid expression. Add a feature in a disposable package and compile both ways. Add build.rs that prints cargo::rustc-env=GUIDED_MODE=lab, read it using env!, then explain OUT_DIR and rerun-if-changed. Generate a Rust constant file into OUT_DIR and include! it; compare include!, include_str!, include_bytes!. Never edit this project manifest for the experiment.
//! Use examples/ch08_scratch.rs; repair intentional failures before committing.

static EMBEDDED: &str = include_str!("included_labels.txt");
/// A. Return whether this target belongs to the Unix family using cfg!.
fn unix_target() -> bool {
    todo!("Implement this task")
}
/// B. Borrow nonempty trimmed lines from the embedded file, in order. No
/// per-line String allocation; returned text has static lifetime.
fn embedded_labels() -> Vec<&'static str> {
    todo!("Implement this task")
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a_target_attribute_is_compile_time_information() {
        assert_eq!(unix_target(), cfg!(target_family = "unix"));
    }
    #[test]
    fn b_file_is_embedded_and_borrowed() {
        assert_eq!(embedded_labels(), ["north", "港", "south"]);
        let first = embedded_labels()[0];
        assert!(first.as_ptr() as usize >= EMBEDDED.as_ptr() as usize);
        assert!((first.as_ptr() as usize) < EMBEDDED.as_ptr() as usize + EMBEDDED.len());
    }
}
