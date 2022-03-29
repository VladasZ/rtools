#![allow(incomplete_features)]
#![feature(specialization)]

#[macro_use]
extern crate log;

pub mod address;
pub mod array_view;
pub mod as_any;
pub mod boxed;
pub mod bytes;
pub mod data;
pub mod event;
pub mod file;
pub mod math;
pub mod paths;
pub mod persistent;
pub mod platform;
mod property;
pub mod regex;
pub mod rglica;
pub mod run;
pub mod test;
pub mod unwrap;

pub use boxed::Boxed;
pub use event::Event;
pub use math::{mm_to_inch, IntoF32};
pub use persistent::Persistent;
pub use property::Property;
pub use rglica::{Rglica, ToRglica};
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
