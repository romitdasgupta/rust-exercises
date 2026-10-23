//! DRILL 08-02 · A pub struct with a private field
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch08`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: can code outside the module write every field of this struct?

mod tickets {
    #[derive(Debug)]
    pub struct Ticket {
        pub id: u32,
        owner: String,
    }

    impl Ticket {
        pub fn new(id: u32, owner: &str) -> Ticket {
            Ticket { id, owner: owner.trim().to_string() }
        }
    }
}

fn main() {
    let t = tickets::Ticket { id: 7, owner: "ada".to_string() };
    assert_eq!(t.id, 7); // KEEP
}
