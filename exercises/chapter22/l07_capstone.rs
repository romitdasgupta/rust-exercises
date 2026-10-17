//! LESSON 07 — Capstone: structured events
//! Run: cargo test --test chapter22 l07_
//!
//! PREDICT before running:
//! 1. Can a macro check that every field name is a Rust identifier?
//! 2. What should duplicate field names mean in this API?
//!
//! IMPLEMENT the tasks; keep the tests.
//! EXPLAIN: Specify the event grammar, evaluation order, exported-helper paths, and diagnostics for unsupported syntax.
//! COMPILER LAB (examples/ch22_scratch.rs): Try an expression instead of an identifier field name, an omitted colon, and a value with a side effect. Improve error messages with a fallback compile_error! rule.

use std::collections::BTreeMap;
#[derive(Debug, PartialEq)]
struct Event {
    kind: String,
    fields: BTreeMap<String, String>,
}
/// A. Construct an Event from owned fields. Kind must be preserved verbatim.
fn make_event(kind: impl Into<String>, fields: BTreeMap<String, String>) -> Event {
    todo!("A: implement the contract")
}
/// B. Accept kind; field_ident: expression pairs, zero or more with optional
/// trailing comma. Evaluate kind first, each field value once in source order.
/// Stringify field names, Display-format values, and keep the last duplicate.
macro_rules! event {
    ($kind:expr; $($field:ident : $value:expr),* $(,)?) => {{ todo!("B: implement the contract") }};
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn event_grammar_builds_a_typed_value() {
        let value: Event = event!("job"; id:7,ok:true,);
        assert_eq!(
            value,
            Event {
                kind: "job".into(),
                fields: BTreeMap::from([("id".into(), "7".into()), ("ok".into(), "true".into())])
            }
        );
        let empty: Event = event!("empty";);
        assert!(empty.fields.is_empty());
    }
    #[test]
    fn event_evaluation_order_and_duplicates() {
        let mut n = 0;
        let e: Event = event!({n+=1;"ordered"}; x:{n+=1;n},x:{n+=1;n});
        assert_eq!(n, 3);
        assert_eq!(e.fields["x"], "3");
    }
}
