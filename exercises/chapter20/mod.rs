//! Original guided exercises; implement TODOs and answer the prompts.
#![allow(
    unused_variables,
    unused_mut,
    unused_imports,
    dead_code,
    unreachable_code
)]
mod c01_code_review;
mod l01_lazy;
mod l02_lifetimes;
mod l03_traits;
mod l04_async_closures;
mod l05_cancellation;
mod l06_racing;
mod l07_capstone;
mod r01_review;

// Lets tests count heap allocations, so they can check how you solved a task.
#[global_allocator]
static ALLOC: practice::alloc::Counting = practice::alloc::Counting;
