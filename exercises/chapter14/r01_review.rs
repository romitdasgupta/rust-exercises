//! REVIEW 01 — Spaced retrieval: Chapters 10–13 inside Chapter 14
//! Run: cargo test --test chapter14 r01_
//!
//! Do this WITHOUT reopening Chapters 10–13 or their hints. Struggling to recall
//! is what makes the knowledge stick; look things up only after a real attempt,
//! and note in NOTES.md anything you had to look up.
//!
//! Each task mixes an earlier idea (clone-on-write, blanket trait impls, enum
//! patterns) with this chapter's closures.

use std::borrow::Cow;

/// A. (Ch13 Cow + Ch14 FnMut) Call `fix` exactly once per item, in order. When it
/// returns None, keep the original item borrowed (no allocation); otherwise keep
/// the owned replacement it returned.
fn normalize_all<'a, F>(items: &[&'a str], mut fix: F) -> Vec<Cow<'a, str>>
where
    F: FnMut(&str) -> Option<String>,
{
    todo!("A: match on fix's result to choose Cow::Borrowed or Cow::Owned")
}

trait Rule {
    fn check(&self, value: i32) -> bool;
}

struct AtLeast(i32);

impl Rule for AtLeast {
    fn check(&self, value: i32) -> bool {
        value >= self.0
    }
}

/// B. (Ch11 blanket impls + Ch14 Fn) Every `Fn(i32) -> bool` closure is a Rule:
/// checking it calls the closure.
impl<F: Fn(i32) -> bool> Rule for F {
    fn check(&self, value: i32) -> bool {
        todo!("B: call the closure")
    }
}

/// B. True when every rule accepts `value` (true for no rules). Stop at the
/// first rule that rejects it.
fn passes_all(rules: &[&dyn Rule], value: i32) -> bool {
    todo!("B: dynamic dispatch through &dyn Rule")
}

#[derive(Debug)]
enum Event {
    Click { x: i32, y: i32 },
    Key(char),
    Quit,
}

/// C. (Ch10 patterns + Ch14 FnMut callbacks) Route events in order: a Click goes
/// to `on_click(x, y)` and a Key to `on_key(c)`. Ignore clicks with a negative
/// coordinate. Stop at the first Quit. Return how many events reached a callback.
fn route(
    events: &[Event],
    mut on_click: impl FnMut(i32, i32),
    mut on_key: impl FnMut(char),
) -> usize {
    todo!("C: one match with a guard; break on Quit")
}

#[cfg(test)]
mod tests {
    use super::*;
    use practice::guess;
    use std::cell::RefCell;

    #[test]
    fn a_unchanged_items_stay_borrowed() {
        let items = ["a b", "c", "d e"];
        let mut calls = 0;
        let out = normalize_all(&items, |s| {
            calls += 1;
            s.contains(' ').then(|| s.replace(' ', "_"))
        });
        assert_eq!(out, ["a_b", "c", "d_e"]);
        assert!(matches!(out[0], Cow::Owned(_)));
        assert!(matches!(out[1], Cow::Borrowed(_)));
        assert_eq!(out[1].as_ptr(), items[1].as_ptr());
        assert_eq!(calls, 3, "call fix once per item");
        assert!(normalize_all(&[], |_| panic!("no items")).is_empty());
    }

    #[test]
    fn b_closures_and_structs_are_both_rules() {
        let even = |n: i32| n % 2 == 0;
        let rules: [&dyn Rule; 2] = [&AtLeast(0), &even];
        assert!(passes_all(&rules, 4));
        assert!(!passes_all(&rules, 3));
        assert!(!passes_all(&rules, -2));
        assert!(passes_all(&[], 7));
        assert!(even.check(10));
    }

    #[test]
    fn b_passes_all_stops_at_the_first_rejection() {
        let unreachable = |_: i32| -> bool { panic!("stop at the first rejection") };
        assert!(!passes_all(&[&AtLeast(10), &unreachable], 0));
    }

    #[test]
    fn c_events_reach_the_matching_callback() {
        let events = [
            Event::Key('h'),
            Event::Click { x: 1, y: 2 },
            Event::Click { x: -1, y: 5 },
            Event::Click { x: 0, y: 0 },
            Event::Key('i'),
            Event::Quit,
            Event::Key('x'),
        ];
        let mut clicks = Vec::new();
        let mut typed = String::new();
        let handled = route(&events, |x, y| clicks.push((x, y)), |c| typed.push(c));
        assert_eq!(handled, 4);
        assert_eq!(clicks, [(1, 2), (0, 0)]);
        assert_eq!(typed, "hi");
        assert_eq!(route(&[Event::Quit, Event::Key('z')], |_, _| {}, |_| {}), 0);
        assert_eq!(route(&[], |_, _| {}, |_| {}), 0);
    }

    struct Noisy<'a>(&'static str, &'a RefCell<Vec<&'static str>>);
    impl Drop for Noisy<'_> {
        fn drop(&mut self) {
            self.1.borrow_mut().push(self.0);
        }
    }

    struct Pair<'a> {
        first: Noisy<'a>,
        second: Noisy<'a>,
    }

    /// P. Chapters 10–13 recall: predict each value BEFORE running.
    #[test]
    fn p_predictions() {
        // P1. (Ch13) In what order are a struct's fields dropped?
        let log = RefCell::new(Vec::new());
        drop(Pair {
            first: Noisy("first", &log),
            second: Noisy("second", &log),
        });
        let p1: Vec<&str> = guess();
        assert_eq!(p1, *log.borrow(), "P1: field drop order");

        // P2. (Ch12) Does a slice holding NaN contain NaN? Then partial_cmp and max.
        let p2: (bool, Option<std::cmp::Ordering>, f64) = guess();
        assert_eq!(
            p2,
            (
                [f64::NAN].contains(&f64::NAN),
                f64::NAN.partial_cmp(&1.0),
                f64::NAN.max(1.0)
            ),
            "P2: NaN is unordered"
        );

        // P3. (Ch10) Bytes in Option<Box<u8>> and in Option<u64>.
        let p3: (usize, usize) = guess();
        assert_eq!(
            p3,
            (size_of::<Option<Box<u8>>>(), size_of::<Option<u64>>()),
            "P3: niche optimization"
        );
    }
}
