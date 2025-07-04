#![feature(portable_simd)]
#![feature(lock_value_accessors)]
#![feature(thread_sleep_until)]
#![allow(async_fn_in_trait)]
pub mod component;
pub mod main_loop;
pub mod object;
pub mod render;
pub mod scene;
pub mod sound;
pub mod transform;
pub mod type_render;
pub mod utils;
pub mod world;
