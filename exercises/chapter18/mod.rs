//! Original guided exercises; implement TODOs and answer the prompts.
#![allow(
    unused_variables,
    unused_mut,
    unused_imports,
    dead_code,
    unreachable_code
)]
mod c01_code_review;
mod l01_readers;
mod l02_buffered;
mod l03_writers;
mod l04_seek_binary;
mod l05_paths;
mod l06_directories;
mod l07_capstone;
mod r01_review;

// Lets tests count heap allocations, so they can check how you solved a task.
#[global_allocator]
static ALLOC: practice::alloc::Counting = practice::alloc::Counting;
