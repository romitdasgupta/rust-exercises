//! LESSON 06 — Owner-bound foreign borrows
//! Run: cargo test --test chapter24 l06_
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider: Does a raw pointer return type express how long a name remains valid? How can PhantomData make a foreign child depend on its owner?
//! IMPLEMENT the tasks below. Preserve the tests and add an edge case.
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
    use practice::guess;
    #[test]
    fn a_b_view_borrows_owner_storage() {
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
    #[test]
    fn a_b_views_share_one_buffer() {
        let repo = Repository {
            name: CString::new("naïve").unwrap(),
        };
        let (a, b) = (repo.view(), repo.view());
        assert_eq!(a.as_cstr().as_ptr(), b.as_cstr().as_ptr());
        assert_eq!(a.as_cstr().to_str(), Ok("naïve"));
        let empty = Repository {
            name: CString::default(),
        };
        assert_eq!(empty.view().as_cstr(), c"");
    }
    #[test]
    fn b_the_cstr_outlives_its_view() {
        let repo = Repository {
            name: CString::new("long-lived").unwrap(),
        };
        let name: &CStr = {
            let view = repo.view();
            view.as_cstr()
        };
        assert_eq!(name, c"long-lived", "the borrow is tied to the repository");
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    #[test]
    fn p_predictions() {
        // P1. Moving a CString into a struct: does its heap buffer move too?
        let name = CString::new("moved").unwrap();
        let before = name.as_ptr();
        let repo = Repository { name };
        let p1: bool = guess(); // true if the buffer address is unchanged
        assert_eq!(p1, repo.name.as_ptr() == before, "P1: moving an owner");

        // P2. CStr::from_ptr reads up to the first NUL. Length of the result?
        let buffer = b"ab\0cd\0";
        // SAFETY: the buffer is live and contains a NUL.
        let first = unsafe { CStr::from_ptr(buffer.as_ptr().cast::<c_char>()) };
        let p2: usize = guess();
        assert_eq!(p2, first.to_bytes().len(), "P2: from_ptr");

        // P3. Size of a NameView on a 64-bit target.
        let p3: usize = guess();
        assert_eq!(
            p3,
            size_of::<NameView<'_>>(),
            "P3: raw pointer plus PhantomData"
        );
    }
}
