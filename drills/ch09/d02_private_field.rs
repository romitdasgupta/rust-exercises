//! DRILL 09-02 · Private fields outside the module
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch09`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: who may build a value whose fields are private?

mod shop {
    pub struct Price {
        cents: u32,
    }

    impl Price {
        pub fn cents(&self) -> u32 {
            self.cents
        }
    }
}

fn main() {
    let price = shop::Price { cents: 250 };
    assert_eq!(price.cents(), 250); // KEEP
}
