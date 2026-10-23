//! DRILL 13-05 · From and TryFrom for the same type
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch13`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: what does std already implement once you write From?

#[derive(Debug, PartialEq)]
struct Port(u16);

impl From<u16> for Port {
    fn from(n: u16) -> Self {
        Port(n)
    }
}

impl TryFrom<u16> for Port {
    type Error = &'static str;
    fn try_from(n: u16) -> Result<Self, Self::Error> {
        if n == 0 { Err("port 0 is reserved") } else { Ok(Port(n)) }
    }
}

fn main() {
    assert_eq!(Port::try_from(0), Err("port 0 is reserved")); // KEEP
    assert_eq!(Port::try_from(80), Ok(Port(80))); // KEEP
}
