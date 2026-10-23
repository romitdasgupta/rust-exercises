#![allow(dead_code, unused_variables, unused_mut)]

mod c01_code_review;
mod l01_panic;
mod l02_catching;
mod l03_propagation;
mod l04_methods_aliases;
mod l05_multiple_errors;
mod l06_custom;
mod l07_invariants_ignoring;
mod l08_capstone;
mod r01_review;

// Lets tests count heap allocations, so they can check how you solved a task.
#[global_allocator]
static ALLOC: practice::alloc::Counting = practice::alloc::Counting;
