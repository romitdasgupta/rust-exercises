//! LESSON 06 — The formatting language
//! Run: cargo test --test chapter17 l06_
//!
//! PREDICT before executing: Does field width truncate long text? What does precision mean
//! for a string versus a float? Is terminal column width equal to formatting width?
//!
//! IMPLEMENT the documented tasks, then keep the tests and add an edge case.
//! EXPLAIN afterward: How do named/indexed arguments and dynamic parameters make a
//! formatter reusable? Why should pointer addresses never be hard-coded in tests?
//!
//! COMPILER / MANUAL LAB (use examples/ch17_scratch.rs):
//! Format integers in decimal/binary/octal/hex with #, signs, and zero padding; floats in
//! fixed/scientific notation; vectors with {:?} and {:#?}; references with {:p}. Use {1}
//! {0} and named arguments. Verify fmt width counts Unicode scalars rather than UTF-8 bytes
//! or terminal columns.
//!
//! TRANSFER: invent a new edge case, predict it, then add a check. A lesson is
//! complete only when its tests, predictions, lab, and explanation are done.

/// A. Right-align label in width Unicode scalars, then ": ", then value with
/// precision digits after decimal. Inputs are finite nonnegative floats.
fn row(label: &str, value: f64, width: usize, precision: usize) -> String {
    todo!("Task 1: implement the documented contract")
}
/// B. Format u16 as lowercase hex with 0x prefix and total minimum width 6,
/// zero-filled after prefix (e.g. 10 -> "0x000a").
fn hex_code(value: u16) -> String {
    todo!("Task 2: implement the documented contract")
}
/// C. Exact one-line debug representation of a slice, not a custom serializer.
fn debug_values(values: &[i32]) -> String {
    todo!("Task 3: implement the documented contract")
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a_dynamic_parameters_control_alignment_and_rounding() {
        assert_eq!(row("air", 1.25, 5, 1), "  air: 1.2");
        assert_eq!(row("é", 2.0, 3, 2), "  é: 2.00");
        assert_eq!(row("long", 0.0, 2, 0), "long: 0");
    }
    #[test]
    fn b_numeric_flags_keep_prefix_before_padding() {
        assert_eq!(hex_code(10), "0x000a");
        assert_eq!(hex_code(u16::MAX), "0xffff");
        assert_eq!(hex_code(0), "0x0000");
    }
    #[test]
    fn c_debug_format_uses_standard_collection_syntax() {
        assert_eq!(debug_values(&[1, -2]), "[1, -2]");
        assert_eq!(debug_values(&[]), "[]");
    }
}
