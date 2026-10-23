//! DRILL 05-08 · Moving an owner while it is borrowed
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch05`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: what would `view` point at after the move?

struct View<'a> {
    text: &'a str,
}

fn consume(s: String) -> usize {
    s.len()
}

fn main() {
    let source = String::from("config");
    let view = View { text: &source[..3] };
    let n = consume(source);
    assert_eq!(view.text, "con"); // KEEP
    assert_eq!(n, 6); // KEEP
}
