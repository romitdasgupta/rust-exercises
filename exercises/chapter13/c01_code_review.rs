//! CODE REVIEW 01 — A teammate's pull request
//! Run: cargo test --test chapter13 c01_
//! Then: cargo clippy --test chapter13
//!
//! This is the kind of code you will be asked to review at work. It compiles,
//! and some behavior tests pass. Your job:
//! 1. Before running anything, read the code and write a review in NOTES.md:
//!    list every bug, every utility trait that is the wrong choice, and every
//!    unnecessary allocation.
//! 2. Run the tests. Compare what they found with your review.
//! 3. Fix it. You MAY change parameter types, return types, and which
//!    conversion trait is implemented where the doc comment says so; the
//!    tests are written to accept the better signature.
//! 4. Run clippy and resolve its warnings for this file.

/// A TCP port, 1..=65535.
#[derive(Debug, PartialEq, Eq)]
struct Port(u16);

/// Parse a port from text such as " 8080 ". Bad input (not a number, out of
/// range, or 0) must be reported to the caller, never panic. You may change
/// which conversion trait is implemented.
impl From<&str> for Port {
    fn from(text: &str) -> Self {
        Port(text.trim().parse().unwrap())
    }
}

/// Convert Windows line endings ("\r\n") to Unix ones ("\n"). Other text is
/// unchanged. You may change the return type.
fn unix_newlines(text: &str) -> String {
    text.to_string().replace('\r', "\n")
}

/// Total Unicode scalars across all names. You may change the parameter type.
fn total_chars(names: &Vec<String>) -> usize {
    let mut total = 0;
    for name in names.iter().map(|n| n.clone()) {
        total += name.chars().count();
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;
    use practice::alloc::count;
    use std::hint::black_box;

    #[test]
    // `Port::try_from` compiles with the starting `From` impl and with a `TryFrom` impl.
    #[allow(clippy::unnecessary_fallible_conversions)]
    fn a_ports_parse_and_report_bad_input() {
        assert_eq!(Port::try_from(" 8080 ").ok(), Some(Port(8080)));
        assert_eq!(Port::try_from("65535").ok(), Some(Port(65_535)));
        assert!(Port::try_from("http").is_err());
        assert!(Port::try_from("65536").is_err());
        assert!(Port::try_from("0").is_err());
        assert!(Port::try_from("").is_err());
    }

    #[test]
    fn b_only_crlf_pairs_change() {
        assert_eq!(unix_newlines("a\r\nb\r\n"), "a\nb\n");
        assert_eq!(unix_newlines("plain\ntext"), "plain\ntext");
        assert_eq!(unix_newlines(""), "");
    }

    #[test]
    fn b_unchanged_text_is_not_copied() {
        let text = "already\nunix\n".repeat(10);
        let (out, allocations) = count(|| black_box(unix_newlines(black_box(&text))));
        assert_eq!(out, text);
        assert_eq!(allocations, 0, "borrow the input when nothing changes");
    }

    #[test]
    fn c_total_counts_scalars() {
        let names = vec!["Zoë".to_string(), "李".to_string(), String::new()];
        assert_eq!(total_chars(&names), 4);
        assert_eq!(total_chars(&Vec::<String>::new()), 0);
    }

    #[test]
    fn c_total_reads_names_without_copying_them() {
        let names: Vec<String> = (0..50).map(|i| format!("name{i}")).collect();
        let (total, allocations) = count(|| total_chars(black_box(&names)));
        assert_eq!(total, 10 * 5 + 40 * 6);
        assert_eq!(allocations, 0, "counting characters needs no copies");
    }
}
