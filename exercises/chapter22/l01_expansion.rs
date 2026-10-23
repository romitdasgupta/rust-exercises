//! LESSON 01 — Expansion and evaluation once
//! Run: cargo test --test chapter22 l01_
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider: Can a macro accidentally execute an argument twice? Do braces in a macro expansion make its locals visible afterward?
//! IMPLEMENT the tasks below. Preserve the tests and add an edge case.
//! EXPLAIN: Trace expansion and evaluation separately. Explain why syntax substitution can change control flow.
//! COMPILER LAB (examples/ch22_scratch.rs): Expand an argument containing return inside a function. Compare a macro with a function; record the different return scopes.

/// A. Evaluate an i32 expression exactly once; return twice its value.
/// Inputs guarantee no arithmetic overflow. Keep the temporary hygienic/local.
macro_rules! double_once {
    ($value:expr) => {{ todo!("A: implement the contract") }};
}
/// B. Return (file, line, module) using built-in macros at the invocation site.
macro_rules! location {
    () => {{ todo!("B: implement the contract") }};
}

#[cfg(test)]
mod tests {
    use super::*;
    use practice::guess;
    #[test]
    fn a_side_effect_runs_once_and_local_is_hygienic() {
        let mut calls = 0;
        let value = 50;
        let answer: i32 = double_once!({
            calls += 1;
            7
        });
        assert_eq!(answer, 14);
        assert_eq!(calls, 1);
        assert_eq!(value, 50);
        let negative: i32 = double_once!(-21);
        assert_eq!(negative, -42);
        let zero: i32 = double_once!(value - 50);
        assert_eq!(zero, 0);
    }
    #[test]
    fn b_builtins_capture_invocation() {
        let ((file, line, module), here): ((&str, u32, &str), u32) = (location!(), line!());
        assert_eq!(line, here, "line of the invocation, not of the macro");
        assert_eq!(file, file!());
        assert_eq!(module, module_path!());
        assert!(file.ends_with("l01_expansion.rs"));
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    /// A wrong prediction is useful: read the failure, then explain it in NOTES.md.
    #[test]
    fn p_predictions() {
        // P1. A naive macro substitutes its argument twice. Result and calls?
        macro_rules! naive_double {
            ($e:expr) => {
                $e + $e
            };
        }
        let mut calls = 0;
        let result = naive_double!({
            calls += 1;
            calls
        });
        let p1: (i32, i32) = guess();
        assert_eq!(p1, (result, calls), "P1: textual substitution");

        // P2. Unlike a C preprocessor macro, does `$x * $x` regroup `2 + 3`?
        macro_rules! square {
            ($x:expr) => {
                $x * $x
            };
        }
        let p2: i32 = guess();
        assert_eq!(p2, square!(2 + 3), "P2: grouping of expr fragments");

        // P3. The macro declares its own `x`. Which value does ours hold?
        macro_rules! shadow_x {
            ($v:expr) => {
                let x = $v;
                let _ = x;
            };
        }
        let x = 1;
        shadow_x!(5);
        let p3: i32 = guess();
        assert_eq!(p3, x, "P3: hygiene of macro-defined locals");
    }
}
