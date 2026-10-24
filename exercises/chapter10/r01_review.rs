//! REVIEW 01 — Spaced retrieval: Chapters 3, 7 and 9 inside Chapter 10
//! Run: cargo test --test chapter10 r01_
//!
//! Do this WITHOUT reopening earlier chapters or their hints. Struggling to
//! recall is what makes the knowledge stick; look things up only after a real
//! attempt, and note in NOTES.md anything you had to look up.
//!
//! Each task mixes an earlier idea (interior mutability, error propagation,
//! Unicode scalars) with this chapter's enums and patterns.

use std::cell::Cell;
use std::num::ParseIntError;

#[derive(Clone, Copy, Debug, PartialEq)]
enum DoorState {
    Open,
    Closed,
    Locked,
}

struct Door {
    state: Cell<DoorState>,
}

impl Door {
    /// A. (Ch9 Cell + Ch10 enums) Through `&self`: Open becomes Closed and
    /// Closed becomes Open, returning true. A Locked door stays Locked and
    /// returns false. Match every variant; no wildcard arm.
    fn toggle(&self) -> bool {
        todo!("A: get the Copy state, match it, set the new one")
    }
}

#[derive(Debug, PartialEq)]
enum Command {
    Move { dx: i32, dy: i32 },
    Say(String),
    Quit,
}

#[derive(Debug, PartialEq)]
enum ParseError {
    Empty,
    Unknown(String),
    BadNumber(ParseIntError),
}

impl From<ParseIntError> for ParseError {
    /// B1. (Ch7 conversions) Wrap the error so `?` can convert it.
    fn from(err: ParseIntError) -> Self {
        todo!("B1: wrap the ParseIntError")
    }
}

/// B2. (Ch7 `?` + Ch10 slice patterns) Split on whitespace and match the words:
/// `move DX DY` (two i32s), `say WORD...` (at least one word, rejoined with
/// single spaces), `quit` (nothing after it). No words: Empty. Anything else:
/// Unknown(first word). Parse numbers with `?`.
fn parse_command(line: &str) -> Result<Command, ParseError> {
    todo!("B2: collect the words, then match words.as_slice()")
}

#[derive(Debug, PartialEq, Default)]
struct Tally {
    letters: usize,
    digits: usize,
    spaces: usize,
    other: usize,
}

/// C. (Ch3 Unicode scalars + Ch10 range patterns) Count each char of `text`:
/// ASCII letters, ASCII digits, whitespace (any Unicode whitespace), other.
/// Use range patterns and one guard; walk chars, not bytes.
fn tally(text: &str) -> Tally {
    todo!("C: match each char against ranges")
}

#[cfg(test)]
mod tests {
    use super::*;
    use practice::{alloc::count, guess};

    #[test]
    fn a_toggle_respects_the_lock() {
        let door = Door {
            state: Cell::new(DoorState::Closed),
        };
        let shared = &door;
        assert!(shared.toggle());
        assert_eq!(door.state.get(), DoorState::Open);
        assert!(shared.toggle());
        assert_eq!(door.state.get(), DoorState::Closed);
        door.state.set(DoorState::Locked);
        assert!(!shared.toggle());
        assert_eq!(door.state.get(), DoorState::Locked);
    }

    #[test]
    fn b_commands_parse_by_shape() {
        assert_eq!(
            parse_command("move 3 -4"),
            Ok(Command::Move { dx: 3, dy: -4 })
        );
        assert_eq!(
            parse_command("  say  hello\tthere "),
            Ok(Command::Say("hello there".into()))
        );
        assert_eq!(parse_command("quit"), Ok(Command::Quit));
        assert_eq!(parse_command(" \n"), Err(ParseError::Empty));
        assert_eq!(parse_command(""), Err(ParseError::Empty));
    }

    #[test]
    fn b_wrong_shapes_and_numbers_are_errors() {
        let bad_number = "x".parse::<i32>().unwrap_err();
        assert_eq!(
            parse_command("move 1 x"),
            Err(ParseError::BadNumber(bad_number))
        );
        let too_big = "99999999999".parse::<i32>().unwrap_err();
        assert_eq!(
            parse_command("move 99999999999 0"),
            Err(ParseError::BadNumber(too_big.clone()))
        );
        for line in ["move 1", "move 1 2 3", "say", "quit now", "jump 1"] {
            let first = line.split_whitespace().next().unwrap();
            assert_eq!(
                parse_command(line),
                Err(ParseError::Unknown(first.into())),
                "{line:?}"
            );
        }
        assert_eq!(
            ParseError::from(too_big.clone()),
            ParseError::BadNumber(too_big)
        );
    }

    #[test]
    fn c_tally_walks_unicode_scalars() {
        assert_eq!(
            tally("Ab 12\té🦀\u{3000}"),
            Tally {
                letters: 2,
                digits: 2,
                spaces: 3,
                other: 2
            }
        );
        assert_eq!(tally(""), Tally::default());
        assert_eq!(
            tally("٣"),
            Tally {
                other: 1,
                ..Tally::default()
            },
            "U+0663 is a digit, but not an ASCII digit"
        );
    }

    #[test]
    fn c_tally_does_not_allocate() {
        let text = "mixed text 42 ✓".repeat(10);
        let (t, allocations) = count(|| tally(&text));
        assert_eq!(t.digits, 20);
        assert_eq!(allocations, 0, "walk the chars; do not collect them");
    }

    #[derive(Debug, Default)]
    struct Pair(u8, bool, String);

    /// P. Earlier-chapter recall: predict each value BEFORE running.
    #[test]
    fn p_predictions() {
        // P1 (Ch9). Write the exact derived Debug output of Pair::default().
        let p1: &str = guess();
        assert_eq!(p1, format!("{:?}", Pair::default()), "P1: derived Default");

        // P2 (Ch8). The package name Cargo compiled this test for.
        let p2: &str = guess();
        assert_eq!(
            p2,
            env!("CARGO_PKG_NAME"),
            "P2: package versus library name"
        );

        // P3 (Ch7). Which of these parse as u8?
        let p3: (bool, bool) = guess();
        assert_eq!(
            p3,
            ("-0".parse::<u8>().is_ok(), "+0".parse::<u8>().is_ok()),
            "P3: signs and unsigned parsing"
        );

        // P4 (Ch6). The value a loop breaks with.
        let mut n = 1;
        let reached = loop {
            if n > 20 {
                break n;
            }
            n *= 3;
        };
        let p4: i32 = guess();
        assert_eq!(p4, reached, "P4: loop as an expression");
    }
}
