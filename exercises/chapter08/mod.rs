#![allow(dead_code, unused_variables, unused_mut)]

mod c01_code_review;
mod l01_crates_profiles;
mod l02_modules;
mod l03_paths_exports;
mod l04_const_static;
mod l05_attributes;
mod l06_tests_docs;
mod l07_dependencies;
mod l08_capstone;
mod r01_review;

// Lets tests count heap allocations, so they can check how you solved a task.
#[global_allocator]
static ALLOC: practice::alloc::Counting = practice::alloc::Counting;
