//! LESSON 06 — Infallible and fallible conversions
//! Run: cargo test --test chapter13 l06_
//!
//! PREDICT before executing: If From<A> for B exists, must Into<B> for A be written
//! separately? Should an out-of-range numeric conversion use From?
//!
//! IMPLEMENT the documented tasks, then keep the tests and add an edge case.
//! EXPLAIN afterward: Why should From avoid failure and loss of value? Where does TryFrom
//! communicate validation failure?
//!
//! COMPILER / MANUAL LAB (use examples/ch13_scratch.rs):
//! Attempt both From<u16> for Port and TryFrom<u16> for Port in scratch. Explain the
//! overlapping blanket implementation diagnostic. Compare u8::try_from(300_u16) with
//! 300_u16 as u8.
//!
//! TRANSFER: invent a new edge case, predict it, then add a check. A lesson is
//! complete only when its tests, predictions, lab, and explanation are done.

#[derive(Debug, PartialEq, Eq)]
struct Label(String);
impl From<String> for Label {
    /// A. Consume the owned string without cloning it.
    fn from(value: String) -> Self {
        todo!("Task 1: implement the documented contract")
    }
}
#[derive(Debug, PartialEq, Eq)]
struct Port(u16);
impl TryFrom<u32> for Port {
    type Error = &'static str;
    /// B. Accept 1..=65535, otherwise Err("port out of range").
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        todo!("Task 2: implement the documented contract")
    }
}
/// C. Accept any infallible Label source through Into<Label>.
fn label_text<L: Into<Label>>(label: L) -> String {
    todo!("Task 3: implement the documented contract")
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a_from_provides_into_and_consumes_ownership() {
        let text = String::from("name");
        let ptr = text.as_ptr();
        let label: Label = text.into();
        assert_eq!(label.0, "name");
        assert_eq!(label.0.as_ptr(), ptr);
    }
    #[test]
    fn b_try_from_checks_both_boundaries() {
        assert_eq!(Port::try_from(1), Ok(Port(1)));
        assert_eq!(Port::try_from(65535), Ok(Port(65535)));
        assert_eq!(Port::try_from(0), Err("port out of range"));
        assert_eq!(Port::try_from(65536), Err("port out of range"));
        let p: Result<Port, _> = 7_u32.try_into();
        assert_eq!(p, Ok(Port(7)));
    }
    #[test]
    fn c_into_bound_accepts_converted_and_original_values() {
        assert_eq!(label_text(String::from("x")), "x");
        assert_eq!(label_text(Label("y".into())), "y");
    }
}
