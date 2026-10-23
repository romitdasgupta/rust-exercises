#![allow(dead_code, unused_variables, unused_mut)]

mod c01_code_review;
mod l01_named;
mod l02_kinds_layout;
mod l03_methods;
mod l04_smart_self;
mod l05_generics;
mod l06_lifetime_const;
mod l07_derive;
mod l08_interior;
mod l09_capstone;
mod r01_review;

// Lets tests count heap allocations, so they can check how you solved a task.
#[global_allocator]
static ALLOC: practice::alloc::Counting = practice::alloc::Counting;
