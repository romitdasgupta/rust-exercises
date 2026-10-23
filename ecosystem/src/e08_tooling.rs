//! LESSON E08 — Tooling workday: docs, lints, macros, profiles, timing, flamegraphs
//! Run: cargo test --manifest-path ecosystem/Cargo.toml e08_
//! Guide: ecosystem/TOOLING.md. Each task there has a command and a "done when" check.
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider: Does `cargo test --release` still check
//! integer overflow? Why can a benchmark of `x * 2` report 0 ns?
//! IMPLEMENT the tasks below, and work through TOOLING.md alongside them. Preserve the
//! tests and add an edge case.
//! EXPLAIN afterward: Explain what a flamegraph's width and stacking mean. Name two
//! reasons why one `Instant` measurement of a function is not a benchmark.
//! JUDGMENT: Start with the tools that ship with cargo (doc, tree, clippy, profiles).
//! Then add criterion/divan for benchmarks, cargo-flamegraph or samply for profiles,
//! and cargo-expand for macros, when a question needs them.

use std::time::Duration;

/// A. Parse a byte size such as `512`, `4KiB`, `10MiB`, or `2GiB` (binary units,
/// case-sensitive, no spaces). The result must fit in a u64.
///
/// TOOLING.md task 1: write an `# Examples` section here with a doc test, then run
/// `cargo test --doc`. Also fix the broken intra-doc link below. The check is
/// `RUSTDOCFLAGS="-D warnings" cargo doc --no-deps`.
/// Errors are described by [`SizeEror`].
pub fn parse_size(text: &str) -> Result<u64, SizeError> {
    todo!("Implement this task")
}

/// Why [`parse_size`] rejected its input.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SizeError {
    /// No leading digits.
    BadNumber,
    /// The suffix is not one of `""`, `KiB`, `MiB`, `GiB`.
    UnknownUnit(String),
    /// The value does not fit in a u64.
    Overflow,
}

/// B. Macro debugging without cargo-expand. `double!(e)` must evaluate to `2 * (e)`
/// for any expression, but `1 + 2` gives 5. Look at what it expands to with the
/// technique in TOOLING.md task 4, then fix the matcher.
#[macro_export]
macro_rules! double {
    ($($e:tt)*) => {
        $($e)* * 2
    };
}

/// C. Summary statistics for timing samples. Sort a copy of the samples. The median is
/// the middle element (for an even count, the lower of the two middle ones). Returns
/// `None` for no samples.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Stats {
    pub min: Duration,
    pub median: Duration,
    pub max: Duration,
}

pub fn summarize(samples: &[Duration]) -> Option<Stats> {
    todo!("Implement this task")
}

/// C. A minimal timing harness. Call `f` `warmup` times unmeasured, then `samples`
/// times, each timed with its own `Instant`, and return the sample durations. Pass every
/// result through `std::hint::black_box`, so the optimizer cannot delete the work.
pub fn measure<R>(warmup: usize, samples: usize, f: impl FnMut() -> R) -> Vec<Duration> {
    todo!("Implement this task")
}

// D. Reading a profile. `folded` holds "folded stacks", the text format that
// flamegraph tools draw: each line is `root;child;leaf COUNT`, where COUNT samples were
// taken with exactly that call stack. Blank lines are ignored.

/// D. Inclusive samples of `frame`: samples whose stack contains `frame` anywhere.
/// This is the width of `frame`'s boxes in a flamegraph. Count a recursive stack once.
pub fn inclusive_samples(folded: &str, frame: &str) -> u64 {
    todo!("Implement this task")
}

/// D. Self samples of `frame`: samples where `frame` is the leaf (top of the stack).
/// This is the part of its box with nothing stacked on top.
pub fn self_samples(folded: &str, frame: &str) -> u64 {
    todo!("Implement this task")
}

/// D. The frame with the most self samples, with its count; ties go to the
/// alphabetically first frame. `None` for an empty profile.
pub fn hottest_self(folded: &str) -> Option<(String, u64)> {
    todo!("Implement this task")
}

/// A profile of a request handler, in folded-stack format (see TOOLING.md task 7).
pub const PROFILE: &str = "\
main;serve;handle;parse_json;alloc 30
main;serve;handle;parse_json 120
main;serve;handle;validate 15
main;serve;handle;render;format 90
main;serve;handle;render;alloc 45
main;serve;handle;render 10
main;serve;log;format 40
main;serve;log;write 25
main;idle 5
";

#[cfg(test)]
mod tests {
    use super::*;
    use crate::guess;

