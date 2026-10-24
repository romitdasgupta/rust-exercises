//! LESSON 05 — Traits handle macro leaf types
//! Run: cargo test --test chapter22 l05_
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider: Should a macro inspect the spelling of a variable to infer its type? Can a generic helper centralize type conversion for many macro invocations?
//! IMPLEMENT the tasks below. Preserve the tests and add an edge case.
//! EXPLAIN: Explain syntax/type separation and the trait bound error for an unsupported leaf.
//! COMPILER LAB (examples/ch22_scratch.rs): Try passing Vec<u8> to row!; add a trait implementation only if you can define an unambiguous representation.

trait Field {
    fn render(self) -> String;
}
impl Field for &str {
    fn render(self) -> String {
        self.to_owned()
    }
}
impl Field for i32 {
    fn render(self) -> String {
        self.to_string()
    }
}
impl Field for bool {
    fn render(self) -> String {
        if self { "yes".into() } else { "no".into() }
    }
}
/// A. Render a value by its Field implementation, without Debug formatting.
fn render<T: Field>(value: T) -> String {
    todo!("A: implement the contract")
}
/// B. Render zero or more expressions into a Vec<String>, via render. Support
/// trailing commas and evaluate all expressions exactly once, left to right.
macro_rules! row {
    ($($value:expr),* $(,)?) => {{ todo!("B: implement the contract") }};
}

#[cfg(test)]
mod tests {
    use super::*;
    use practice::guess;
    #[test]
    fn a_render_uses_the_field_impl() {
        assert_eq!(render(false), "no");
        assert_eq!(render(-3), "-3");
        assert_eq!(render("say \"hi\""), "say \"hi\"", "no Debug quoting");
        assert_eq!(render(""), "");
    }
    #[test]
    fn b_leaves_use_trait_behavior() {
        let value: i32 = 12;
        let row: Vec<String> = row!("item", value, true,);
        assert_eq!(row, ["item", "12", "yes"]);
        let empty: Vec<String> = row!();
        assert!(empty.is_empty());
    }
    #[test]
    fn b_expressions_are_evaluated_once() {
        let mut n = 0;
        let out: Vec<String> = row!(
            {
                n += 1;
                n
            },
            {
                n += 1;
                n
            }
        );
        assert_eq!(out, ["1", "2"]);
        assert_eq!(n, 2);
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    #[test]
    fn p_predictions() {
        trait Width {
            fn width(&self) -> usize;
        }
        impl Width for &str {
            fn width(&self) -> usize {
                self.chars().count()
            }
        }
        impl Width for char {
            fn width(&self) -> usize {
                1
            }
        }
        impl Width for u32 {
            fn width(&self) -> usize {
                self.to_string().len()
            }
        }
        macro_rules! widths {
            ($($v:expr),*) => { [$(Width::width(&$v)),*] };
        }
        // P1. Each leaf picks its own impl. What are the three widths?
        let p1: [usize; 3] = guess();
        assert_eq!(p1, widths!("née", 'x', 1000u32), "P1: per-leaf impls");

        trait Name {
            fn name(self) -> &'static str;
        }
        impl Name for i64 {
            fn name(self) -> &'static str {
                "i64"
            }
        }
        impl Name for &str {
            fn name(self) -> &'static str {
                "&str"
            }
        }
        fn name_of<T: Name>(value: T) -> &'static str {
            value.name()
        }
        // P2. An unsuffixed `7`, and only one integer type implements Name.
        // Which name is returned? (It compiles.)
        let p2: &str = guess();
        assert_eq!(p2, name_of(7), "P2: integer literal inference");
    }
}
