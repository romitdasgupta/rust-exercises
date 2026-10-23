//! LESSON 06 — Callback registration and invocation
//! Run: cargo test --test chapter14 l06_
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider: What lifetime does Box<dyn FnMut(&str)>
//! imply here? Why must a callback owning String use move?
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

type Callback = Box<dyn FnMut(&str) -> String>;

#[derive(Default)]
struct Callbacks {
    handlers: Vec<Callback>,
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
    use practice::guess;
    use std::cell::RefCell;
    use std::rc::Rc;
    #[test]
    fn a_registration_is_lazy_and_owns_captures() {
        let mut c = Callbacks::default();
        let prefix = String::from("owned:");
        c.register(move |s| format!("{}{}", prefix, s));
        assert_eq!(c.handlers.len(), 1);
        c.register(|_| panic!("register must not call the callback"));
        assert_eq!(c.handlers.len(), 2);
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

    struct Noisy<'a>(&'static str, &'a RefCell<Vec<&'static str>>);
    impl Noisy<'_> {
        fn note(&self, event: &'static str) {
            self.1.borrow_mut().push(event);
        }
    }
    impl Drop for Noisy<'_> {
        fn drop(&mut self) {
            self.note(self.0);
        }
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    /// A wrong prediction is useful: read the failure, then explain it in NOTES.md.
    #[test]
    fn p_predictions() {
        // P1. Machine words in one Box<dyn FnMut(&str) -> String> list entry.
        let p1: usize = guess();
        assert_eq!(
            p1 * size_of::<usize>(),
            size_of::<Box<dyn FnMut(&str) -> String>>(),
            "P1: data pointer plus vtable pointer"
        );

        // P2. Two callbacks each own a clone of one Rc.
        // Strong count while registered, and after the list is dropped?
        let shared = Rc::new(RefCell::new(Vec::<String>::new()));
        type Handler = Box<dyn FnMut(&str)>;
        let mut handlers: Vec<Handler> = Vec::new();
        for tag in ["a", "b"] {
            let sink = Rc::clone(&shared);
            handlers.push(Box::new(move |e| {
                sink.borrow_mut().push(format!("{tag}:{e}"))
            }));
        }
        for h in &mut handlers {
            h("go");
        }
        let registered = Rc::strong_count(&shared);
        drop(handlers);
        let p2: (usize, usize) = guess();
        assert_eq!(
            p2,
            (registered, Rc::strong_count(&shared)),
            "P2: callbacks own their captures"
        );
        assert_eq!(*shared.borrow(), ["a:go", "b:go"]);

        // P3. When is a callback's captured token dropped?
        let log = RefCell::new(Vec::new());
        let token = Noisy("token dropped", &log);
        let mut handlers: Vec<Box<dyn FnMut() + '_>> = Vec::new();
        handlers.push(Box::new(move || token.note("called")));
        for _ in 0..2 {
            for h in &mut handlers {
                h();
            }
        }
        log.borrow_mut().push("before drop(handlers)");
        drop(handlers);
        let p3: Vec<&str> = guess();
        assert_eq!(
            p3,
            *log.borrow(),
            "P3: captures live as long as the callback"
        );
    }
}
