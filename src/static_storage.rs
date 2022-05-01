use std::{any::Any, collections::HashMap, ptr::null_mut};

type Storage = HashMap<&'static str, Box<dyn Any>>;

static mut STORAGE: *mut Storage = null_mut();

pub trait StaticStorage<T: 'static>: Sized {
    fn get() -> &'static T {
        let bx = match Self::storage().get(Self::type_name()) {
            Some(val) => val,
            None => {
                Self::set(Self::default());
                Self::storage().get(Self::type_name()).unwrap()
            }
        };
        bx.downcast_ref().unwrap()
    }

    fn set(value: T) {
        Self::storage().insert(Self::type_name(), Box::new(value));
    }

    fn default() -> T;
}

trait StaticStorageInternal<T: 'static, S: StaticStorage<T>> {
    fn type_name() -> &'static str;
    fn storage() -> &'static mut Storage;
}

impl<T: 'static, S: StaticStorage<T>> StaticStorageInternal<T, S> for S {
    fn type_name() -> &'static str {
        std::any::type_name::<Self>()
    }

    fn storage() -> &'static mut Storage {
        unsafe {
            if STORAGE.is_null() {
                STORAGE = Box::into_raw(Box::new(Default::default()));
            }
            STORAGE.as_mut().unwrap()
        }
    }
}

#[macro_export]
macro_rules! static_storage {
    ($name:ident, $type:ident) => {
        pub struct $name;
        impl StaticStorage<$type> for $name {
            fn default() -> $type {
                Default::default()
            }
        }
    };
    ($name:ident, $type:ident, $default:expr) => {
        pub struct $name;
        impl StaticStorage<$type> for $name {
            fn default() -> $type {
                $default
            }
        }
    };
}
