#![feature(allocator_api)]
#![allow(non_upper_case_globals)]

extern crate self as backstep;

pub mod component;
pub mod ecs;
pub mod entity;
pub mod frame;
pub mod rollback;
pub mod scheduler;
pub mod storage;
pub mod system;
pub mod tick;
pub mod view;
pub mod world;
pub mod arena;

// Re-export macros
pub use backstep_macros::system;
