//! LESSON 03 — Type and identifier fragments
//! Run: cargo test --test chapter22 l03_
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider: Why can a $ty:ty fragment not be replaced by $name:ident in every type position? Does a macro-generated constructor obey ordinary privacy/type rules?
//! IMPLEMENT the tasks below. Preserve the tests and add an edge case.
//! EXPLAIN: Explain which tokens the macro parses and which rules the Rust compiler still checks after expansion.
//! COMPILER LAB (examples/ch22_scratch.rs): Invoke define_cell with Vec<u8> and then with a non-type expression; compare diagnostics. Try forwarding an expr fragment into a literal-matching helper.

/// The item scaffold deliberately exists so unfinished exercises compile.
/// A/B. Implement generated new/get bodies. The macro accepts an identifier
/// and any Rust type, generating a value-owning struct with borrowed access:
/// `new` moves the value in (no clone), `get` borrows it.
macro_rules! define_cell {
    ($name:ident : $ty:ty) => {
        struct $name {
            value: $ty,
        }
        impl $name {
            fn new(value: $ty) -> Self {
                todo!("A: implement the contract")
            }
            fn get(&self) -> &$ty {
                todo!("B: implement the contract")
            }
        }
    };
}
define_cell!(Labels:Vec<String>);
/// C. Evaluate input once and parse it as the selected type; return parse Result.
macro_rules! parse_as {
    ($input:expr => $target:ty) => {{ todo!("C: implement the contract") }};
}

#[cfg(test)]
mod tests {
    use super::*;
    use practice::{alloc::count, guess};
    define_cell!(Pair: (u8, &'static str));
    #[test]
    fn a_b_generated_items_obey_the_type_contract() {
        let cell = Labels::new(vec!["one".into()]);
        assert_eq!(cell.get(), &[String::from("one")]);
        assert_eq!(cell.get().as_ptr(), cell.get().as_ptr());
        let pair = Pair::new((7, "seven"));
        assert_eq!(pair.get(), &(7, "seven"));
    }
    #[test]
    fn a_b_new_moves_and_get_borrows() {
        let labels = vec![String::from("one"), String::from("two")];
        let p = labels.as_ptr();
        let (cell, allocations) = count(|| Labels::new(labels));
        assert_eq!(
            allocations, 0,
            "new should move the vector in, not clone it"
        );
        assert_eq!(
            cell.get().as_ptr(),
            p,
            "get should borrow the stored vector"
        );
    }
    #[test]
    fn c_type_fragment_accepts_a_selected_parse_target() {
        let a: Result<u16, std::num::ParseIntError> = parse_as!("42"=>u16);
        assert_eq!(a, Ok(42));
        let b: Result<i32, std::num::ParseIntError> = parse_as!("bad"=>i32);
        assert!(b.is_err());
        let c: Result<u8, std::num::ParseIntError> = parse_as!("256"=>u8);
        assert!(c.is_err(), "256 does not fit in u8");
    }
    #[test]
    fn c_target_may_be_a_path_and_input_is_evaluated_once() {
        let mut calls = 0;
        let nz: Result<std::num::NonZeroU8, std::num::ParseIntError> =
            parse_as!({ calls += 1; "5" } => std::num::NonZeroU8);
        assert_eq!(nz.map(|n| n.get()), Ok(5));
        assert_eq!(calls, 1);
        let zero: Result<std::num::NonZeroU8, std::num::ParseIntError> =
            parse_as!("0" => std::num::NonZeroU8);
        assert!(zero.is_err(), "zero is not a NonZeroU8");
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    #[test]
    fn p_predictions() {
        // Rules are tried top to bottom; the first one that matches wins.
        macro_rules! kind {
            ($l:literal) => {
                "literal"
            };
            ($i:ident) => {
                "ident"
            };
            ($e:expr) => {
                "expr"
            };
        }
        // P1. Which rule matches each of `-5`, `true`, and `(5)`?
        let p1: (&str, &str, &str) = guess();
        assert_eq!(
            p1,
            (kind!(-5), kind!(true), kind!((5))),
            "P1: fragment matching"
        );

        // P2. Once captured as `expr`, does `x` still match the `ident` rule?
        macro_rules! forward {
            ($e:expr) => {
                kind!($e)
            };
        }
        let x = 1;
        let p2: &str = guess();
        assert_eq!(p2, forward!(x), "P2: forwarding a captured fragment");

        // P3. The caller supplies the identifier. Which `total` do we see?
        macro_rules! set_to_ten {
            ($name:ident) => {
                let $name = 10;
            };
        }
        let total = 1;
        set_to_ten!(total);
        let p3: i32 = guess();
        assert_eq!(p3, total, "P3: hygiene of caller-supplied identifiers");
    }
}
