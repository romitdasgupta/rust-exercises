//! CODE REVIEW 01 — A teammate's pull request
//! Run: cargo test --test chapter07 c01_
//! Then: cargo clippy --test chapter07
//!
//! This is the kind of code you will be asked to review at work. It compiles,
//! and most behavior tests pass. Your job:
//! 1. Before running anything, read the code and write a review in NOTES.md:
//!    list every bug, every swallowed or stringified error, and every
//!    unnecessary allocation.
//! 2. Run the tests. Compare what they found with your review.
//! 3. Fix it. You MAY change types where the doc comment says so; the tests
//!    are written to accept the better signature.
//! 4. Run clippy and resolve its warnings for this file.

use std::num::ParseIntError;

/// Find `key` in "KEY=VALUE" lines (blank lines are skipped) and parse its
/// value as u32. Ok(None) if the key is absent. A line without exactly one
/// '=' or an invalid number is an error. You may change the error type.
fn setting(text: &str, key: &str) -> Result<Option<u32>, String> {
    for line in text.lines() {
        if line.trim().is_empty() {
            continue;
        }
        let parts: Vec<&str> = line.split('=').collect();
        if parts.len() != 2 {
            return Err(format!("malformed line: {}", line));
        }
        if parts[0].trim() == key {
            return Ok(Some(parts[1].trim().parse().unwrap()));
        }
    }
    Ok(None)
}

/// A missing value means the default limit 10; a present value must be a
/// valid u32, and an invalid one is an error. Keep this signature.
fn limit_or_default(value: Option<&str>) -> Result<u32, ParseIntError> {
    match value {
        Some(v) => match v.trim().parse::<u32>() {
            Ok(n) => Ok(n),
            Err(_) => Ok(10),
        },
        None => Ok(10),
    }
}

/// Sum the valid rows and count the invalid ones. The total fits in i64.
/// Keep this signature.
fn tally(rows: &[&str]) -> (i64, usize) {
    let mut sum = 0;
    let mut errors = Vec::new();
    for row in rows {
        let parsed = row.trim().parse::<i64>();
        if parsed.is_ok() {
            sum += parsed.unwrap();
        } else {
            errors.push(parsed.unwrap_err().to_string());
        }
    }
    (sum, errors.len())
}

#[cfg(test)]
mod tests {
    use super::*;
    use practice::alloc::count;
    use std::panic::catch_unwind;

    #[test]
    fn a_setting_finds_values_and_absent_keys() {
        let text = "host = example\n\nport = 8080\n";
        assert_eq!(setting(text, "port").ok(), Some(Some(8080)));
        assert_eq!(setting(text, "user").ok(), Some(None));
        assert!(setting("broken line", "port").is_err());
        assert!(setting("a=b=c", "a").is_err());
    }

    #[test]
    fn a_setting_reports_invalid_numbers_instead_of_panicking() {
        let outcome = catch_unwind(|| setting("port = eighty", "port").is_err());
        assert_eq!(
            outcome.ok(),
            Some(true),
            "an invalid number is an Err, not a panic"
        );
    }

    #[test]
    fn a_setting_does_not_allocate_on_success() {
        let mut text = String::new();
        for i in 0..50 {
            text.push_str(&format!("key{i} = {i}\n"));
        }
        let (value, allocations) = count(|| setting(&text, "key49").ok());
        assert_eq!(value, Some(Some(49)));
        assert_eq!(allocations, 0, "split_once borrows; no Vec per line");
    }

    #[test]
    fn b_limit_uses_the_default_only_when_absent() {
        assert_eq!(limit_or_default(None), Ok(10));
        assert_eq!(limit_or_default(Some(" 25 ")), Ok(25));
        assert_eq!(limit_or_default(Some("0")), Ok(0));
        assert!(
            limit_or_default(Some("ten")).is_err(),
            "a present but invalid value must not become the default"
        );
    }

    #[test]
    fn c_tally_counts_and_sums() {
        assert_eq!(tally(&["1", "x", " -3 ", ""]), (-2, 2));
        assert_eq!(tally(&[]), (0, 0));
    }

    #[test]
    fn c_tally_does_not_render_errors_it_only_counts() {
        let rows = ["bad"; 40];
        let (result, allocations) = count(|| tally(&rows));
        assert_eq!(result, (0, 40));
        assert_eq!(
            allocations, 0,
            "{allocations} allocations to count 40 errors"
        );
    }
}
