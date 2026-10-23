//! REVIEW 01 — Spaced retrieval: Chapters 9, 11, and 12 inside Chapter 13
//! Run: cargo test --test chapter13 r01_
//!
//! Do this WITHOUT reopening Chapters 9, 11, or 12 or their hints. Struggling
//! to recall is what makes the knowledge stick; look things up only after a
//! real attempt, and note in NOTES.md anything you had to look up.
//!
//! Each task mixes an earlier idea (operator traits, trait objects, struct
//! update syntax) with this chapter's utility traits.

use std::cell::RefCell;
use std::ops::Add;
use std::rc::Rc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Cents(u64);

impl From<(u32, u32)> for Cents {
    /// A1. (Ch13 From) Convert (dollars, cents) to a total in cents. The cents
    /// part may exceed 99: (1, 150) is Cents(250). Never overflow.
    fn from((dollars, cents): (u32, u32)) -> Self {
        todo!("A1: widen both parts to u64 before multiplying")
    }
}

impl Add for Cents {
    type Output = Cents;
    /// A2. (Ch12 operators) Sum two amounts.
    fn add(self, rhs: Cents) -> Cents {
        todo!("A2: add the inner values")
    }
}

type Log = Rc<RefCell<Vec<String>>>;

trait Job {
    fn run(&self);
}

struct Step {
    name: &'static str,
    log: Log,
}

impl Job for Step {
    fn run(&self) {
        self.log.borrow_mut().push(format!("run {}", self.name));
    }
}

impl Drop for Step {
    fn drop(&mut self) {
        self.log.borrow_mut().push(format!("drop {}", self.name));
    }
}

/// B. (Ch11 trait objects + Ch13 Drop) Run the jobs in order. Each job must be
/// dropped right after it runs, before the next one starts, so its resources
/// are released as early as possible.
fn run_all(jobs: Vec<Box<dyn Job>>) {
    todo!("B: which kind of loop gives you each Box by value?")
}

#[derive(Debug, PartialEq)]
struct Settings {
    retries: u8,
    name: String,
    verbose: bool,
}

impl Default for Settings {
    /// C1. (Ch13 Default) 3 retries, name "anonymous", not verbose.
    fn default() -> Self {
        todo!("C1: spell out every field")
    }
}

/// C2. (Ch9 struct update syntax) Default settings with the given name.
/// Name only the field that changes.
fn named(name: &str) -> Settings {
    todo!("C2: Settings {{ name: ..., ..Default::default() }}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use practice::guess;

    #[test]
    fn a_conversion_and_addition_compose() {
        assert_eq!(Cents::from((1, 150)), Cents(250));
        assert_eq!(Cents::from((0, 0)), Cents(0));
        let total: Cents = Cents::from((2, 5)) + (0, 95).into();
        assert_eq!(total, Cents(300));
        assert_eq!(
            Cents::from((u32::MAX, u32::MAX)),
            Cents(u64::from(u32::MAX) * 101)
        );
    }

    #[test]
    fn b_each_job_is_dropped_right_after_it_runs() {
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        let job = |name| -> Box<dyn Job> {
            Box::new(Step {
                name,
                log: Rc::clone(&log),
            })
        };
        run_all(vec![job("a"), job("b")]);
        assert_eq!(*log.borrow(), ["run a", "drop a", "run b", "drop b"]);
        run_all(Vec::new());
        assert_eq!(log.borrow().len(), 4);
        assert_eq!(Rc::strong_count(&log), 1, "every job was dropped");
    }

    #[test]
    fn c_defaults_and_update_syntax() {
        assert_eq!(
            Settings::default(),
            Settings {
                retries: 3,
                name: "anonymous".into(),
                verbose: false
            }
        );
        let s = named("é");
        assert_eq!((s.retries, s.name.as_str(), s.verbose), (3, "é", false));
    }

    #[derive(PartialEq, PartialOrd)]
    enum Level {
        Low = 10,
        High = 1,
    }

    fn bits<T>() -> usize {
        size_of::<T>() * 8
    }

    /// P. Chapters 9, 10, 11, and 12 recall: predict each value BEFORE running.
    #[test]
    fn p_predictions() {
        // P1 (Ch12). Derived PartialOrd on an enum with explicit discriminants.
        let p1: bool = guess();
        assert_eq!(p1, Level::Low < Level::High, "P1: derived enum order");

        // P2 (Ch11). One generic function, two monomorphized copies.
        let p2: (usize, usize) = guess();
        assert_eq!(p2, (bits::<u16>(), bits::<char>()), "P2: size_of::<T>");

        // P3 (Ch10). A binding with @ and a range pattern.
        let doubled = match Some(3) {
            Some(n @ 1..=5) => n * 2,
            _ => 0,
        };
        let p3: i32 = guess();
        assert_eq!(p3, doubled, "P3: @ bindings");

        // P4 (Ch9). Size in bytes of a (u8, u32) tuple.
        let p4: usize = guess();
        assert_eq!(p4, size_of::<(u8, u32)>(), "P4: padding and alignment");
    }
}
