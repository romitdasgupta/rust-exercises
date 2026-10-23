//! DRILL 13-07 · Cow cannot borrow a local
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch13`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: who owns the String that Cow::Borrowed would point at?

use std::borrow::Cow;

fn tidy(text: &str) -> Cow<'_, str> {
    if text.contains('\t') {
        let replaced = text.replace('\t', "    ");
        Cow::Borrowed(&replaced)
    } else {
        Cow::Borrowed(text)
    }
}

fn main() {
    assert_eq!(tidy("a\tb"), "a    b"); // KEEP
    assert!(matches!(tidy("ab"), Cow::Borrowed(_))); // KEEP
}
