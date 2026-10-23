//! CODE REVIEW 01 — A teammate's pull request
//! Run: cargo test --test chapter17 c01_
//! Then: cargo clippy --test chapter17
//!
//! This is the kind of code you will be asked to review at work. It compiles,
//! and some behavior tests pass. Your job:
//! 1. Before running anything, read the code and write a review in NOTES.md:
//!    list every bug, every byte-versus-scalar mistake, and every allocation
//!    that borrowing or a better std API would avoid.
//! 2. Run the tests. Compare what they found with your review.
//! 3. Fix it. You MAY change parameter and return types where the doc comment
//!    says so; the tests are written to accept the better signature.
//! 4. Run clippy and resolve its warnings for this file.

/// Turn a title into a URL slug: lowercase it, keep alphanumeric Unicode
/// scalars, replace every run of other characters with one '-', and trim
/// '-' from both ends. You may change the parameter type.
fn slugify(title: &String) -> String {
    let lower = title.to_lowercase();
    let mut out = String::new();
    let mut last_dash = false;
    for c in lower.chars() {
        if c.is_ascii_alphanumeric() {
            out = out + &c.to_string();
            last_dash = false;
        } else if !last_dash {
            out = out + "-";
            last_dash = true;
        }
    }
    out.trim_matches('-').to_string()
}

/// Shorten text for display. If it has more than `max` Unicode scalars, keep
/// the first `max - 1` and append '…', so the result has exactly `max`
/// scalars. Text that fits comes back unchanged. `max` is at least 1.
/// You may change the return type.
fn truncate_display(text: &str, max: usize) -> String {
    if text.len() > max {
        text[..max - 1].to_string() + "…"
    } else {
        text.to_string()
    }
}

/// Parse "k=v;k=v" settings. Segments are separated by ';'. Skip empty
/// segments and segments without '='; split each pair on its first '='.
/// You may change the return type to borrow from `text`.
fn parse_pairs(text: &str) -> Vec<(String, String)> {
    let mut out = Vec::new();
    for segment in text.split(";") {
        if segment.is_empty() || !segment.contains("=") {
            continue;
        }
        let parts: Vec<&str> = segment.splitn(2, "=").collect();
        out.push((parts[0].to_string(), parts[1].to_string()));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use practice::alloc::count;

    /// Calls `slugify` with a `&String`, which also coerces to `&str`.
    fn slug(title: &str) -> String {
        let owned = String::from(title);
        slugify(&owned)
    }

    fn pairs<A: AsRef<str>, B: AsRef<str>>(v: &[(A, B)]) -> Vec<(&str, &str)> {
        v.iter().map(|(a, b)| (a.as_ref(), b.as_ref())).collect()
    }

    #[test]
    fn a_slugify_collapses_separators() {
        assert_eq!(slug("  Hello, World!  "), "hello-world");
        assert_eq!(slug("Rust 2024"), "rust-2024");
        assert_eq!(slug("--"), "");
    }

    #[test]
    fn a_slugify_keeps_unicode_letters() {
        assert_eq!(slug("Café Olé"), "café-olé");
        assert_eq!(slug("ΣΟΦΙΑ & Ünïcode"), "σοφια-ünïcode");
    }

    #[test]
    fn a_slugify_does_not_allocate_per_char() {
        let title = "word ".repeat(40);
        let (out, allocations) = count(|| slugify(&title));
        assert!(out.starts_with("word-word"));
        assert!(
            allocations <= 10,
            "a 200-byte title cost {allocations} allocations"
        );
    }

    #[test]
    fn b_truncate_counts_scalars() {
        assert_eq!(truncate_display("hello", 5), "hello");
        assert_eq!(truncate_display("hello world", 6), "hello…");
        assert_eq!(truncate_display("héllo wörld", 5), "héll…");
        assert_eq!(truncate_display("é", 1), "é");
        assert_eq!(truncate_display("🦀🦀🦀", 2), "🦀…");
        assert_eq!(truncate_display("ab", 1), "…");
    }

    #[test]
    fn b_truncate_borrows_text_that_fits() {
        let text = "short enough";
        let (len, allocations) = count(|| truncate_display(text, 40).len());
        assert_eq!(len, text.len());
        assert_eq!(allocations, 0, "text that fits can be returned borrowed");
    }

    #[test]
    fn c_parse_pairs_splits_on_the_first_equals() {
        let out = parse_pairs("a=1;;b=x=y;junk;c=");
        assert_eq!(pairs(&out), [("a", "1"), ("b", "x=y"), ("c", "")]);
        assert!(parse_pairs("").is_empty());
    }

    #[test]
    fn c_parse_pairs_borrows_from_the_input() {
        let text: String = (0..50).map(|i| format!("key{i}=value{i};")).collect();
        let (n, allocations) = count(|| parse_pairs(&text).len());
        assert_eq!(n, 50);
        assert!(allocations <= 8, "50 pairs cost {allocations} allocations");
    }
}
