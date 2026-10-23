//! LESSON 08 — Capstone: initialized-prefix storage
//! Run: cargo test --test chapter23 l08_
//!
//! PREDICT before running: May MaybeUninit<T> itself be dropped before it contains a T? Why must len change before returning a moved-out value?
//! IMPLEMENT the tasks below. Preserve the tests and add an edge case.
//! EXPLAIN: Write the invariant len<=N and exactly data[..len] initialized. Audit push, pop, and provided cleanup for normal returns and panics.
//! COMPILER LAB (examples/ch23_scratch.rs): Add a panicking operation BEFORE a move and use catch_unwind in scratch. Verify existing values remain owned exactly once; never inject a panic into the move/bookkeeping interval.

use std::mem::MaybeUninit;
struct Slots<T, const N: usize> {
    data: [MaybeUninit<T>; N],
    len: usize,
}
impl<T, const N: usize> Slots<T, N> {
    fn new() -> Self {
        Self {
            data: [const { MaybeUninit::uninit() }; N],
            len: 0,
        }
    }
    /// A. Push into the next slot or return the unchanged value if full.
    /// Write before incrementing len, and run no user callback between them.
    fn push(&mut self, value: T) -> Result<(), T> {
        todo!("A: implement the contract")
    }
    /// B. Decrement len, then assume_init_read the former last initialized slot.
    /// Empty returns None. No clones or default placeholders.
    fn pop(&mut self) -> Option<T> {
        todo!("B: implement the contract")
    }
    /// C. Borrow only the initialized prefix as a slice. Never include unused
    /// slots, even when they happen to hold old representation bytes.
    fn as_slice(&self) -> &[T] {
        todo!("C: implement the contract")
    }
}
// Safe cleanup scaffold is supplied so unfinished learner functions cannot
// trigger a second TODO panic during unwinding. Preserve this implementation.
impl<T, const N: usize> Drop for Slots<T, N> {
    fn drop(&mut self) {
        while self.len > 0 {
            self.len -= 1; // SAFETY: invariant guarantees this former prefix element was initialized and is now removed from ownership bookkeeping.
            unsafe {
                self.data[self.len].assume_init_drop();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use practice::alloc::count;
    #[test]
    fn a_b_c_capacity_and_reuse_preserve_sequence() {
        let mut s = Slots::<String, 2>::new();
        s.push("a".into()).unwrap();
        s.push("b".into()).unwrap();
        assert_eq!(s.push("c".into()), Err("c".into()));
        assert_eq!(s.as_slice(), ["a", "b"]);
        assert_eq!(s.pop().as_deref(), Some("b"));
        s.push("d".into()).unwrap();
        assert_eq!(s.as_slice(), ["a", "d"]);
        assert_eq!(s.pop().as_deref(), Some("d"));
        assert_eq!(s.pop().as_deref(), Some("a"));
        assert!(s.pop().is_none());
        let mut zero = Slots::<u8, 0>::new();
        assert_eq!(zero.push(7), Err(7));
        assert!(zero.as_slice().is_empty());
    }
    #[test]
    fn a_b_every_owned_value_drops_once() {
        use std::sync::{
            Arc,
            atomic::{AtomicUsize, Ordering},
        };
        #[derive(Debug)]
        struct Spy(Arc<AtomicUsize>);
        impl Drop for Spy {
            fn drop(&mut self) {
                self.0.fetch_add(1, Ordering::Relaxed);
            }
        }
        let count = Arc::new(AtomicUsize::new(0));
        let mut s = Slots::<Spy, 3>::new();
        s.push(Spy(Arc::clone(&count))).unwrap();
        s.push(Spy(Arc::clone(&count))).unwrap();
        let out = s.pop().unwrap();
        assert_eq!(count.load(Ordering::Relaxed), 0);
        drop(s);
        assert_eq!(count.load(Ordering::Relaxed), 1);
        drop(out);
        assert_eq!(count.load(Ordering::Relaxed), 2);
    }
    #[test]
    fn a_b_values_move_in_and_out_without_copies() {
        let mut s = Slots::<String, 2>::new();
        let a = String::from("alpha");
        let pa = a.as_ptr();
        let (pushed, allocations) = count(|| s.push(a));
        assert_eq!((pushed, allocations), (Ok(()), 0));
        s.push("beta".into()).unwrap();
        let rejected = String::from("gamma");
        let pr = rejected.as_ptr();
        let back = s.push(rejected).unwrap_err();
        assert_eq!(back.as_ptr(), pr, "a rejected value comes back unchanged");
        assert_eq!(s.pop().as_deref(), Some("beta"));
        let (out, allocations) = count(|| s.pop());
        assert_eq!(allocations, 0, "pop moves the value out");
        let out = out.unwrap();
        assert_eq!(out.as_ptr(), pa);
    }
    #[test]
    fn c_slice_borrows_the_initialized_prefix() {
        let mut s = Slots::<u32, 4>::new();
        assert!(s.as_slice().is_empty());
        s.push(10).unwrap();
        s.push(20).unwrap();
        s.push(30).unwrap();
        assert_eq!(s.pop(), Some(30));
        let (len, allocations) = count(|| s.as_slice().len());
        assert_eq!((len, allocations), (2, 0));
        assert_eq!(s.as_slice(), [10, 20], "the popped slot is not included");
        assert_eq!(s.as_slice().as_ptr(), s.data.as_ptr().cast::<u32>());
    }
}
