//! DRILL 10-04 · Or-patterns must bind the same names
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch10`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: which value would `code` hold for a Click?

enum Event {
    Key(char),
    Click,
    Tick,
}

fn source(event: &Event) -> &'static str {
    match event {
        Event::Key(code) | Event::Click => "input",
        Event::Tick => "timer",
    }
}

fn main() {
    assert_eq!(source(&Event::Key('q')), "input"); // KEEP
    assert_eq!(source(&Event::Click), "input"); // KEEP
    assert_eq!(source(&Event::Tick), "timer"); // KEEP
}
