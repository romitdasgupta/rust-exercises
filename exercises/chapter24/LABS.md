# Real foreign-library projects

The automated core uses supplied Rust-defined `extern "C"` mocks. It exercises ABI types and safe-wrapper obligations without a system linker dependency. These isolated projects are required practice for real linking and libgit2; they are not run by the root chapter tests.

## Local C library and exported Rust entry point

Create an isolated Cargo package with build.rs and the current `cc` build dependency, following the cc crate and Rust FFI documentation. Write a small C library implementing a repr(C)-compatible record transform, status-plus-output division, a callback taking void* userdata, and a read-only library version variable. Include a real header; use its exact C integer types and size_t, mapping to std::ffi c_* aliases/usize. Compile C and Rust sizeof/alignment/offsetof checks and compare on this target. Declare functions/variables in an `unsafe extern "C"` block; safe wrappers must explain every precondition. Avoid mutable global state. Exercise owned CString arguments, borrowed CStr returns, null handling, initialized output only on success, and exactly-matched allocation/free functions.

Add an exported Rust function with Rust 2024 `#[unsafe(no_mangle)]` and extern C ABI, build a staticlib/cdylib in this isolated package, and call it from a tiny C driver. Give all exports unique names. Do not let a Rust panic unwind through C: validate inputs without panicking and catch_unwind around an arbitrary user closure in a callback trampoline, recording failure in explicit userdata and returning a C error code. Document that allocation aborts and panic=abort are not catchable. Acceptance: success/failure/null cases, callback userdata lifetime, panic containment, C driver output, and an explanation of linker flags/library search paths. Never free foreign malloc memory with Box::from_raw or Rust memory with C free.

## Raw and safe libgit2 interface

Use an isolated temporary project and a locally installed libgit2 discovered with pkg-config (or the documented platform equivalent). Do not silently assume it is installed or modify system packages without an explicit user request. Pin/document the version you build against and use the matching official headers/docs. Create your own temporary repository fixture, no remote fetch/push/network and no writes to a real user repository.

Stage 1: declare the exact init/shutdown, repository_open, repository_free, revparse/commit lookup, commit_free, and error_last signatures using opaque types and C aliases. Link through the actual library. On failed creation, do not assume the out pointer initialized; copy error_last message and class immediately before another library call can invalidate thread-local error storage. Treat null error pointers as a separate fallback case.

Stage 2: implement an RAII library Session and Repository owner. Ensure library initialization precedes all handles and shutdown follows every handle drop, including early Result returns. Make cleanup nonpanicking. Use the matching libgit2 free for every allocation. Represent Commit<'repo> with a repository lifetime (or a documented owning shared relationship) so it cannot outlive its repo; expose borrowed strings only for their documented valid lifetime or copy them. Do not add unsafe Send/Sync without proving the library's version-specific threading guarantees.

Acceptance: open existing temp repo, nonexistent path with copied diagnostic, invalid embedded NUL path before FFI, lookup failure without reading uninitialized output, owner/child drop ordering, and a compile-fail child-outlives-owner example. Write an ownership table for every pointer (borrowed/owned, null policy, validity interval, free function, thread constraints). Add one commit fixture through local git commands or libgit2; compare raw and safe calls. Test cleanup on a deliberately injected Rust-side error after repository creation. Explain init reference counts and why a safe Rust lifetime alone does not establish foreign thread safety.

Report toolchain, target, libgit2 version, linker command/result, and platform limitations. Optional external projects require their dependencies/toolchain and are not claimed as automated validation of the portable mock core.

References: [Rustonomicon FFI](https://doc.rust-lang.org/nomicon/ffi.html), [std::ffi](https://doc.rust-lang.org/std/ffi/), [cc](https://docs.rs/cc/latest/cc/), and [libgit2 API (choose your installed version)](https://libgit2.org/docs/reference/main/).
