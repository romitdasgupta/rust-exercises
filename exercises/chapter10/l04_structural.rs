//! LESSON 04 — Literal, tuple and struct patterns
//! Run: cargo test --test chapter10 l04_
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider: Does lowercase x in a pattern compare against an existing x or create a new binding? When should _ differ from _name?
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
    use practice::{alloc::count, guess};
    use std::cell::RefCell;
    #[test]
    fn a_specific_patterns_precede_general_ones() {
        assert_eq!(location(Point { x: 0, y: 0 }), "origin");
        assert_eq!(location(Point { x: 0, y: 2 }), "vertical");
        assert_eq!(location(Point { x: 3, y: 0 }), "horizontal");
        assert_eq!(location(Point { x: -1, y: 2 }), "plane");
        assert_eq!(location(Point { x: i32::MIN, y: 0 }), "horizontal");
    }
    #[test]
    fn b_tuple_pattern_ignores_the_middle() {
        assert_eq!(endpoint_sum((3, "unused".into(), -1)), 2);
        assert_eq!(
            endpoint_sum((i32::MAX, String::new(), i32::MAX)),
            4294967294
        );
        assert_eq!(endpoint_sum((i32::MIN, "🦀".into(), i32::MIN)), -4294967296);
    }
    #[test]
    fn b_ignored_string_is_not_cloned() {
        let input = (1, String::from("payload"), 2);
        let (sum, allocations) = count(|| endpoint_sum(input));
        assert_eq!(sum, 3);
        assert_eq!(allocations, 0, "ignore the String; do not copy it");
    }

    struct Noisy<'a>(&'static str, &'a RefCell<Vec<&'static str>>);
    impl Drop for Noisy<'_> {
        fn drop(&mut self) {
            self.1.borrow_mut().push(self.0);
        }
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    #[test]
    fn p_predictions() {
        // P1. `_` does not bind; `_held` binds and moves. Where does "payload"
        // appear in each log?
        let log = RefCell::new(Vec::new());
        {
            let value = Some(Noisy("payload", &log));
            match value {
                Some(_) => log.borrow_mut().push("arm"),
                None => log.borrow_mut().push("none"),
            }
            log.borrow_mut().push("after match");
        }
        let p1a: Vec<&str> = guess();
        assert_eq!(p1a, *log.borrow(), "P1a: matching with _");
        let log = RefCell::new(Vec::new());
        {
            let value = Some(Noisy("payload", &log));
            match value {
                Some(_held) => log.borrow_mut().push("arm"),
                None => log.borrow_mut().push("none"),
            }
            log.borrow_mut().push("after match");
        }
        let p1b: Vec<&str> = guess();
        assert_eq!(p1b, *log.borrow(), "P1b: matching with _held");

        // P2. A lowercase name in a pattern is a new binding, not a comparison.
        let x = 5;
        let y = match (1, 2) {
            (x, 2) => x * 10,
            _ => 0,
        };
        let p2: (i32, i32) = guess();
        assert_eq!(p2, (y, x), "P2: patterns bind, they do not compare");

        // P3. `field: name` binds the field to a new name.
        let Point { y: x, .. } = Point { x: 1, y: 2 };
        let p3: i32 = guess();
        assert_eq!(p3, x, "P3: renaming in struct patterns");
    }
}
