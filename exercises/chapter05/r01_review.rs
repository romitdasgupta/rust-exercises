//! REVIEW 01 — Spaced retrieval: Chapters 3 and 4 inside Chapter 5
//! Run: cargo test --test chapter05 r01_
//!
//! Do this WITHOUT reopening Chapters 3 or 4 or their hints. Struggling to
//! recall is what makes the knowledge stick; look things up only after a real
//! attempt, and note in NOTES.md anything you had to look up.
//!
//! Each task mixes an earlier idea (moving out of a vector, overflow policies,
//! UTF-8 boundaries) with this chapter's borrowing rules.

/// A. (Ch4 indexed moves + Ch5 last use) Remove and return the FIRST name with
/// the most Unicode scalars, keeping the others in their original order.
/// None for an empty list. Find it through a borrow, end that borrow, then
/// move the String out. Do not clone any String.
fn take_longest(names: &mut Vec<String>) -> Option<String> {
    todo!("A: borrow to choose an index, then mutate")
}

/// B. (Ch3 overflow + Ch5 mutable references) Add `amount` to every counter in
/// place, clamping at u8::MAX. Return how many counters overflowed, that is,
/// whose exact sum would have exceeded 255. A sum of exactly 255 fits.
fn bump_all(counters: &mut [u8], amount: u8) -> usize {
    todo!("B: mutate through iter_mut with an explicit overflow policy")
}

/// C. (Ch3 UTF-8 + Ch5 returning references) Borrow the longest prefix of
/// `text` that is at most `max_bytes` bytes long and never splits a Unicode
/// scalar. Return a borrow of `text`; do not allocate.
fn truncate_bytes(text: &str, max_bytes: usize) -> &str {
    todo!("C: back up to a char boundary")
}

#[cfg(test)]
mod tests {
    use super::*;
    use practice::{alloc::count, guess};
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn a_longest_is_moved_out_in_order() {
        let mut names: Vec<String> = ["ab", "ñ🦀", "abcd", "wxyz", "d"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let p = names[2].as_ptr();
        let (taken, allocations) = count(|| take_longest(&mut names));
        let taken = taken.unwrap();
        assert_eq!(taken, "abcd", "most scalars, first of a tie");
        assert_eq!(taken.as_ptr(), p, "move the String out; don't clone it");
        assert_eq!(names, ["ab", "ñ🦀", "wxyz", "d"]);
        assert_eq!(allocations, 0);
        assert_eq!(take_longest(&mut names).as_deref(), Some("wxyz"));
        assert_eq!(take_longest(&mut names).as_deref(), Some("ab"));
        assert_eq!(take_longest(&mut vec![]), None);
    }

    #[test]
    fn b_counters_saturate_and_report_overflow() {
        let mut counters = [250, 5, 255, 0, 251];
        assert_eq!(bump_all(&mut counters, 5), 2);
        assert_eq!(counters, [255, 10, 255, 5, 255]);
        assert_eq!(bump_all(&mut counters, 0), 0);
        assert_eq!(counters, [255, 10, 255, 5, 255]);
        assert_eq!(bump_all(&mut [], 9), 0);
    }

    #[test]
    fn c_truncation_borrows_and_respects_boundaries() {
        let text = String::from("ñ🦀rust");
        let (cut, allocations) = count(|| truncate_bytes(&text, 4));
        assert_eq!(cut, "ñ");
        assert_eq!(cut.as_ptr(), text.as_ptr());
        assert_eq!(allocations, 0);
        assert_eq!(truncate_bytes(&text, 6), "ñ🦀");
        assert_eq!(truncate_bytes(&text, 7), "ñ🦀r");
        assert_eq!(truncate_bytes(&text, 1), "");
        assert_eq!(truncate_bytes(&text, 100), text);
        assert_eq!(truncate_bytes("🦀", 3), "");
        assert_eq!(truncate_bytes("", 0), "");
    }

    struct Noisy<'a>(&'static str, &'a RefCell<Vec<&'static str>>);
    impl Drop for Noisy<'_> {
        fn drop(&mut self) {
            self.1.borrow_mut().push(self.0);
        }
    }

    /// P. Chapter 3 and 4 recall: predict each value BEFORE running.
    #[test]
    fn p_predictions() {
        // P1. (Ch4) `moved` is declared last. In what order are the owners dropped?
        let log = RefCell::new(Vec::new());
        {
            let a = Noisy("a", &log);
            let _b = Noisy("b", &log);
            let _moved = a;
        }
        let p1: Vec<&str> = guess();
        assert_eq!(p1, *log.borrow(), "P1: drop order follows the owners");

        // P2. (Ch4) Contents after swap_remove(1).
        let mut v = vec!['a', 'b', 'c', 'd'];
        v.swap_remove(1);
        let p2: Vec<char> = guess();
        assert_eq!(p2, v, "P2: swap_remove");

        // P3. (Ch3) Integer and float casts with `as`.
        let (minus_one, almost_four, too_big) = (-1_i32, -3.99_f64, 300.0_f64);
        let p3: (u8, i32, u8) = guess();
        assert_eq!(
            p3,
            (minus_one as u8, almost_four as i32, too_big as u8),
            "P3: as truncates integers, and truncates and saturates floats"
        );

        // P4. (Ch4) The strong count after these clones and drops.
        let a = Rc::new(5);
        let b = Rc::clone(&a);
        let c = Rc::clone(&b);
        drop(a);
        let p4: usize = guess();
        assert_eq!(p4, Rc::strong_count(&c), "P4: Rc counts owners");
    }
}
