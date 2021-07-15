//#![allow(dead_code)]
#![crate_name = "tools"]
#![crate_type = "lib"]

pub mod array_view;
pub mod as_any;
pub mod event;
pub mod has_new;
pub mod log;
pub mod platform;
pub mod property_wrapper;
pub mod refs;
pub mod regex;
pub mod weak_self;

pub use as_any::AsAny;
pub use event::Event;
pub use has_new::HasNew;
pub use property_wrapper::PropertyWrapper;
