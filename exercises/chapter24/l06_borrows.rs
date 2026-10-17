//! LESSON 06 — Owner-bound foreign borrows
//! Run: cargo test --test chapter24 l06_
//!
//! PREDICT before running:
//! 1. Does a raw pointer return type express how long a name remains valid?
//! 2. How can PhantomData make a foreign child depend on its owner?
//!
//! IMPLEMENT the tasks; keep the tests.
//! EXPLAIN: Explain the lifetime of the returned borrowed object and the difference between it and an owned copy. Relate this to repositories and commits.
//! COMPILER LAB (examples/ch24_scratch.rs): Try returning a NameView from a block whose Repository is dropped. Keep the compiler error, then repair by copying the name.

use std::{
    ffi::{CStr, CString, c_char},
    marker::PhantomData,
};
struct Repository {
    name: CString,
}
struct NameView<'repo> {
    raw: *const c_char,
    owner: PhantomData<&'repo Repository>,
}
impl Repository {
    /// A. Create a view tied to this repository borrow. The CString's allocation
    /// remains valid because name is private and unmodified while borrowed.
    fn view(&self) -> NameView<'_> {
        todo!("A: implement the contract")
    }
}
impl<'repo> NameView<'repo> {
    /// B. Reconstruct the borrowed CStr using the owner lifetime invariant.
    fn as_cstr(&self) -> &'repo CStr {
        todo!("B: implement the contract")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn view_borrows_owner_storage() {
        let repo = Repository {
            name: CString::new("project").unwrap(),
        };
        let view = repo.view();
        assert_eq!(view.as_cstr(), c"project");
        assert_eq!(view.as_cstr().as_ptr(), repo.name.as_ptr());
        let copy = view.as_cstr().to_owned();
        drop(repo);
        assert_eq!(copy, c"project");
    }
}
