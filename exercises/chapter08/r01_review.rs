//! REVIEW 01 — Spaced retrieval: Chapters 4, 6 and 7 inside Chapter 8
//! Run: cargo test --test chapter08 r01_
//!
//! Do this WITHOUT reopening earlier chapters or their hints. Struggling to
//! recall is what makes the knowledge stick; look things up only after a real
//! attempt, and note in NOTES.md anything you had to look up.
//!
//! Each task mixes an earlier idea (moving out of collections, match and
//! early exits, error conversion) with this chapter's modules and items.

mod config {
    use std::num::ParseIntError;

    #[derive(Debug, PartialEq)]
    pub enum ConfigError {
        Parse(ParseIntError),
        Zero,
    }

    /// A1. (Ch7 From + Ch8 modules) Wrap a ParseIntError so ? can convert it.
    impl From<ParseIntError> for ConfigError {
        fn from(error: ParseIntError) -> Self {
            todo!("A1: one line")
        }
    }

    /// A2. Parse a trimmed port. Use ? (relying on your From impl), and
    /// reject 0 with ConfigError::Zero.
    pub fn port(text: &str) -> Result<u16, ConfigError> {
        todo!("A2: no map_err needed")
    }
}

/// Unit suffixes and their multipliers.
static UNITS: [(&str, u64); 3] = [("k", 1_000), ("M", 1_000_000), ("G", 1_000_000_000)];

/// B. (Ch6 match/let else + Ch8 statics) Parse ASCII digits followed by an
/// optional unit suffix: "42", "12k", "3M", "7G". Look the suffix up in UNITS
/// (do not repeat the multipliers). None for no digits, an unknown suffix, or
/// u64 overflow.
fn scaled(text: &str) -> Option<u64> {
    todo!("B: split where the digits end, then match the suffix")
}

mod store {
    /// Messages in arrival order. The field stays private to this module.
    #[derive(Debug, Default)]
    pub struct Inbox {
        messages: Vec<String>,
    }

    impl Inbox {
        pub fn push(&mut self, message: String) {
            self.messages.push(message);
        }

        pub fn messages(&self) -> &[String] {
            &self.messages
        }

        /// C. (Ch4 moves + Ch8 privacy) Move every message starting with
        /// `prefix` out, in order, and keep the others in order. No clones.
        pub fn take_matching(&mut self, prefix: &str) -> Vec<String> {
            todo!("C: move Strings; never copy them")
        }
    }
}

pub use store::Inbox;

#[cfg(test)]
mod tests {
    use super::config::{ConfigError, port};
    use super::*;
    use practice::{alloc::count, guess};
    use std::cell::RefCell;
    use std::num::IntErrorKind;

    #[test]
    fn a_port_converts_errors_with_from() {
        assert_eq!(port(" 8080 "), Ok(8080));
        assert_eq!(port("65535"), Ok(65535));
        assert_eq!(port("0"), Err(ConfigError::Zero));
        let Err(ConfigError::Parse(e)) = port("65536") else {
            panic!("65536 should be a parse error");
        };
        assert_eq!(e.kind(), &IntErrorKind::PosOverflow);
        assert!(matches!(port(""), Err(ConfigError::Parse(_))));
        let from: ConfigError = "x".parse::<u8>().unwrap_err().into();
        assert!(matches!(from, ConfigError::Parse(_)));
    }

    #[test]
    fn b_scaled_uses_the_unit_table() {
        assert_eq!(scaled("42"), Some(42));
        assert_eq!(scaled("12k"), Some(12_000));
        assert_eq!(scaled("3M"), Some(3_000_000));
        assert_eq!(scaled("7G"), Some(7_000_000_000));
        assert_eq!(scaled("0k"), Some(0));
        assert_eq!(scaled("18446744073709551615"), Some(u64::MAX));
        assert_eq!(scaled("18446744073709552k"), None, "overflow");
        assert_eq!(scaled("18446744073709551616"), None, "overflow");
        assert_eq!(scaled("5x"), None);
        assert_eq!(scaled("5kk"), None);
        assert_eq!(scaled("k"), None);
        assert_eq!(scaled(""), None);
        assert_eq!(scaled("-3k"), None);
        assert_eq!(scaled("+3"), None, "only ASCII digits");
        assert_eq!(scaled("3K"), None, "suffixes are case-sensitive");
        assert_eq!(scaled("4é"), None);
    }

    #[test]
    fn c_take_matching_moves_without_cloning() {
        let mut inbox = Inbox::default();
        for m in ["err: disk", "ok: boot", "err: net", "ok: done"] {
            inbox.push(m.to_string());
        }
        let first_err = inbox.messages()[0].as_ptr();
        let kept_ok = inbox.messages()[1].as_ptr();
        let (taken, allocations) = count(|| inbox.take_matching("err:"));
        assert_eq!(taken, ["err: disk", "err: net"]);
        assert_eq!(inbox.messages(), ["ok: boot", "ok: done"]);
        assert_eq!(taken[0].as_ptr(), first_err, "move the String out");
        assert_eq!(
            inbox.messages()[0].as_ptr(),
            kept_ok,
            "keep the String in place"
        );
        assert!(allocations <= 2, "no clones; got {allocations} allocations");
        assert!(inbox.take_matching("err:").is_empty());
        assert_eq!(inbox.take_matching(""), ["ok: boot", "ok: done"]);
        assert!(inbox.messages().is_empty());
    }

    struct Noisy<'a>(&'static str, &'a RefCell<Vec<&'static str>>);
    impl Drop for Noisy<'_> {
        fn drop(&mut self) {
            self.1.borrow_mut().push(self.0);
        }
    }

    /// P. Earlier-chapter recall: predict each value BEFORE running.
    #[test]
    fn p_predictions() {
        let parsed: Result<Vec<u8>, _> =
            ["1", "x", "300"].iter().map(|s| s.parse::<u8>()).collect();
        let p1: IntErrorKind = guess();
        assert_eq!(
            p1,
            *parsed.unwrap_err().kind(),
            "P1: Ch7 collect stops at the first Err"
        );

        let mut n = 0;
        'outer: for i in 0..3 {
            for j in 0..3 {
                if j == 2 {
                    continue 'outer;
                }
                if i == 2 {
                    break 'outer;
                }
                n += 1;
            }
        }
        let p2: i32 = guess();
        assert_eq!(p2, n, "P2: Ch6 labeled continue and break");

        let log = RefCell::new(Vec::new());
        {
            let _a = Noisy("a", &log);
            let _v = [Noisy("b", &log), Noisy("c", &log)];
        }
        let p3: Vec<&str> = guess();
        assert_eq!(p3, *log.borrow(), "P3: Ch4 drop order");

        let p4: (i32, i32) = guess();
        assert_eq!(p4, ((-17i32).div_euclid(5), -17 / 5), "P4: Ch3 division");
    }
}
