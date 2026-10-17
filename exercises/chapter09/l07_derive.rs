//! LESSON 07 — Deriving common traits
//! Run: cargo test --test chapter09 l07_
//!
//! PREDICT before running: Does deriving Clone on a struct containing String create independent text? Can that struct derive Copy?
//! IMPLEMENT the tasks below. Preserve the tests and add an edge case.
//! EXPLAIN afterward: Explain field-by-field derived traits, Debug versus Display, and why derived Default is not automatically a valid domain default.
//! COMPILER / TRANSFER LAB: Derive Copy on a struct of integers, then add Vec<u8>. Remove an inner field trait and inspect the resulting outer derive error. Use a domain type whose zero value is invalid and explain why blindly deriving Default breaks its intended API.
//! Use examples/ch09_scratch.rs; repair intentional failures before committing.

#[derive(Clone, Debug, PartialEq, Eq, Default)]
struct Settings {
    label: String,
    retries: u8,
    enabled: bool,
}
/// A. Clone settings, replace the cloned label by appending suffix, and leave
/// original settings unchanged. Explicitly use the derived Clone implementation.
fn variant(settings: &Settings, suffix: &str) -> Settings {
    todo!("Implement this task")
}
/// B. Return a derived default Settings, then set only enabled to true using
/// struct update syntax and Default::default().
fn enabled_defaults() -> Settings {
    todo!("Implement this task")
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a_derived_clone_is_fieldwise() {
        let s = Settings {
            label: "base".into(),
            retries: 3,
            enabled: true,
        };
        let v = variant(&s, "!");
        assert_eq!(s.label, "base");
        assert_eq!(
            v,
            Settings {
                label: "base!".into(),
                retries: 3,
                enabled: true
            }
        );
        assert_ne!(v.label.as_ptr(), s.label.as_ptr());
        assert!(format!("{s:?}").contains("Settings"));
    }
    #[test]
    fn b_default_and_update_have_explicit_policy() {
        assert_eq!(
            enabled_defaults(),
            Settings {
                label: String::new(),
                retries: 0,
                enabled: true
            }
        );
        assert_ne!(enabled_defaults(), Settings::default());
    }
}
