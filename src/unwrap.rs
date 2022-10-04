use std::ops::{Deref, DerefMut};

use crate::backtrace;

pub struct Unwrap<T> {
    value: Option<T>,
}

impl<T> Unwrap<T> {
    pub fn new(val: T) -> Self {
        Self { value: Some(val) }
    }
}

impl<T> Deref for Unwrap<T> {
    type Target = T;
    fn deref(&self) -> &T {
        match self.value.as_ref() {
            Some(rf) => rf,
            None => {
                backtrace();
                panic!("Invalid Unwrap<{}>", std::any::type_name::<T>());
            }
        }
    }
}

impl<T> DerefMut for Unwrap<T> {
    fn deref_mut(&mut self) -> &mut T {
        match self.value.as_mut() {
            Some(rf) => rf,
            None => {
                backtrace();
                panic!("Invalid Unwrap<{}>", std::any::type_name::<T>());
            }
        }
    }
}

impl<T> From<T> for Unwrap<T> {
    fn from(val: T) -> Self {
        Self { value: val.into() }
    }
}

impl<T> Default for Unwrap<T> {
    fn default() -> Self {
        Self { value: None }
    }
}
