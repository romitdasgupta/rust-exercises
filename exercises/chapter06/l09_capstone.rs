//! LESSON 09 — Capstone: command expression engine
//! Run: cargo test --test chapter06 l09_
//!
//! PREDICT before running: Which commands should stop a loop, skip a line, or return an error immediately? Where should the error line number come from?
//! IMPLEMENT the tasks below. Preserve the tests and add an edge case.
//! EXPLAIN afterward: Explain the role of each expression construct, how every path produces Result, and why work after stop is not inspected.
//! COMPILER / TRANSFER LAB: Extend the language with reset. Before coding, decide whether reset is a command or a value and whether it changes error line numbering. Rewrite one let else as match and compare clarity.
//! Use examples/ch06_scratch.rs; repair intentional failures before committing.

/// A. Decode one trimmed line: blank/comment => Ok(None), integer =>
/// Ok(Some(i32)), invalid integer => Err(()). A comment line starts with '#'
/// after trimming; there are no trailing comments, so "4 # four" is invalid.
fn decode(line: &str) -> Result<Option<i32>, ()> {
    todo!("Implement this task")
}
/// B. Sum decoded lines until a trimmed "stop". Return Err(one_based_line)
/// for invalid input OR i32 sum overflow, ignoring all lines after stop.
/// Blank and comment lines still count toward line numbering. Check overflow
/// after every addition (not only at the end), and allocate nothing: walk the
/// borrowed lines with a loop; do not collect them or build Strings.
fn execute(lines: &[&str]) -> Result<i32, usize> {
    todo!("Implement this task")
}
#[cfg(test)]
mod tests {
    use super::*;
    use practice::alloc::count;
    #[test]
    fn a_decode_separates_skips_from_errors() {
        assert_eq!(decode(" -3 "), Ok(Some(-3)));
        assert_eq!(decode(" # note"), Ok(None));
        assert_eq!(decode("  "), Ok(None));
        assert_eq!(decode("bad"), Err(()));
        assert_eq!(decode("#"), Ok(None));
        assert_eq!(decode("+7"), Ok(Some(7)));
        assert_eq!(decode("4 # four"), Err(()));
        assert_eq!(decode("2147483648"), Err(()));
        assert_eq!(decode("\u{3000}12\u{3000}"), Ok(Some(12)));
    }
    #[test]
    fn b_engine_controls_evaluation() {
        assert_eq!(execute(&["# h", "2", "-1", " stop ", "bad"]), Ok(1));
        assert_eq!(execute(&["", "bad"]), Err(2));
        assert_eq!(execute(&["2147483647", "1"]), Err(2));
        assert_eq!(execute(&[]), Ok(0));
        assert_eq!(execute(&["stop", "2147483647", "1"]), Ok(0));
        assert_eq!(execute(&["-2147483648", "-1"]), Err(2));
        assert_eq!(execute(&["1", "STOP"]), Err(2), "stop is case-sensitive");
    }
    #[test]
    fn b_overflow_is_checked_at_every_step() {
        // A wider running total would end at i32::MAX and miss the overflow on line 2.
        assert_eq!(execute(&["2147483647", "1", "-1"]), Err(2));
        assert_eq!(execute(&["2147483647", "-1", "1"]), Ok(i32::MAX));
    }
    #[test]
    fn a_b_engine_does_not_allocate() {
        let lines: Vec<String> = (0..100).map(|i| format!(" {i} ")).collect();
        let mut borrowed: Vec<&str> = lines.iter().map(String::as_str).collect();
        borrowed.push("# done");
        let (sum, allocations) = count(|| execute(&borrowed));
        assert_eq!(sum, Ok(4950));
        assert_eq!(allocations, 0, "execute should borrow every line");
    }
}
