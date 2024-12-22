#![allow(unexpected_cfgs)]
#![feature(const_trait_impl)]

pub mod animation;
mod apply;
pub mod data;
pub mod debug;
pub mod elapsed;
pub mod every;
pub mod file;
mod logger;
mod misc;
pub mod passed;
pub mod platform;
mod random;
pub mod regex;
pub mod stored;

pub use animation::Animation;
pub use apply::*;
pub use debug::*;
pub use misc::*;
pub use random::*;
pub use stored::Stored;
