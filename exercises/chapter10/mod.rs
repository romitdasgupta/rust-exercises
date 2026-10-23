#![allow(dead_code, unused_variables, unused_mut)]

mod c01_code_review;
mod l01_variants;
mod l02_recursive;
mod l03_generic;
mod l04_structural;
mod l05_reference_patterns;
mod l06_slice_patterns;
mod l07_guards_or_at;
mod l08_contexts_public;
mod l09_capstone;
mod r01_review;

// Lets tests count heap allocations, so they can check how you solved a task.
#[global_allocator]
static ALLOC: practice::alloc::Counting = practice::alloc::Counting;
