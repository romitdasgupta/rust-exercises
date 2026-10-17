//! LESSON 05 — Opaque handle ownership
//! Run: cargo test --test chapter24 l05_
//!
//! PREDICT before running:
//! 1. Why must an opaque pointer not be freed with an arbitrary allocator?
//! 2. Could two RAII owners created from one raw handle double-free it?
//!
//! IMPLEMENT the tasks; keep the tests.
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
    #[test]
    fn handle_frees_exactly_once_and_copy_outlives_it() {
        let closed = Arc::new(AtomicUsize::new(0));
        let owner = Owner::open("repo-é", Arc::clone(&closed)).unwrap();
        let name = owner.name_owned();
        assert_eq!(closed.load(Ordering::Relaxed), 0);
        drop(owner);
        assert_eq!(closed.load(Ordering::Relaxed), 1);
        assert_eq!(name, "repo-é");
    }
    #[test]
    fn invalid_text_never_allocates_resource() {
        let closed = Arc::new(AtomicUsize::new(0));
        assert!(Owner::open("bad\0name", Arc::clone(&closed)).is_err());
        assert_eq!(closed.load(Ordering::Relaxed), 0);
    }
}
