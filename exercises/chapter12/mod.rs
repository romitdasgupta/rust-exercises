//! Chapter 12: Operator Overloading. Original guided practice.
//! Run one lesson: cargo test --test chapter12 l01_
#![allow(dead_code, unused_variables, unused_mut, unused_imports)]

mod c01_code_review;
mod l01_arithmetic;
mod l02_equality;
mod l03_ordering;
mod l04_indexing;
mod l05_capstone;
mod r01_review;

// Lets tests count heap allocations, so they can check how you solved a task.
#[global_allocator]
static ALLOC: practice::alloc::Counting = practice::alloc::Counting;
