//#![allow(dead_code)]
#![crate_name = "tools"]
#![crate_type = "lib"]

pub mod array_view;
pub mod as_any;
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

pub use as_any::AsAny;
pub use event::Event;
pub use has_new::new;
pub use has_new::New;
pub use own::Own;
pub use property_wrapper::PropertyWrapper;
pub use rglica::Rglica;
