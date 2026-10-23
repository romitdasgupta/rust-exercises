//! Chapter 13: Utility Traits. Original guided practice.
//! Run one lesson: cargo test --test chapter13 l01_
#![allow(dead_code, unused_variables, unused_mut, unused_imports)]

mod c01_code_review;
mod l01_drop;
mod l02_sized_clone;
mod l03_deref_default;
mod l04_views;
mod l05_borrow;
mod l06_conversions;
mod l07_cow;
mod l08_capstone;
mod r01_review;

// Lets tests count heap allocations, so they can check how you solved a task.
#[global_allocator]
static ALLOC: practice::alloc::Counting = practice::alloc::Counting;
