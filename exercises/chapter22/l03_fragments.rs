//! LESSON 03 — Type and identifier fragments
//! Run: cargo test --test chapter22 l03_
//!
//! PREDICT before running:
//! 1. Why can a $ty:ty fragment not be replaced by $name:ident in every type position?
//! 2. Does a macro-generated constructor obey ordinary privacy/type rules?
//!
//! IMPLEMENT the tasks; keep the tests.
//! EXPLAIN: Explain which tokens the macro parses and which rules the Rust compiler still checks after expansion.
//! COMPILER LAB (examples/ch22_scratch.rs): Invoke define_cell with Vec<u8> and then with a non-type expression; compare diagnostics. Try forwarding an expr fragment into a literal-matching helper.

/// The item scaffold deliberately exists so unfinished exercises compile.
/// A/B. Implement generated new/get bodies. The macro accepts an identifier
/// and any Rust type, generating a value-owning struct with borrowed access.
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
    #[test]
    fn generated_items_obey_the_type_contract() {
        let cell = Labels::new(vec!["one".into()]);
        assert_eq!(cell.get(), &[String::from("one")]);
        assert_eq!(cell.get().as_ptr(), cell.get().as_ptr());
    }
    #[test]
    fn type_fragment_accepts_a_selected_parse_target() {
        let a: Result<u16, std::num::ParseIntError> = parse_as!("42"=>u16);
        assert_eq!(a, Ok(42));
        let b: Result<i32, std::num::ParseIntError> = parse_as!("bad"=>i32);
        assert!(b.is_err());
    }
}
