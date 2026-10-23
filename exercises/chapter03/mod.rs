//! Chapter 3 exercises. Run one lesson with `cargo test --test chapter03 l01_`.
//! Implement the TODO bodies, keep the tests, and answer each lesson's prompts.

// Unfinished TODO bodies intentionally leave their parameters unused.
#![allow(unused_variables, unused_mut)]

mod c01_code_review;
mod l01_inference;
mod l02_integers;
mod l03_overflow;
mod l04_floats;
mod l05_bool_tuples;
mod l06_pointers;
mod l07_arrays;
mod l08_vectors;
mod l09_slices;
mod l10_unicode;
mod l11_strings;
mod l12_boundary_types;
mod l13_capstone;

// Lets tests count heap allocations, so they can check how you solved a task.
#[global_allocator]
static ALLOC: practice::alloc::Counting = practice::alloc::Counting;
