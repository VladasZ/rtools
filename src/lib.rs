#![crate_name = "tools"]
#![crate_type = "lib"]
#![feature(option_result_unwrap_unchecked)]

pub mod array_view;
pub mod event;
pub mod has_new;
pub mod log;
pub mod own;
pub mod platform;
pub mod property_wrapper;
pub mod refs;
pub mod regex;
pub mod rglica;
pub mod weak_self;

pub use event::Event;
pub use has_new::{new, Boxed, New};
pub use own::Own;
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
    fn address(&self) -> u64 { *self as *const T as *const () as u64 }
}
