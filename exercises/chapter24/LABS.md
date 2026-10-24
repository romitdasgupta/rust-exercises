# Real foreign-library projects

The automated core uses supplied Rust-defined `extern "C"` mocks. It exercises ABI types and safe-wrapper obligations without a system linker dependency. These isolated projects are required practice for real linking and libgit2; they are not run by the root chapter tests.

Work in a directory outside this repository. Both labs need a C toolchain; Lab 2 also needs a locally installed libgit2.

## Lab 1 · Local C library and exported Rust entry point

Goal: link a real C library into Rust, export a Rust function back to C, and keep panics from crossing the boundary.

Minimal `Cargo.toml` (check the cc crate's documentation for its current version):

```toml
[package]
name = "c-ffi-lab"
version = "0.1.0"
edition = "2024"

[lib]
crate-type = ["rlib", "staticlib", "cdylib"]  # staticlib/cdylib for the C driver (part B)

[build-dependencies]
cc = "1"
```

```rust
// build.rs
fn main() {
    cc::Build::new().file("c/lab.c").compile("lab");
    println!("cargo::rerun-if-changed=c/lab.c");
    println!("cargo::rerun-if-changed=c/lab.h");
}
```

### Part A: call C from Rust

1. Create the isolated package with the `build.rs` above, following the cc crate and Rust FFI documentation.
   *Checkpoint:* `cargo build` compiles an empty `c/lab.c` and links it.
2. Write `c/lab.h` and `c/lab.c`, a small C library with:
   - a transform of a record that is compatible with a Rust `#[repr(C)]` struct;
   - a division that returns a status and writes its result through an output pointer;
   - a function that takes a callback and a `void*` userdata;
   - a read-only library version variable.
   Use exact C integer types and `size_t` in the header.
3. Map each type to the `std::ffi` `c_*` aliases, and `size_t` to `usize`. Compile `sizeof`, `_Alignof`, and `offsetof` checks in C and `size_of`, `align_of`, and `offset_of!` checks in Rust, then compare them on this target.
   *Checkpoint:* the C and Rust numbers agree for every field of the record.
4. Declare the functions and the version variable in an `unsafe extern "C"` block. Write safe wrappers that explain every precondition in a SAFETY comment. Avoid mutable global state.
5. Exercise, through the safe wrappers: owned `CString` arguments, borrowed `CStr` returns, null handling, reading the output only on success, and allocation/free functions that exactly match.
   *Checkpoint:* the division wrapper returns `Err` on failure without reading the output slot, and the callback sees the userdata you passed.

### Part B: call Rust from C

6. Add an exported Rust function using Rust 2024 `#[unsafe(no_mangle)]` and the `extern "C"` ABI. Give every export a unique name.
7. Build it as a staticlib or cdylib in this isolated package and call it from a tiny C driver.
   *Checkpoint:* the C driver prints a value computed by your Rust function.
8. Do not let a Rust panic unwind through C. Validate inputs without panicking. In a callback trampoline that runs an arbitrary user closure, use `catch_unwind`, record the failure in explicit userdata, and return a C error code.
9. Document that allocation failure aborts and that `panic = "abort"` cannot be caught.
10. Explain the linker flags and library search paths your build used.

Never free foreign `malloc` memory with `Box::from_raw`, or Rust memory with C `free`.

Acceptance criteria:

- [ ] Success, failure, and null cases for each wrapper.
- [ ] Callback userdata lifetime demonstrated and explained.
- [ ] Panic containment: a panicking closure becomes a C error code, not an abort or UB.
- [ ] The C driver's output recorded.
- [ ] C and Rust layout checks agree.
- [ ] Written explanation of linker flags and library search paths, and of the abort cases.

## Lab 2 · Raw and safe libgit2 interface

Goal: a raw binding and a safe RAII wrapper for a real C library with its own allocator, error state, and initialization rules.

Prerequisites: a locally installed libgit2 discovered with pkg-config (or the documented platform equivalent). Do not silently assume it is installed, and do not modify system packages without an explicit user request. Pin and document the version you build against, and use the matching official headers and docs. Create your own temporary repository fixture. Do no remote fetch, push, or other network access, and never write to a real user repository.

Minimal `Cargo.toml`:

```toml
[package]
name = "git-ffi-lab"
version = "0.1.0"
edition = "2024"

[build-dependencies]
pkg-config = "0.3"
```

```rust
// build.rs: links libgit2 and fails clearly if it is not installed.
fn main() {
    pkg_config::probe_library("libgit2").expect("libgit2 not found by pkg-config");
}
```

*Checkpoint:* `cargo build` succeeds, and you have recorded the libgit2 version pkg-config found.

### Stage 1: raw interface

1. Declare the exact signatures of init and shutdown, `repository_open`, `repository_free`, a revparse/commit lookup, `commit_free`, and `error_last`, using opaque types and C aliases. Link through the actual library.
   *Checkpoint:* init followed by shutdown returns the documented status values.
2. On a failed creation call, do not assume the out pointer was initialized.
3. Copy the `error_last` message and class immediately, before another library call can overwrite the thread-local error storage. Treat a null error pointer as a separate fallback case.
   *Checkpoint:* opening a nonexistent path gives you an owned Rust error with libgit2's message, even after further calls.

### Stage 2: safe interface

4. Implement an RAII library `Session` and a `Repository` owner. Library initialization must precede every handle, and shutdown must follow every handle's drop, including on early `Result` returns.
5. Make cleanup non-panicking, and use the matching libgit2 free function for every allocation.
6. Represent `Commit<'repo>` with a repository lifetime (or a documented owning shared relationship) so it cannot outlive its repository. Expose borrowed strings only for their documented valid lifetime, or copy them.
   *Checkpoint:* a commit used after its repository is dropped fails to compile.
7. Do not add `unsafe impl Send`/`Sync` without proving the library's version-specific threading guarantees.
8. Add one commit to your fixture, using local git commands or libgit2, and compare the raw and the safe calls for it.
9. Inject a deliberate Rust-side error after repository creation and test that cleanup still runs.

Acceptance criteria:

- [ ] Opening the existing temp repository.
- [ ] A nonexistent path, with the copied diagnostic.
- [ ] A path with an embedded NUL, rejected before any FFI call.
- [ ] A lookup failure that never reads uninitialized output.
- [ ] Owner/child drop ordering tested.
- [ ] A compile-fail example of a child outliving its owner.
- [ ] An ownership table for every pointer: borrowed or owned, null policy, validity interval, free function, thread constraints.
- [ ] Cleanup verified after the injected Rust-side error.
- [ ] Written explanation of init reference counts, and of why a safe Rust lifetime alone does not establish foreign thread safety.

Report the toolchain, target, libgit2 version, linker command and result, and platform limitations. Optional external projects require their dependencies and toolchain, and are not claimed as automated validation of the portable mock core.

References: [Rustonomicon FFI](https://doc.rust-lang.org/nomicon/ffi.html), [std::ffi](https://doc.rust-lang.org/std/ffi/), [cc](https://docs.rs/cc/latest/cc/), and [libgit2 API (choose your installed version)](https://libgit2.org/docs/reference/main/).
