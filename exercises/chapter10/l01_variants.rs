//! LESSON 01 — Variants and payloads
//! Run: cargo test --test chapter10 l01_
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider: Can a data-carrying enum be cast to an integer like a fieldless repr(u8) enum? Can two variants carry different types?
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
    use practice::{alloc::count, guess};
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
        assert_eq!(render(Reply::Count(u32::MAX)), "count:4294967295");
        assert_eq!(render(Reply::Text(String::new())), "");
    }
    #[test]
    fn b_text_payload_moves_without_allocating() {
        let reply = Reply::Text("ready".into());
        let (out, allocations) = count(|| render(reply));
        assert_eq!(out, "ready");
        assert_eq!(allocations, 0, "the Text payload should move out");
    }

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    enum Size {
        Small,
        Large(u8),
    }
    enum Level {
        Low = 1,
        Mid,
        High = 10,
        Top,
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    #[test]
    fn p_predictions() {
        // P1. Discriminants continue from the previous one.
        let p1: (i32, i32) = guess();
        assert_eq!(
            p1,
            (Level::Mid as i32, Level::Top as i32),
            "P1: implicit discriminants"
        );

        // P2. How many machine words (usize) does Option<Box<i32>> take?
        let p2: usize = guess();
        assert_eq!(
            p2 * size_of::<usize>(),
            size_of::<Option<Box<i32>>>(),
            "P2: the null-pointer optimization"
        );

        // P3. Derived ordering on an enum: write the sorted vector.
        let mut sizes = vec![Size::Large(1), Size::Small, Size::Large(0)];
        sizes.sort();
        let p3: Vec<Size> = guess();
        assert_eq!(p3, sizes, "P3: variants order by declaration");

        // P4. Do these two values have the same discriminant?
        let p4: bool = guess();
        assert_eq!(
            p4,
            std::mem::discriminant(&Reply::Count(1)) == std::mem::discriminant(&Reply::Count(2)),
            "P4: discriminant ignores the payload"
        );
    }
}
