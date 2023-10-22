#![allow(clippy::mismatched_target_os)]
#![feature(const_trait_impl)]
#![feature(const_fn_floating_point_arithmetic)]

#[macro_use]
extern crate log;
extern crate core;

pub mod animation;
mod apply;
pub mod data;
pub mod data_manager;
pub mod debug;
pub mod file;
mod logger;
pub mod math;
mod misc;
pub mod passed;
pub mod platform;
mod random;
pub mod regex;
pub mod stored;

pub use animation::Animation;
pub use apply::*;
pub use debug::*;
pub use logger::*;
pub use math::{mm_to_inch, IntoF32};
pub use misc::*;
pub use random::*;
pub use stored::Stored;
