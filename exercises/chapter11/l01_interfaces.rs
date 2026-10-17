//! LESSON 01 — Traits, implementations, and defaults
//! Run: cargo test --test chapter11 l01_
//!
//! PREDICT before executing: Can a default method call a required method? Does importing a
//! type also import every extension trait?
//!
//! IMPLEMENT the documented tasks, then keep the tests and add an edge case.
//! EXPLAIN afterward: Which method does your implementation supply, and which comes from
//! the trait? Why can two unrelated types implement one interface?
//!
//! COMPILER / MANUAL LAB (use examples/ch11_scratch.rs):
//! Copy the trait into the scratch file. Remove the required-method implementation; record
//! the error. Then override the default method and observe the changed behavior.
//!
//! TRANSFER: invent a new edge case, predict it, then add a check. A lesson is
//! complete only when its tests, predictions, lab, and explanation are done.

trait Describe {
    fn description(&self) -> String;
    fn brief(&self) -> String {
        format!("[{}]", self.description())
    }
}
struct Sensor {
    name: String,
    channels: usize,
}
impl Describe for Sensor {
    /// A. Return exactly NAME/CHANNELS, including zero channels.
    fn description(&self) -> String {
        todo!("Task 1: implement the documented contract")
    }
}
/// B. Use the trait's default brief method through a generic bound.
fn describe_brief<T: Describe>(value: &T) -> String {
    todo!("Task 2: implement the documented contract")
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a_implements_required_behavior() {
        assert_eq!(
            Sensor {
                name: "air".into(),
                channels: 3
            }
            .description(),
            "air/3"
        );
        assert_eq!(
            Sensor {
                name: "".into(),
                channels: 0
            }
            .description(),
            "/0"
        );
    }
    #[test]
    fn b_preserves_default_dispatch() {
        struct Custom;
        impl Describe for Custom {
            fn description(&self) -> String {
                "custom".into()
            }
        }
        assert_eq!(describe_brief(&Custom), "[custom]");
        assert_eq!(
            describe_brief(&Sensor {
                name: "🦀".into(),
                channels: 1
            }),
            "[🦀/1]"
        );
    }
}
