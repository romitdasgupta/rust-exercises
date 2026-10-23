//! LESSON 07 — Operators, calls, fields and assignment
//! Run: cargo test --test chapter06 l07_
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider: Does false && right() evaluate right? What value does an assignment expression return? Is a < b < c legal?
//! IMPLEMENT the tasks below. Preserve the tests and add an edge case.
//! EXPLAIN afterward: Explain evaluation order, precedence, reference operators, bit masks, and the difference between assignment and equality.
//! COMPILER / TRANSFER LAB: Create a tuple containing an array, borrow it, and access `.0[1]`. Compare `&array[index]` with `(&array)[index]`. Try `let result = (x += 1);` and print its type via a unit assertion. Show arithmetic precedence with parentheses; do not rely on wrapping overflow.
//! Use examples/ch06_scratch.rs; repair intentional failures before committing.

/// A. Pack only the low four bits of each input into a byte: high in bits 4–7,
/// low in bits 0–3. Use &, |, and <<; ignore all higher input bits.
fn packed(high: u8, low: u8) -> u8 {
    todo!("Implement this task")
}
/// B. Return enabled && (value > 0), incrementing calls exactly when the
/// comparison is evaluated. Use short-circuit && rather than an if statement.
fn counted_check(enabled: bool, value: i32, calls: &mut u32) -> bool {
    todo!("Implement this task")
}
#[cfg(test)]
mod tests {
    use super::*;
    use practice::guess;
    #[test]
    fn a_bits_are_masked_before_packing() {
        assert_eq!(packed(2, 3), 0x23);
        assert_eq!(packed(0xff, 0xa2), 0xf2);
        assert_eq!(packed(0, 0), 0);
        assert_eq!(packed(0x01, 0x30), 0x10, "mask the low input too");
        assert_eq!(packed(0x5f, 0xff), 0xff);
    }
    #[test]
    fn b_right_operand_runs_only_when_needed() {
        let mut calls = 0;
        assert!(!counted_check(false, 9, &mut calls));
        assert_eq!(calls, 0);
        assert!(!counted_check(true, -1, &mut calls));
        assert!(counted_check(true, 2, &mut calls));
        assert_eq!(calls, 2);
        assert!(!counted_check(true, 0, &mut calls));
        assert!(!counted_check(false, -5, &mut calls));
        assert_eq!(calls, 3);
    }

    fn logged(log: &mut Vec<char>, name: char, value: bool) -> bool {
        log.push(name);
        value
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    #[test]
    fn p_predictions() {
        // P1. Unary minus binds looser than a method call.
        #[allow(clippy::precedence)]
        let value = -2i32.pow(2);
        let p1: i32 = guess();
        assert_eq!(p1, value, "P1: precedence of unary minus");

        // P2. Right shift of an unsigned and of a negative signed value.
        let p2: (u8, i8) = guess();
        assert_eq!(
            p2,
            (!0u8 >> 4, -16i8 >> 2),
            "P2: logical versus arithmetic shift"
        );

        // P3. && binds tighter than ||. Which operands run, and the result?
        let mut log = Vec::new();
        let result = logged(&mut log, 'a', false) && logged(&mut log, 'b', true)
            || logged(&mut log, 'c', true);
        let p3: (Vec<char>, bool) = guess();
        assert_eq!(p3, (log, result), "P3: short-circuit order");
    }
}
