//! LESSON 01 — Variants and payloads
//! Run: cargo test --test chapter10 l01_
//!
//! PREDICT before running: Can a data-carrying enum be cast to an integer like a fieldless repr(u8) enum? Can two variants carry different types?
//! IMPLEMENT the tasks below. Preserve the tests and add an edge case.
//! EXPLAIN afterward: Explain the discriminant, payload, exhaustive match, and why default enum layout is not a stable serialization format.
//! COMPILER / TRANSFER LAB: Inspect size_of and align_of for a fieldless enum, an enum with String and integer payloads, and Option<Box<i32>>. Separate observations from layout guarantees; explain the documented null-pointer optimization only for supported types. Add a variant and let the compiler identify matches needing updates.
//! Use examples/ch10_scratch.rs; repair intentional failures before committing.

#[derive(Debug, PartialEq)]
enum Reply {
    Pending,
    Count(u32),
    Text(String),
}
/// A. Classify every variant as "pending", "count", or "text".
fn kind(reply: &Reply) -> &'static str {
    todo!("Implement this task")
}
/// B. Consume a reply and render Pending => "waiting", Count(n) => "count:N",
/// Text(s) => s unchanged (move its owner, do not clone).
fn render(reply: Reply) -> String {
    todo!("Implement this task")
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a_each_state_has_one_variant() {
        assert_eq!(kind(&Reply::Pending), "pending");
        assert_eq!(kind(&Reply::Count(0)), "count");
        assert_eq!(kind(&Reply::Text(String::new())), "text");
    }
    #[test]
    fn b_payload_ownership_moves_out() {
        assert_eq!(render(Reply::Pending), "waiting");
        assert_eq!(render(Reply::Count(7)), "count:7");
        let s = "港".to_owned();
        let p = s.as_ptr();
        let out = render(Reply::Text(s));
        assert_eq!(out, "港");
        assert_eq!(out.as_ptr(), p);
    }
}
