//! Original guided exercises; implement TODOs and answer the prompts.
#![allow(
    unused_variables,
    unused_mut,
    unused_imports,
    dead_code,
    unreachable_code
)]
mod c01_code_review;
mod l01_fork_join;
mod l02_scoped;
mod l03_channels;
mod l04_send_sync;
mod l05_mutexes;
mod l06_coordination;
mod l07_atomics_tls;
mod l08_capstone;
mod r01_review;

// Lets tests count heap allocations, so they can check how you solved a task.
#[global_allocator]
static ALLOC: practice::alloc::Counting = practice::alloc::Counting;
