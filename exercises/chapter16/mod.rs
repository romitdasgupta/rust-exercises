//! Chapter 16: Collections. Original guided practice.
//! Run one lesson: cargo test --test chapter16 l01_
#![allow(dead_code, unused_variables, unused_mut, unused_imports)]

mod c01_code_review;
mod l01_vectors;
mod l02_validity;
mod l03_sort_search;
mod l04_deques;
mod l05_heaps;
mod l06_maps;
mod l07_sets;
mod l08_hashing;
mod l09_capstone;
mod r01_review;

// Lets tests count heap allocations, so they can check how you solved a task.
#[global_allocator]
static ALLOC: practice::alloc::Counting = practice::alloc::Counting;
