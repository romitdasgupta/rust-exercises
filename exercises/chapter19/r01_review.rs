//! REVIEW 01 — Spaced retrieval: Chapters 14–18 inside Chapter 19
//! Run: cargo test --test chapter19 r01_
//!
//! Do this WITHOUT reopening Chapters 14–18 or their hints. Struggling to recall
//! is what makes the knowledge stick; look things up only after a real attempt,
//! and note in NOTES.md anything you had to look up.
//!
//! Each task mixes an earlier idea (readers, maps, boxed closures) with this
//! chapter's threads.

use std::{
    collections::BTreeMap,
    io::{self, BufRead, Cursor},
    sync::Mutex,
    thread,
};

/// A. (Ch18 buffered readers + Ch19 spawn/join) Move each byte buffer into its
/// own worker, which counts its lines with `BufRead::lines` over a `Cursor`.
/// Invalid UTF-8 is that source's error. Results come back in input order.
fn line_counts(sources: Vec<Vec<u8>>) -> Vec<io::Result<usize>> {
    todo!("A: one worker per source; join in spawn order")
}

/// B. (Ch16 maps + Ch19 scoped threads and Mutex) Count whitespace-separated
/// words across all chunks, case-sensitively. Use one scoped worker per chunk.
/// Each worker counts into its own map first, then locks the shared total once
/// to merge. Return the totals.
fn word_counts(chunks: &[&str]) -> BTreeMap<String, usize> {
    todo!("B: local map per worker, one lock per worker to merge")
}

/// C. (Ch14 boxed closures + Ch19 Send) Run every job on its own new thread
/// (never on the caller's thread) and return the results in input order.
fn run_all<T: Send + 'static>(jobs: Vec<Box<dyn FnOnce() -> T + Send>>) -> Vec<T> {
    todo!("C: each FnOnce moves into the thread that calls it")
}

#[cfg(test)]
mod tests {
    use super::*;
    use practice::guess;
    use std::collections::HashSet;

    #[test]
    fn a_counts_lines_per_source_in_order() {
        let results = line_counts(vec![
            b"one\ntwo\n".to_vec(),
            Vec::new(),
            b"no newline".to_vec(),
            vec![b'o', b'k', b'\n', 0xff, b'\n'],
            "crab 🦀\r\nline\n".as_bytes().to_vec(),
        ]);
        let summary: Vec<Result<usize, io::ErrorKind>> = results
            .into_iter()
            .map(|r| r.map_err(|e| e.kind()))
            .collect();
        assert_eq!(
            summary,
            [Ok(2), Ok(0), Ok(1), Err(io::ErrorKind::InvalidData), Ok(2)]
        );
        assert!(line_counts(vec![]).is_empty());
    }

    #[test]
    fn b_word_counts_merge_every_chunk() {
        let counts = word_counts(&["the cat", "the  hat\tsat", "", "Cat été été"]);
        let expected: BTreeMap<String, usize> = [
            ("Cat", 1),
            ("cat", 1),
            ("hat", 1),
            ("sat", 1),
            ("the", 2),
            ("été", 2),
        ]
        .into_iter()
        .map(|(w, n)| (w.to_string(), n))
        .collect();
        assert_eq!(counts, expected);
        assert!(word_counts(&[]).is_empty());
    }

    #[test]
    fn b_many_chunks_lose_no_updates() {
        let chunks = vec!["a b a"; 64];
        let counts = word_counts(&chunks);
        assert_eq!(counts["a"], 128);
        assert_eq!(counts["b"], 64);
    }

    #[test]
    fn c_results_keep_input_order() {
        let jobs: Vec<Box<dyn FnOnce() -> String + Send>> = vec![
            Box::new(|| "first".to_string()),
            Box::new(|| "second".repeat(2)),
            Box::new(String::new),
        ];
        assert_eq!(run_all(jobs), ["first", "secondsecond", ""]);
        assert!(run_all::<u8>(Vec::new()).is_empty());
    }

    #[test]
    fn c_every_job_gets_its_own_thread() {
        let owned = String::from("moved into the job");
        let mut jobs: Vec<Box<dyn FnOnce() -> (thread::ThreadId, usize) + Send>> =
            vec![Box::new(move || (thread::current().id(), owned.len()))];
        for n in 0..5 {
            jobs.push(Box::new(move || (thread::current().id(), n)));
        }
        let results = run_all(jobs);
        let ids: HashSet<_> = results.iter().map(|(id, _)| *id).collect();
        assert_eq!(ids.len(), 6, "each job must run on a different thread");
        assert!(
            !ids.contains(&thread::current().id()),
            "no job may run on the caller's thread"
        );
        assert_eq!(results[0].1, 18);
    }

    /// P. Earlier-chapter recall: predict each value BEFORE running.
    #[test]
    fn p_predictions() {
        // P1 (Ch18). What does read_line return, and what does it store?
        let mut line = String::new();
        let n = Cursor::new("ab\ncd").read_line(&mut line).unwrap();
        let p1: (usize, String) = guess();
        assert_eq!(p1, (n, line), "P1: read_line keeps the newline");

        // P2 (Ch17). Byte length of "Straße", and its uppercase form.
        let p2: (usize, String) = guess();
        assert_eq!(
            p2,
            ("Straße".len(), "Straße".to_uppercase()),
            "P2: UTF-8 length and case mapping"
        );

        // P3 (Ch16). Entry counting into a BTreeMap, then iterating it.
        let mut m = BTreeMap::new();
        for w in "b a b c a b".split(' ') {
            *m.entry(w).or_insert(0) += 1;
        }
        let p3: Vec<(&str, i32)> = guess();
        assert_eq!(
            p3,
            m.into_iter().collect::<Vec<_>>(),
            "P3: BTreeMap iteration order"
        );

        // P4 (Ch15). A reversed, stepped range.
        let p4: Vec<i32> = guess();
        assert_eq!(
            p4,
            (1..=10).rev().step_by(3).collect::<Vec<_>>(),
            "P4: rev then step_by"
        );
    }
}
