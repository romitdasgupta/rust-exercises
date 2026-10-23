#![allow(dead_code, unused_variables, unused_mut)]

mod c01_code_review;
mod l01_blocks;
mod l02_branching;
mod l03_let_else;
mod l04_loops;
mod l05_labels;
mod l06_const_unsafe;
mod l07_operators;
mod l08_casts_closures;
mod l09_capstone;
mod r01_review;

// Lets tests count heap allocations, so they can check how you solved a task.
#[global_allocator]
static ALLOC: practice::alloc::Counting = practice::alloc::Counting;
