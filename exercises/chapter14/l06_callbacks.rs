//! LESSON 06 — Callback registration and invocation
//! Run: cargo test --test chapter14 l06_
//!
//! PREDICT before executing: What lifetime does Box<dyn FnMut(&str)> imply here? Why must a
//! callback owning String use move?
//!
//! IMPLEMENT the documented tasks, then keep the tests and add an edge case.
//! EXPLAIN afterward: Why is a callback list homogeneous in its erased interface but
//! heterogeneous in concrete types? Why does dispatch need &mut self?
//!
//! COMPILER / MANUAL LAB (use examples/ch14_scratch.rs):
//! Register a closure borrowing a local String and observe the implicit 'static object
//! lifetime. Repair by moving owned data or parameterizing the registry by a lifetime.
//! Compare both API designs.
//!
//! TRANSFER: invent a new edge case, predict it, then add a check. A lesson is
//! complete only when its tests, predictions, lab, and explanation are done.

#[derive(Default)]
struct Callbacks {
    handlers: Vec<Box<dyn FnMut(&str) -> String>>,
}
impl Callbacks {
    /// A. Register an owning callback. No execution occurs during registration.
    fn register<F: FnMut(&str) -> String + 'static>(&mut self, f: F) {
        todo!("Task 1: implement the documented contract")
    }
    /// B. Invoke each callback once, in registration order, preserving its state.
    fn dispatch(&mut self, event: &str) -> Vec<String> {
        todo!("Task 2: implement the documented contract")
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a_registration_is_lazy_and_owns_captures() {
        let mut c = Callbacks::default();
        let prefix = String::from("owned:");
        c.register(move |s| format!("{}{}", prefix, s));
        assert_eq!(c.handlers.len(), 1);
    }
    #[test]
    fn b_callbacks_run_in_order_and_retain_mutable_state() {
        let mut c = Callbacks::default();
        c.register(|s| format!("a:{s}"));
        let mut calls = 0;
        c.register(move |s| {
            calls += 1;
            format!("{calls}:{s}")
        });
        assert_eq!(c.dispatch("x"), ["a:x", "1:x"]);
        assert_eq!(c.dispatch("y"), ["a:y", "2:y"]);
        assert!(Callbacks::default().dispatch("x").is_empty());
    }
}
