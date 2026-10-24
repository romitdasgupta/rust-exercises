//! LESSON 05 — Opaque handle ownership
//! Run: cargo test --test chapter24 l05_
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider: Why must an opaque pointer not be freed with an arbitrary allocator? Could two RAII owners created from one raw handle double-free it?
//! IMPLEMENT the tasks below. Preserve the tests and add an edge case.
//! EXPLAIN: Specify the ownership transfer at construction and exactly-once destructor call. Explain why the mock uses Box consistently and a real library needs its own free function.
//! COMPILER LAB (examples/ch24_scratch.rs): Try deriving Clone for the owner conceptually; explain why shallow pointer cloning would break ownership. Design an explicit shared/refcounted API instead.

use std::{
    ffi::{CStr, CString, NulError},
    ptr::NonNull,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
};
struct Resource {
    name: CString,
    closed: Arc<AtomicUsize>,
}
impl Drop for Resource {
    fn drop(&mut self) {
        self.closed.fetch_add(1, Ordering::Relaxed);
    }
}
// Supplied allocator pair models a foreign library; keep it unchanged.
fn raw_open(name: CString, closed: Arc<AtomicUsize>) -> *mut Resource {
    Box::into_raw(Box::new(Resource { name, closed }))
}
// # Safety: p is the unique live owned result of raw_open; call exactly once.
unsafe extern "C" fn raw_close(p: *mut Resource) {
    if !p.is_null() {
        // SAFETY: unique ownership is returned to the exact allocator used by raw_open.
        unsafe {
            drop(Box::from_raw(p));
        }
    }
}
struct Owner {
    raw: NonNull<Resource>,
}
impl Owner {
    /// A. Validate C text before allocation, take unique ownership of raw_open,
    /// and retain it as NonNull. raw_open never returns null in this mock.
    fn open(name: &str, closed: Arc<AtomicUsize>) -> Result<Self, NulError> {
        todo!("A: implement the contract")
    }
    /// B. Copy the resource name to owned Rust text. This mock only stores valid
    /// UTF-8 created by open. Never let a borrow escape beyond Owner's lifetime.
    fn name_owned(&self) -> String {
        todo!("B: implement the contract")
    }
}
// Safe cleanup scaffold: no TODO or fallible work in Drop.
impl Drop for Owner {
    fn drop(&mut self) {
        // SAFETY: Owner uniquely owns this live handle and cannot be cloned; raw_close is the matching deallocator.
        unsafe {
            raw_close(self.raw.as_ptr());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use practice::guess;
    #[test]
    fn a_b_handle_frees_exactly_once_and_copy_outlives_it() {
        let closed = Arc::new(AtomicUsize::new(0));
        let owner = Owner::open("repo-é", Arc::clone(&closed)).unwrap();
        let name = owner.name_owned();
        assert_eq!(closed.load(Ordering::Relaxed), 0);
        drop(owner);
        assert_eq!(closed.load(Ordering::Relaxed), 1);
        assert_eq!(name, "repo-é");
    }
    #[test]
    fn a_invalid_text_never_allocates_resource() {
        let closed = Arc::new(AtomicUsize::new(0));
        assert!(Owner::open("bad\0name", Arc::clone(&closed)).is_err());
        assert_eq!(closed.load(Ordering::Relaxed), 0);
        assert_eq!(Arc::strong_count(&closed), 1, "no Resource holds a clone");
    }
    #[test]
    fn a_b_each_owner_closes_its_own_handle() {
        let closed = Arc::new(AtomicUsize::new(0));
        let first = Owner::open("", Arc::clone(&closed)).unwrap();
        let second = Owner::open("🦀 second", Arc::clone(&closed)).unwrap();
        assert_eq!(Arc::strong_count(&closed), 3);
        assert_eq!(first.name_owned(), "");
        assert_eq!(second.name_owned(), "🦀 second");
        drop(second);
        assert_eq!(closed.load(Ordering::Relaxed), 1);
        assert_eq!(first.name_owned(), "", "the first handle is untouched");
        drop(first);
        assert_eq!(closed.load(Ordering::Relaxed), 2);
        assert_eq!(Arc::strong_count(&closed), 1);
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    #[test]
    fn p_predictions() {
        // P1. Sizes on a 64-bit target: Owner and Option<Owner>.
        let p1: (usize, usize) = guess();
        assert_eq!(
            p1,
            (size_of::<Owner>(), size_of::<Option<Owner>>()),
            "P1: NonNull niche"
        );

        // P2. Close a raw handle once, then "close" a null pointer.
        // The closed counter after each call?
        let closed = Arc::new(AtomicUsize::new(0));
        let raw = raw_open(CString::new("raw").unwrap(), Arc::clone(&closed));
        // SAFETY: raw is the unique live result of raw_open, closed once.
        unsafe { raw_close(raw) };
        let after_close = closed.load(Ordering::Relaxed);
        // SAFETY: raw_close accepts null and does nothing.
        unsafe { raw_close(std::ptr::null_mut()) };
        let p2: (usize, usize) = guess();
        assert_eq!(
            p2,
            (after_close, closed.load(Ordering::Relaxed)),
            "P2: raw_close"
        );

        // P3. strong_count of `counter` while a raw handle is open, and after it closes.
        let counter = Arc::new(AtomicUsize::new(0));
        let raw = raw_open(CString::new("count").unwrap(), Arc::clone(&counter));
        let while_open = Arc::strong_count(&counter);
        // SAFETY: as above.
        unsafe { raw_close(raw) };
        let p3: (usize, usize) = guess();
        assert_eq!(
            p3,
            (while_open, Arc::strong_count(&counter)),
            "P3: who owns the clone"
        );
    }
}
