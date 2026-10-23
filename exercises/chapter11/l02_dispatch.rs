//! LESSON 02 — Static and dynamic dispatch
//! Run: cargo test --test chapter11 l02_
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider: Can Vec<Box<dyn Measure>> contain two
//! concrete types? Can one Vec<T> do that without an enum or trait object?
//!
//! IMPLEMENT the documented tasks, then keep the tests and add an edge case.
//! EXPLAIN afterward: Which call sites require a vtable, which can be specialized, and why
//! does ?Sized matter for dyn Write?
//!
//! COMPILER / MANUAL LAB (use examples/ch11_scratch.rs):
//! Try a trait with fn duplicate(&self) -> Self, then form &dyn Trait. Repair it with where
//! Self: Sized and document which calls remain available. Compare sizes of &u32 and &dyn
//! Measure; do not assume vtable layout.
//!
//! TRANSFER: invent a new edge case, predict it, then add a check. A lesson is
//! complete only when its tests, predictions, lab, and explanation are done.

use std::io::{self, Write};
trait Measure {
    fn measure(&self) -> u32;
}
struct Fixed(u32);
impl Measure for Fixed {
    fn measure(&self) -> u32 {
        self.0
    }
}
struct Pair(u16, u16);
impl Measure for Pair {
    fn measure(&self) -> u32 {
        u32::from(self.0) + u32::from(self.1)
    }
}
/// A. Accept a homogeneous slice through a generic parameter and sum measurements.
fn total_static<T: Measure>(items: &[T]) -> u64 {
    todo!("Task 1: implement the documented contract")
}
/// B. Accept mixed concrete types via borrowed trait objects. Empty total is zero.
fn total_dynamic(items: &[&dyn Measure]) -> u64 {
    todo!("Task 2: implement the documented contract")
}
/// C. Accept both concrete writers and dyn Write. Write exactly b"record:" then body;
/// propagate errors.
fn write_record<W: Write + ?Sized>(out: &mut W, body: &[u8]) -> io::Result<()> {
    todo!("Task 3: implement the documented contract")
}
#[cfg(test)]
mod tests {
    use super::*;
    use practice::{alloc::count, guess};
    use std::hint::black_box;
    #[test]
    fn a_generic_dispatch_handles_empty_and_large_totals() {
        assert_eq!(total_static::<Fixed>(&[]), 0);
        assert_eq!(
            total_static(&[Fixed(u32::MAX), Fixed(2)]),
            u64::from(u32::MAX) + 2
        );
    }
    #[test]
    fn b_dynamic_dispatch_accepts_mixed_implementors() {
        let a = Fixed(9);
        let b = Pair(2, 3);
        assert_eq!(total_dynamic(&[&a, &b]), 14);
        assert_eq!(total_dynamic(&[]), 0);
    }
    #[test]
    fn c_writer_bound_supports_erasure_and_propagates_errors() {
        let mut bytes = Vec::new();
        let out: &mut dyn Write = &mut bytes;
        write_record(out, b"ok").unwrap();
        assert_eq!(bytes, b"record:ok");
        assert_eq!(write_record(&mut io::sink(), b"").unwrap(), ());
        struct Broken;
        impl Write for Broken {
            fn write(&mut self, _: &[u8]) -> io::Result<usize> {
                Err(io::Error::other("broken writer"))
            }
            fn flush(&mut self) -> io::Result<()> {
                Ok(())
            }
        }
        assert_eq!(
            write_record(&mut Broken, b"x").unwrap_err().to_string(),
            "broken writer"
        );
    }

    #[test]
    fn b_dynamic_total_does_not_overflow_u32() {
        let big = Fixed(u32::MAX);
        let pair = Pair(u16::MAX, 1);
        assert_eq!(total_dynamic(&[&big, &pair]), u64::from(u32::MAX) + 65_536);
    }

    #[test]
    fn c_short_writes_are_completed() {
        // A writer may accept fewer bytes than offered; `write` alone would lose data.
        struct OneByte(Vec<u8>);
        impl Write for OneByte {
            fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
                match buf.first() {
                    Some(&b) => {
                        self.0.push(b);
                        Ok(1)
                    }
                    None => Ok(0),
                }
            }
            fn flush(&mut self) -> io::Result<()> {
                Ok(())
            }
        }
        let mut out = OneByte(Vec::new());
        write_record(&mut out, "é!".as_bytes()).unwrap();
        assert_eq!(out.0, "record:é!".as_bytes());
    }

    trait Speak {
        fn word(&self) -> &'static str {
            "default"
        }
    }
    struct Quiet;
    impl Speak for Quiet {}
    struct Chatty;
    impl Speak for Chatty {
        fn word(&self) -> &'static str {
            "chatty"
        }
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    #[test]
    fn p_predictions() {
        // P1. Size in machine words (usize) of a thin and a fat (trait object) reference.
        let p1: (usize, usize) = guess();
        assert_eq!(
            (p1.0 * size_of::<usize>(), p1.1 * size_of::<usize>()),
            (size_of::<&Fixed>(), size_of::<&dyn Measure>()),
            "P1: &T vs &dyn Trait"
        );

        // P2. Heap allocations to build each two-element vector.
        let (boxed, boxed_allocations) = count(|| {
            black_box(vec![
                Box::new(Fixed(1)) as Box<dyn Measure>,
                Box::new(Pair(2, 3)),
            ])
        });
        let (plain, plain_allocations) = count(|| black_box(vec![Fixed(1), Fixed(2)]));
        let p2: (usize, usize) = guess();
        assert_eq!(
            p2,
            (boxed_allocations, plain_allocations),
            "P2: the cost of Box<dyn Trait>"
        );
        assert_eq!(boxed.len() + plain.len(), 4);

        // P3. Calls through &dyn Speak: which method body runs for each value?
        let all: [&dyn Speak; 2] = [&Quiet, &Chatty];
        let p3: [&str; 2] = guess();
        assert_eq!(
            p3,
            all.map(|s| s.word()),
            "P3: default methods in the vtable"
        );
    }
}
