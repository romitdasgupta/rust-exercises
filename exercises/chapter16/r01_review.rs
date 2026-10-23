//! REVIEW 01 — Spaced retrieval: earlier chapters inside Chapter 16
//! Run: cargo test --test chapter16 r01_
//!
//! Do this WITHOUT reopening earlier chapters or their hints. Struggling to
//! recall is what makes the knowledge stick; look things up only after a real
//! attempt, and note in NOTES.md anything you had to look up.
//!
//! Each task mixes an earlier idea (iterator adapters, generic closures, error
//! enums) with this chapter's collections.

use std::collections::{BTreeMap, HashMap};

/// A. (Ch15 iterators + Ch16 maps) Count whitespace-separated words exactly
/// (case-sensitive). Return the `n` most frequent as (word, count), count
/// descending, then word ascending. Borrow each word from `text`: do not build
/// a String per word.
fn top_words(text: &str, n: usize) -> Vec<(&str, usize)> {
    todo!("A: a map of borrowed keys, then an explicit sort")
}

/// B. (Ch14 closures + Ch16 tree maps) Group items by the key the closure
/// computes. Keys come out in ascending order; each group keeps input order.
/// Move the items: `T` need not be Clone.
fn group_by<T, K: Ord>(items: Vec<T>, key: impl Fn(&T) -> K) -> BTreeMap<K, Vec<T>> {
    todo!("B: compute the key before moving the item into the map")
}

#[derive(Debug, PartialEq)]
enum ConfigError {
    /// 1-based line number of a nonblank line that is not `key=value` with a
    /// nonempty key and a u32 value.
    Malformed(usize),
    /// A repeated key, with the 1-based line number where it repeats.
    Duplicate(String, usize),
}

/// C. (Ch7 errors + Ch10 enums + Ch16 entries) Parse `key=value` lines into a
/// map. Trim keys and values, skip blank lines, split on the first '='.
/// Stop at the first problem and return it.
fn parse_config(text: &str) -> Result<BTreeMap<String, u32>, ConfigError> {
    todo!("C: match on the entry to tell a new key from a repeat")
}

#[cfg(test)]
mod tests {
    use super::*;
    use practice::{alloc::count, guess};

    #[test]
    fn a_top_words_orders_ties_and_borrows() {
        let text = "b a  c\nb é a b";
        assert_eq!(top_words(text, 2), [("b", 3), ("a", 2)]);
        assert_eq!(top_words(text, 9), [("b", 3), ("a", 2), ("c", 1), ("é", 1)]);
        assert!(top_words(text, 0).is_empty());
        assert!(top_words(" \n ", 3).is_empty());
        assert_eq!(top_words("Go go", 5), [("Go", 1), ("go", 1)]);
    }

    #[test]
    fn a_top_words_does_not_allocate_per_word() {
        let text = "red green blue ".repeat(300);
        let (top, allocations) = count(|| top_words(&text, 1));
        assert_eq!(top, [("blue", 300)]);
        assert!(
            allocations <= 10,
            "900 words cost {allocations} allocations"
        );
    }

    #[test]
    fn b_group_by_moves_items_and_keeps_order() {
        let words: Vec<String> = ["kiwi", "fig", "plum", "pear", "date"]
            .into_iter()
            .map(String::from)
            .collect();
        let kiwi = words[0].as_ptr();
        let groups = group_by(words, |w| w.len());
        assert_eq!(groups.keys().copied().collect::<Vec<_>>(), [3, 4]);
        assert_eq!(groups[&3], ["fig"]);
        assert_eq!(groups[&4], ["kiwi", "plum", "pear", "date"]);
        assert_eq!(groups[&4][0].as_ptr(), kiwi, "items are moved, not cloned");
        assert!(group_by(Vec::<u8>::new(), |&b| b).is_empty());
        let parity = group_by(vec![3, -2, 5, 0], |n| n % 2 == 0);
        assert_eq!(parity[&false], [3, 5]);
        assert_eq!(parity[&true], [-2, 0]);
    }

    #[test]
    fn c_config_parses_and_reports_the_first_problem() {
        let ok = parse_config(" port = 8080\n\nretries=3\n").unwrap();
        assert_eq!(
            ok,
            BTreeMap::from([("port".to_string(), 8080), ("retries".to_string(), 3)])
        );
        assert_eq!(parse_config(""), Ok(BTreeMap::new()));
        assert_eq!(
            parse_config("a=1\n\na = 2\nb"),
            Err(ConfigError::Duplicate("a".into(), 3))
        );
        for (text, line) in [
            ("x", 1),
            ("a=1\n=2", 2),
            ("a=-1", 1),
            ("a=1=2", 1),
            ("a=", 1),
        ] {
            assert_eq!(
                parse_config(text),
                Err(ConfigError::Malformed(line)),
                "{text:?}"
            );
        }
        assert_eq!(
            parse_config("a=1\nbad\na=2"),
            Err(ConfigError::Malformed(2)),
            "the first problem wins"
        );
    }

    /// P. Earlier-chapter recall: predict each value BEFORE running.
    #[test]
    fn p_predictions() {
        // P1 (Ch15). Iterators are lazy: how many times does the map closure run?
        let mut calls = 0;
        let first = (1..10)
            .map(|x| {
                calls += 1;
                x * x
            })
            .find(|&square| square > 10);
        let p1: (Option<i32>, usize) = guess();
        assert_eq!(p1, (first, calls), "P1: laziness");

        // P2 (Ch15). rev + step_by, and the length of a zip.
        let p2: (Vec<i32>, usize) = guess();
        assert_eq!(
            p2,
            (
                (1..=4).rev().step_by(2).collect(),
                [1, 2, 3].iter().zip("ab".chars()).count()
            ),
            "P2: adapters"
        );

        // P3 (Ch13). Default for a tuple.
        let p3: (String, Vec<u8>, Option<i32>, bool) = guess();
        assert_eq!(p3, Default::default(), "P3: Default");

        // P4 (Ch10). Slice patterns.
        let v = [1, 2, 3, 4];
        let ends = match v {
            [first, .., last] => first + last,
        };
        let rest = match &v[..] {
            [_, _, rest @ ..] => rest.len(),
            _ => 0,
        };
        let p4: (i32, usize) = guess();
        assert_eq!(p4, (ends, rest), "P4: slice patterns");
    }
}
