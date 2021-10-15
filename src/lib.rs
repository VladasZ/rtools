#![crate_name = "tools"]
#![crate_type = "lib"]
#![feature(option_result_unwrap_unchecked)]
#![feature(default_free_fn)]

pub mod array_view;
pub mod as_any;
pub mod boxed;
pub mod event;
pub mod log;
pub mod math;
pub mod platform;
pub mod property_wrapper;
pub mod regex;
pub mod rglica;

pub use boxed::Boxed;
pub use event::Event;
pub use property_wrapper::PropertyWrapper;
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
