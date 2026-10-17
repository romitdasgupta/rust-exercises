//! LESSON 04 — Literal, tuple and struct patterns
//! Run: cargo test --test chapter10 l04_
//!
//! PREDICT before running: Does lowercase x in a pattern compare against an existing x or create a new binding? When should _ differ from _name?
//! IMPLEMENT the tasks below. Preserve the tests and add an edge case.
//! EXPLAIN afterward: Explain shadowing in patterns, ignored fields, and named versus positional destructuring.
//! COMPILER / TRANSFER LAB: Place a lowercase variable as the first match arm and observe unreachable later arms. Compare _ with _held when matching a String and then trying to use the source. Rewrite one nested tuple pattern using named fields.
//! Use examples/ch10_scratch.rs; repair intentional failures before committing.

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}
/// A. Use struct patterns: origin => "origin", x=0 => "vertical",
/// y=0 => "horizontal", otherwise => "plane". Order matters.
fn location(point: Point) -> &'static str {
    todo!("Implement this task")
}
/// B. Destructure the tuple parameter in the signature. Sum only first and
/// third as i64; ignore the owned String without cloning it.
fn endpoint_sum((first, _, third): (i32, String, i32)) -> i64 {
    todo!("Implement this task")
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a_specific_patterns_precede_general_ones() {
        assert_eq!(location(Point { x: 0, y: 0 }), "origin");
        assert_eq!(location(Point { x: 0, y: 2 }), "vertical");
        assert_eq!(location(Point { x: 3, y: 0 }), "horizontal");
        assert_eq!(location(Point { x: -1, y: 2 }), "plane");
    }
    #[test]
    fn b_tuple_pattern_ignores_the_middle() {
        assert_eq!(endpoint_sum((3, "unused".into(), -1)), 2);
        assert_eq!(
            endpoint_sum((i32::MAX, String::new(), i32::MAX)),
            4294967294
        );
    }
}
