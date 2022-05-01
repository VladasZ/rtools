#![allow(incomplete_features)]
#![allow(clippy::mismatched_target_os)]
#![feature(specialization)]
#![feature(const_trait_impl)]
#![feature(const_fn_floating_point_arithmetic)]
#![feature(const_for)]

#[macro_use]
extern crate log;
extern crate core;

pub mod address;
pub mod animation;
pub mod array_view;
pub mod boxed;
pub mod bytes;
pub mod data;
pub mod data_manager;
pub mod event;
pub mod file;
pub mod math;
pub mod misc;
pub mod passed;
pub mod paths;
pub mod persistent;
pub mod platform;
mod property;
pub mod regex;
pub mod rglica;
pub mod run;
pub mod static_storage;
pub mod test;
mod time;
pub mod unwrap;

pub use animation::Animation;
pub use boxed::Boxed;
pub use event::Event;
pub use math::{mm_to_inch, IntoF32};
pub use misc::backtrace;
pub use persistent::Persistent;
pub use property::Property;
pub use rglica::{Rglica, ToRglica};
pub use static_storage::StaticStorage;
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
            dbg!("Nothing to remove");
        }
    }
}
