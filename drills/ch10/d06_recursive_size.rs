//! DRILL 10-06 · A recursive enum
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch10`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: how big would a List be if it contained a List?

enum List {
    Cons(i32, List),
    Nil,
}

fn sum(list: &List) -> i32 {
    match list {
        List::Cons(head, tail) => head + sum(tail),
        List::Nil => 0,
    }
}

fn main() {
    let list = List::Cons(1, List::Cons(2, List::Nil));
    assert_eq!(sum(&list), 3); // KEEP
}
