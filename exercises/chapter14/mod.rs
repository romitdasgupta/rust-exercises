//! Chapter 14: Closures. Original guided practice.
//! Run one lesson: cargo test --test chapter14 l01_
#![allow(dead_code, unused_variables, unused_mut, unused_imports)]

mod c01_code_review;
mod l01_borrowed;
mod l02_capture_modes;
mod l03_call_traits;
mod l04_returning;
mod l05_cloning;
mod l06_callbacks;
mod l07_capstone;
mod r01_review;

// Lets tests count heap allocations, so they can check how you solved a task.
#[global_allocator]
static ALLOC: practice::alloc::Counting = practice::alloc::Counting;
