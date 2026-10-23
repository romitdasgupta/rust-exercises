//! LESSON 05 — FromStr and textual conversion
//! Run: cargo test --test chapter17 l05_
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider: Does parse::<u16>() accept leading whitespace automatically?
//! Is Display necessarily a round-trippable serialization format?
//!
//! IMPLEMENT the documented tasks, then keep the tests and add an edge case.
//! EXPLAIN afterward: Why should a parser return an error rather than quietly truncate? How
//! do parse and to_string use traits?
//!
//! COMPILER / MANUAL LAB (use examples/ch17_scratch.rs):
//! Test "42", " 42 ", "+42", "-1", and "65536" with u16::from_str. Try parse without any
//! type context and repair its inference. Add a round-trip test for a deliberately
//! specified custom format.
//!
//! TRANSFER: invent a new edge case, predict it, then add a check. A lesson is
//! complete only when its tests, predictions, lab, and explanation are done.

use std::str::FromStr;
#[derive(Debug, PartialEq, Eq)]
struct Dimensions {
    width: u16,
    height: u16,
}
impl FromStr for Dimensions {
    type Err = &'static str;
    /// A. Parse exactly WIDTHxHEIGHT (one lowercase 'x') as unsigned decimals after
    /// trimming the whole input and each component. Parse each component with
    /// u16's FromStr, so "+12" is accepted. Both must be >0; all failures are
    /// "invalid dimensions".
    fn from_str(text: &str) -> Result<Self, Self::Err> {
        todo!("Task 1: implement the documented contract")
    }
}
/// B. Produce the specified machine format WIDTHxHEIGHT, with no spaces.
fn machine_text(dimensions: &Dimensions) -> String {
    todo!("Task 2: implement the documented contract")
}
/// C. Parse radix-16 u32 after trimming; do not accept a 0x prefix automatically.
fn hex_number(text: &str) -> Result<u32, std::num::ParseIntError> {
    todo!("Task 3: implement the documented contract")
}
#[cfg(test)]
mod tests {
    use super::*;
    use practice::guess;
    #[test]
    fn a_parser_validates_shape_range_and_nonzero_invariants() {
        assert_eq!(
            " 12 x 8 ".parse::<Dimensions>(),
            Ok(Dimensions {
                width: 12,
                height: 8
            })
        );
        assert_eq!(
            "+3x4".parse::<Dimensions>(),
            Ok(Dimensions {
                width: 3,
                height: 4
            })
        );
        for text in [
            "", "1", "1x2x3", "0x4", "2x0", "65536x1", "-1x2", "1xno", "3X4", "x", "1 2x3",
        ] {
            assert_eq!(
                text.parse::<Dimensions>(),
                Err("invalid dimensions"),
                "{text}"
            );
        }
    }
    #[test]
    fn b_machine_format_round_trips_under_its_contract() {
        let d = Dimensions {
            width: 65535,
            height: 1,
        };
        let text = machine_text(&d);
        assert_eq!(text, "65535x1");
        assert_eq!(text.parse::<Dimensions>(), Ok(d));
    }
    #[test]
    fn c_radix_parser_is_distinct_from_decimal_parse() {
        assert_eq!(hex_number(" ff ").unwrap(), 255);
        assert!(hex_number("0xff").is_err());
        assert!(hex_number("100000000").is_err());
        assert_eq!(hex_number("FfFf").unwrap(), 0xffff);
        assert!(hex_number("").is_err());
        assert!(hex_number("g").is_err());
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    #[test]
    fn p_predictions() {
        // P1. Which of these does u16 accept?
        let p1: (bool, bool, bool) = guess();
        assert_eq!(
            p1,
            (
                " 42".parse::<u16>().is_ok(),
                "+42".parse::<u16>().is_ok(),
                "-0".parse::<u16>().is_ok()
            ),
            "P1: what parse accepts"
        );

        // P2. The error kinds for "256" and "" parsed as u8.
        use std::num::IntErrorKind;
        let p2: (IntErrorKind, IntErrorKind) = guess();
        assert_eq!(
            p2,
            (
                *"256".parse::<u8>().unwrap_err().kind(),
                *"".parse::<u8>().unwrap_err().kind()
            ),
            "P2: ParseIntError kinds"
        );

        // P3. Is "3.0" a valid i32? What f64 is "1e3"?
        let p3: (bool, f64) = guess();
        assert_eq!(
            p3,
            ("3.0".parse::<i32>().is_ok(), "1e3".parse::<f64>().unwrap()),
            "P3: integer vs float syntax"
        );

        // P4. Display for two floats.
        let p4: (String, String) = guess();
        assert_eq!(
            p4,
            (1.0f64.to_string(), 1.5f64.to_string()),
            "P4: Display for f64"
        );
    }
}
