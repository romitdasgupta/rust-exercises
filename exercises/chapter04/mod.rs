#![allow(dead_code, unused_variables, unused_mut)]

mod c01_code_review;
mod l01_scope;
mod l02_move_paths;
mod l03_control_flow;
mod l04_indexed;
mod l05_replace;
mod l06_copy_clone;
mod l07_shared;
mod l08_capstone;
mod r01_review;

// Lets tests count heap allocations, so they can check how you solved a task.
#[global_allocator]
static ALLOC: practice::alloc::Counting = practice::alloc::Counting;
