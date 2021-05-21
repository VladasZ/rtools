#![warn(dead_code)]

#![crate_name = "tools"]
#![crate_type = "lib"]

pub mod log;
pub mod refs;
pub mod regex;
pub mod as_any;
pub mod has_new;
pub mod platform;
pub mod weak_self;
pub mod array_view;

pub use has_new::HasNew;
pub use as_any::AsAny;
