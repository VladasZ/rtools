#![crate_name = "tools"]
#![crate_type = "lib"]

#[macro_use]
extern crate log;

pub mod array_view;
pub mod as_any;
pub mod boxed;
pub mod event;
pub mod file;
pub mod log_macro;
pub mod math;
pub mod persistent;
pub mod platform;
mod property;
pub mod regex;
pub mod rglica;
pub mod run;

pub use boxed::Boxed;
pub use event::Event;
pub use math::{mm_to_inch, IntoF32};
pub use persistent::Persistent;
pub use property::Property;
pub use rglica::{Rglica, ToRglica};

pub trait Delete<T> {
    fn delete(&mut self, val: &T);
}

impl<T: PartialEq> Delete<T> for Vec<T> {
    fn delete(&mut self, val: &T) {
        if let Some(pos) = self.iter().position(|a| a == val) {
            self.remove(pos);
        } else {
            dbg!("Nothing to delete");
        }
    }
}

pub trait Address {
    fn address(&self) -> u64;
}

impl<T: ?Sized> Address for &T {
    fn address(&self) -> u64 {
        *self as *const T as *const () as u64
    }
}
