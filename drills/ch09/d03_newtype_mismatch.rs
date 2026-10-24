//! DRILL 09-03 · Newtypes are distinct types
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch09`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: does the same representation make two tuple structs interchangeable?

struct UserId(u64);
struct OrderId(u64);

fn cancel(order: OrderId) -> u64 {
    order.0
}

fn main() {
    let user = UserId(7);
    let order = OrderId(42);
    let cancelled = cancel(user);
    assert_eq!(cancelled, 42); // KEEP
}
