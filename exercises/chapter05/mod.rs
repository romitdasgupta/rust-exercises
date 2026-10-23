#![allow(dead_code, unused_variables, unused_mut)]

mod c01_code_review;
mod l01_basics;
mod l02_identity;
mod l03_optional_fat;
mod l04_lifetime_flow;
mod l05_borrowed_structs;
mod l06_distinct_lifetimes;
mod l07_exclusive;
mod l08_disjoint;
mod l09_capstone;
mod r01_review;

// Lets tests count heap allocations, so they can check how you solved a task.
#[global_allocator]
static ALLOC: practice::alloc::Counting = practice::alloc::Counting;
