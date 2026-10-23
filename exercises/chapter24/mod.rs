//! Original guided exercises; implement TODOs and answer the prompts.
#![allow(
    unused_variables,
    unused_mut,
    unused_imports,
    dead_code,
    unreachable_code
)]
mod c01_code_review;
mod l01_representations;
mod l02_c_strings;
mod l03_abi_status;
mod l04_callbacks;
mod l05_handles;
mod l06_borrows;
mod l07_capstone;
mod r01_review;

// Lets tests count heap allocations, so they can check how you solved a task.
#[global_allocator]
static ALLOC: practice::alloc::Counting = practice::alloc::Counting;