    #[test]
    fn a_parse_size_units_and_errors() {
        assert_eq!(parse_size("512"), Ok(512));
        assert_eq!(parse_size("4KiB"), Ok(4096));
        assert_eq!(parse_size("10MiB"), Ok(10 << 20));
        assert_eq!(parse_size("2GiB"), Ok(2 << 30));
        assert_eq!(parse_size("0GiB"), Ok(0));
        assert_eq!(parse_size(""), Err(SizeError::BadNumber));
        assert_eq!(parse_size("KiB"), Err(SizeError::BadNumber));
        assert_eq!(parse_size("4kb"), Err(SizeError::UnknownUnit("kb".into())));
        assert_eq!(
            parse_size("4 KiB"),
            Err(SizeError::UnknownUnit(" KiB".into()))
        );
        assert_eq!(parse_size("18446744073709551615"), Ok(u64::MAX));
        assert_eq!(parse_size("18446744073709551616"), Err(SizeError::Overflow));
        assert_eq!(parse_size("17179869184GiB"), Err(SizeError::Overflow));
    }

    #[test]
    fn b_double_respects_precedence() {
        assert_eq!(double!(1 + 2), 6);
        assert_eq!(double!(10 - 4), 12);
        assert_eq!(double!(7), 14);
        let x = 3;
        assert_eq!(double!(if x > 2 { x } else { 0 }), 6);
    }

    #[test]
    fn c_summarize_sorts_and_picks_the_lower_median() {
        let ms = Duration::from_millis;
        assert_eq!(summarize(&[]), None);
        let stats = summarize(&[ms(9), ms(1), ms(5)]).unwrap();
        assert_eq!(
            stats,
            Stats {
                min: ms(1),
                median: ms(5),
                max: ms(9)
            }
        );
        let stats = summarize(&[ms(4), ms(1), ms(3), ms(2)]).unwrap();
        assert_eq!(stats.median, ms(2));
        assert_eq!(summarize(&[ms(7)]).unwrap().median, ms(7));
    }

    #[test]
    fn c_measure_warms_up_and_times_each_sample() {
        let mut calls = 0;
        let samples = measure(3, 5, || {
            calls += 1;
            std::thread::sleep(Duration::from_millis(1));
        });
        assert_eq!(calls, 8);
        assert_eq!(samples.len(), 5);
        assert!(samples.iter().all(|&d| d >= Duration::from_millis(1)));
        assert!(measure(0, 0, || 1).is_empty());
    }

    #[test]
    fn d_reads_the_profile() {
        assert_eq!(inclusive_samples(PROFILE, "main"), 380);
        assert_eq!(inclusive_samples(PROFILE, "handle"), 310);
        assert_eq!(inclusive_samples(PROFILE, "render"), 145);
        assert_eq!(inclusive_samples(PROFILE, "format"), 130);
        assert_eq!(inclusive_samples(PROFILE, "missing"), 0);
        assert_eq!(self_samples(PROFILE, "parse_json"), 120);
        assert_eq!(self_samples(PROFILE, "alloc"), 75);
        assert_eq!(self_samples(PROFILE, "handle"), 0);
        assert_eq!(hottest_self(PROFILE), Some(("format".to_owned(), 130)));
        assert_eq!(hottest_self(""), None);
    }

    #[test]
    fn d_recursion_and_ties() {
        let folded = "main;walk;walk;walk 7\nmain;walk 3\n\nmain;a 5\nmain;b 5\n";
        assert_eq!(inclusive_samples(folded, "walk"), 10, "count a stack once");
        assert_eq!(self_samples(folded, "walk"), 10);
        assert_eq!(hottest_self(folded), Some(("walk".to_owned(), 10)));
        assert_eq!(hottest_self("x;a 5\nx;b 5"), Some(("a".to_owned(), 5)));
    }

    macro_rules! double_tt {
        ($($e:tt)*) => {
            $($e)* * 2
        };
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    #[test]
    fn p_predictions() {
        // P1. This is the original, buggy `double!`. What is `double_tt!(1 + 2)`?
        let p1: i32 = guess();
        assert_eq!(
            p1,
            double_tt!(1 + 2),
            "P1: tt repetitions paste tokens, not expressions"
        );

        // P2. With this package's profiles, is overflow checked exactly when
        // debug assertions are on? (Answer for both `cargo test` and `--release`.)
        let overflow_checked =
            std::panic::catch_unwind(|| std::hint::black_box(u8::MAX) + 1).is_err();
        let p2: bool = guess();
        assert_eq!(
            p2,
            overflow_checked == cfg!(debug_assertions),
            "P2: release disables both unless the profile says otherwise"
        );

        // P3. In PROFILE, how many samples does `serve` have in total (its box width)?
        let width: u64 = PROFILE
            .lines()
            .filter(|l| {
                l.split(' ')
                    .next()
                    .unwrap()
                    .split(';')
                    .any(|f| f == "serve")
            })
            .map(|l| l.rsplit(' ').next().unwrap().parse::<u64>().unwrap())
            .sum();
        let p3: u64 = guess();
        assert_eq!(
            p3, width,
            "P3: a frame's width includes everything above it"
        );
    }
}
