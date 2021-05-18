#![warn(dead_code)]

#![crate_name = "tools"]
#![crate_type = "lib"]

pub mod log;
pub mod new;
pub mod refs;
pub mod regex;
pub mod as_any;
pub mod platform;
pub mod weak_self;
pub mod array_view;

pub use new::New;
pub use as_any::AsAny;
