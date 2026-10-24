//! LESSON 01 — Traits, implementations, and defaults
//! Run: cargo test --test chapter11 l01_
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider: Can a default method call a required method?
//! Does importing a type also import every extension trait?
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
    use practice::guess;
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

    #[test]
    fn b_brief_calls_the_trait_method_not_a_copy_of_it() {
        struct Angle;
        impl Describe for Angle {
            fn description(&self) -> String {
                "angle".into()
            }
            fn brief(&self) -> String {
                "<angle>".into()
            }
        }
        assert_eq!(describe_brief(&Angle), "<angle>");
    }

    trait Greet {
        fn name(&self) -> String;
        fn hello(&self) -> String {
            format!("hi {}", self.name())
        }
    }
    struct Plain;
    impl Greet for Plain {
        fn name(&self) -> String {
            "plain".into()
        }
    }
    struct Loud;
    impl Greet for Loud {
        fn name(&self) -> String {
            "loud".into()
        }
        fn hello(&self) -> String {
            self.name().to_uppercase()
        }
    }

    trait Exclaim {
        fn exclaim(&self) -> String;
    }
    impl<T: std::fmt::Display + ?Sized> Exclaim for T {
        fn exclaim(&self) -> String {
            format!("{self}!")
        }
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    /// A wrong prediction is useful: read the failure, then explain it in NOTES.md.
    #[test]
    fn p_predictions() {
        // P1. Plain keeps the default `hello`; Loud overrides it. What does each return?
        let p1: (String, String) = guess();
        assert_eq!(p1, (Plain.hello(), Loud.hello()), "P1: default vs override");

        // P2. Implementing a trait adds methods. Does it add bytes to each value?
        let p2: usize = guess();
        assert_eq!(
            p2,
            size_of::<Plain>(),
            "P2: size of a unit struct with a trait"
        );

        // P3. A blanket impl gives `exclaim` to every Display type, including ones
        // this crate did not define.
        let p3: (String, String, String) = guess();
        assert_eq!(
            p3,
            (7.exclaim(), 'x'.exclaim(), 1.50_f64.exclaim()),
            "P3: blanket implementation"
        );
    }
}
