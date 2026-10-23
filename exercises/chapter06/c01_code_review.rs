//! CODE REVIEW 01 — A teammate's pull request
//! Run: cargo test --test chapter06 c01_
//! Then: cargo clippy --test chapter06
//!
//! This is the kind of code you will be asked to review at work. It compiles,
//! and most behavior tests pass. Your job:
//! 1. Before running anything, read the code and write a review in NOTES.md:
//!    list every bug, every needless allocation, and every statement-style
//!    loop or flag that an expression would say more clearly.
//! 2. Run the tests. Compare what they found with your review.
//! 3. Fix it. You MAY change parameter types where the doc comment says so;
//!    the tests are written to accept the better signature.
//! 4. Run clippy and resolve its warnings for this file.

/// Return the 1-based line number of the first line whose trimmed text is
/// "end", or None. Lines after it are never inspected. You may change the
/// parameter type.
fn end_line(lines: &Vec<String>) -> Option<usize> {
    let mut result = None;
    let mut i = 0;
    while i < lines.len() {
        if lines[i].trim().to_string() == "end" && result == None {
            result = Some(i + 1);
        }
        i = i + 1;
    }
    return result;
}

/// Classify a status code: 100..=199 "info", 200..=299 "ok",
/// 300..=399 "redirect", 400..=599 "error", anything else "invalid".
/// Keep this signature.
fn classify(code: u16) -> &'static str {
    let mut kind = "invalid";
    if code >= 100 && code < 200 {
        kind = "info";
    }
    if code >= 200 && code < 300 {
        kind = "ok";
    }
    if code >= 300 && code < 400 {
        kind = "redirect";
    }
    if code >= 400 && code < 599 {
        kind = "error";
    }
    kind
}

/// Sum byte counts written as decimal text (surrounding whitespace allowed).
/// None if any entry is not a valid number or the total does not fit in u32.
/// Keep this signature.
fn total_size(sizes: &[&str]) -> Option<u32> {
    let mut total: u64 = 0;
    for s in sizes {
        match s.trim().parse::<u64>() {
            Ok(n) => total = total + n,
            Err(_) => return None,
        }
    }
    Some(total as u32)
}

#[cfg(test)]
mod tests {
    use super::*;
    use practice::alloc::count;

    fn lines(items: &[&str]) -> Vec<String> {
        items.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn a_end_line_is_one_based_and_first() {
        assert_eq!(end_line(&lines(&["a", " end ", "end"])), Some(2));
        assert_eq!(end_line(&lines(&["end"])), Some(1));
        assert_eq!(end_line(&lines(&["ending", "END"])), None);
        assert_eq!(end_line(&Vec::new()), None);
    }

    #[test]
    fn a_end_line_borrows_instead_of_copying() {
        let mut many = lines(&["x"; 200]);
        many[3] = "end".into();
        let (found, allocations) = count(|| end_line(&many));
        assert_eq!(found, Some(4));
        assert_eq!(allocations, 0, "comparing &str needs no new String");
    }

    #[test]
    fn b_classify_covers_every_range_edge() {
        for (code, kind) in [
            (99, "invalid"),
            (100, "info"),
            (199, "info"),
            (200, "ok"),
            (299, "ok"),
            (300, "redirect"),
            (399, "redirect"),
            (400, "error"),
            (599, "error"),
            (600, "invalid"),
            (0, "invalid"),
            (u16::MAX, "invalid"),
        ] {
            assert_eq!(classify(code), kind, "code {code}");
        }
    }

    #[test]
    fn c_total_size_adds_valid_entries() {
        assert_eq!(total_size(&[" 10", "20 ", "0"]), Some(30));
        assert_eq!(total_size(&[]), Some(0));
        assert_eq!(total_size(&["10", "ten"]), None);
        assert_eq!(total_size(&["-1"]), None);
    }

    #[test]
    fn c_total_size_rejects_overflow_instead_of_truncating() {
        assert_eq!(total_size(&["4294967295"]), Some(u32::MAX));
        assert_eq!(total_size(&["4294967295", "1"]), None);
        assert_eq!(total_size(&["4294967296"]), None);
        assert_eq!(total_size(&["18446744073709551615", "1"]), None);
    }
}
