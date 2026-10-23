//! Chapter 15: Iterators. Original guided practice.
//! Run one lesson: cargo test --test chapter15 l01_
#![allow(dead_code, unused_variables, unused_mut, unused_imports)]

mod c01_code_review;
mod l01_sources;
mod l02_generated;
mod l03_mapping;
mod l04_boundaries;
mod l05_composition;
mod l06_aggregation;
mod l07_search;
mod l08_folds;
mod l09_collecting;
mod l10_custom;
mod l11_capstone;
mod r01_review;

// Lets tests count heap allocations, so they can check how you solved a task.
#[global_allocator]
static ALLOC: practice::alloc::Counting = practice::alloc::Counting;
