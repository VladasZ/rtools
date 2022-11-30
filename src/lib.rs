#![allow(incomplete_features)]
#![allow(clippy::mismatched_target_os)]
#![feature(specialization)]
#![feature(const_trait_impl)]
#![feature(const_fn_floating_point_arithmetic)]
#![feature(const_for)]
#![feature(coerce_unsized)]
#![feature(unsize)]
#![feature(ptr_metadata)]
#![feature(const_default_impls)]

#[macro_use]
extern crate log;
extern crate core;

pub mod animation;
mod apply;
pub mod array_view;
pub mod bytes;
pub mod data;
pub mod data_manager;
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

pub use animation::Animation;
pub use apply::*;
pub use file_log::FileLog;
pub use logger::init_log;
pub use math::{mm_to_inch, IntoF32};
pub use misc::{backtrace, *};
pub use random::Random;
pub use selectable::Selectable;
pub use stored::Stored;
pub use time::Time;
pub use unwrap::Unwrap;

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
