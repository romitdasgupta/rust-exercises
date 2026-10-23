//! CODE REVIEW 01 — A teammate's macro pull request
//! Run: cargo test --test chapter22 c01_
//! Then: cargo clippy --test chapter22
//!
//! This is the kind of code you will be asked to review at work. It compiles,
//! and most behavior tests pass. Your job:
//! 1. Before running anything, read the code and write a review in NOTES.md:
//!    list every bug, every unnecessary allocation, and every place where an
//!    error is swallowed or a panic replaces propagation.
//! 2. Run the tests. Compare what they found with your review.
//! 3. Fix it. Keep each macro's invocation syntax; the tests use it.
//! 4. Run clippy and resolve its warnings for this file.

use std::fmt;

/// Return the larger of two expressions; ties return the first. Like a
/// function call, evaluate each argument exactly once, left to right.
macro_rules! max_of {
    ($a:expr, $b:expr) => {
        if $a >= $b { $a } else { $b }
    };
}

/// Build a Vec of "#tag" Strings, one per argument, in order. Exported for
/// use anywhere in the crate; it must always call THIS module's `tag_name`,
/// even where the caller defines a function with the same name.
#[macro_export]
macro_rules! ch22_tags {
    ($($tag:expr),* $(,)?) => {{
        let mut tags = Vec::new();
        $(
            let name: String = tag_name($tag);
            tags.push(name.clone());
        )*
        tags
    }};
}

#[doc(hidden)]
pub fn tag_name(tag: &str) -> String {
    format!("#{}", tag.to_string())
}

/// Implement Display for unit newtypes: `Meters(3)` shows as "3 m". Errors
/// from the formatter must reach the caller of `write!`.
macro_rules! impl_unit_display {
    ($($ty:ident => $unit:literal),* $(,)?) => {
        $(
            impl fmt::Display for $ty {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    let text = format!("{} {}", self.0, $unit);
                    f.write_str(&text).unwrap();
                    Ok(())
                }
            }
        )*
    };
}

struct Meters(u32);
struct Seconds(f64);
impl_unit_display!(Meters => "m", Seconds => "s");

#[cfg(test)]
mod tests {
    use super::*;
    use practice::alloc::count;
    use std::fmt::Write;

    #[test]
    fn a_max_of_evaluates_each_argument_once() {
        let mut calls = 0;
        let mut next = || {
            calls += 1;
            calls
        };
        let biggest = max_of!(next(), next());
        assert_eq!(biggest, 2);
        assert_eq!(calls, 2, "two arguments, two evaluations");
    }

    #[test]
    fn a_max_of_returns_the_first_on_a_tie() {
        let zero = max_of!(-0.0_f64, 0.0_f64);
        assert!(zero.is_sign_negative(), "-0.0 == 0.0, so keep the first");
        assert_eq!(max_of!("b", "a"), "b");
    }

    #[test]
    fn b_tags_render_in_order() {
        assert_eq!(ch22_tags!("a", "naïve",), ["#a", "#naïve"]);
        let none: Vec<String> = ch22_tags!();
        assert!(none.is_empty());
    }

    mod caller {
        fn tag_name(_: &str) -> String {
            "wrong helper".into()
        }
        pub fn run() -> Vec<String> {
            crate::ch22_tags!("x")
        }
    }

    #[test]
    fn b_tags_use_the_defining_module_helper() {
        assert_eq!(caller::run(), ["#x"]);
    }

    #[test]
    fn b_tags_do_not_clone_or_regrow() {
        let (tags, allocations) =
            count(|| ch22_tags!("t0", "t1", "t2", "t3", "t4", "t5", "t6", "t7"));
        assert_eq!(tags.len(), 8);
        assert!(
            allocations <= 2 * 8 + 1,
            "8 tags cost {allocations} allocations"
        );
    }

    #[test]
    fn c_units_display() {
        assert_eq!(Meters(3).to_string(), "3 m");
        assert_eq!(Seconds(1.5).to_string(), "1.5 s");
    }

    /// A writer that always fails, like a closed pipe.
    struct Refuse;
    impl Write for Refuse {
        fn write_str(&mut self, _: &str) -> fmt::Result {
            Err(fmt::Error)
        }
    }

    #[test]
    fn c_display_propagates_formatter_errors() {
        assert!(write!(Refuse, "{}", Meters(3)).is_err());
    }

    #[test]
    fn c_display_writes_without_a_temporary_string() {
        let mut out = String::with_capacity(64);
        let (result, allocations) = count(|| write!(out, "{}", Meters(42)));
        assert!(result.is_ok());
        assert_eq!(out, "42 m");
        assert_eq!(allocations, 0, "write straight into the formatter");
    }
}
