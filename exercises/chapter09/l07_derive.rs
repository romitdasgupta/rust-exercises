//! LESSON 07 — Deriving common traits
//! Run: cargo test --test chapter09 l07_
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider: Does deriving Clone on a struct containing String create independent text? Can that struct derive Copy?
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
    use practice::{alloc::count, guess};
    use std::rc::Rc;
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
        let plain = Settings::default();
        assert_eq!(
            variant(&plain, "→"),
            Settings {
                label: "→".into(),
                ..Settings::default()
            }
        );
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

    #[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
    struct Rank {
        level: u8,
        name: &'static str,
    }
    #[derive(Clone)]
    struct Shared {
        name: Rc<String>,
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    #[test]
    fn p_predictions() {
        // P1. Derived Ord compares fields in declaration order. Which name wins?
        let ranks = [
            Rank {
                level: 2,
                name: "b",
            },
            Rank {
                level: 1,
                name: "z",
            },
            Rank {
                level: 2,
                name: "a",
            },
        ];
        let p1: &str = guess();
        assert_eq!(p1, ranks.iter().max().unwrap().name, "P1: derived Ord");

        // P2. Write the exact derived Debug output.
        let p2: &str = guess();
        assert_eq!(
            p2,
            format!("{:?}", Settings::default()),
            "P2: derived Debug and Default"
        );

        // P3. Derived Clone on a struct holding an Rc: strong count afterward,
        // and heap allocations made by the clone.
        let a = Shared {
            name: Rc::new("cfg".into()),
        };
        let (b, allocations) = count(|| a.clone());
        let p3: (usize, usize) = guess();
        assert_eq!(
            p3,
            (Rc::strong_count(&a.name), allocations),
            "P3: Clone is field-wise"
        );
    }
}
