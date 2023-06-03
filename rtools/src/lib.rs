#![allow(clippy::mismatched_target_os)]
#![feature(const_trait_impl)]
#![feature(const_fn_floating_point_arithmetic)]

#[macro_use]
extern crate log;
extern crate core;

pub mod animation;
mod apply;
pub mod bytes;
pub mod data;
pub mod data_manager;
pub mod debug;
pub mod file;
mod file_log;
mod logger;
pub mod math;
mod misc;
pub mod passed;
pub mod paths;
pub mod platform;
mod random;
pub mod regex;
pub mod run;
mod selectable;
mod static_default;
mod static_init;
pub mod stored;
pub mod test;
mod time;
mod unwrap;
pub mod vec;

pub use animation::Animation;
pub use apply::*;
pub use debug::*;
pub use file_log::FileLog;
pub use logger::*;
pub use math::{mm_to_inch, IntoF32};
pub use misc::*;
pub use random::*;
pub use selectable::Selectable;
pub use stored::Stored;
pub use time::Time;
pub use unwrap::Unwrap;
pub use vec::*;

pub trait Remove<T> {
    fn remove(&mut self, val: &T);
}

impl<T: PartialEq> Remove<T> for Vec<T> {
    fn remove(&mut self, val: &T) {
        if let Some(pos) = self.iter().position(|a| a == val) {
            self.remove(pos);
        } else {
            info!("Nothing to remove");
        }
    }
}
