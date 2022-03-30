use std::{
    fmt::{Debug, Formatter},
    ops::{Deref, DerefMut},
};

use crate::{misc::backtrace, Rglica, ToRglica};

pub struct Unwrap<T> {
    value: Option<Box<T>>,
}

impl<T> Unwrap<T> {
    pub fn is_ok(&self) -> bool {
        self.value.is_some()
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

impl<T> Default for Unwrap<T> {
    fn default() -> Self {
        Self { value: None }
    }
}

impl<T> From<T> for Unwrap<T> {
    fn from(value: T) -> Self {
        Self {
            value: Box::new(value).into(),
        }
    }
}

impl<T: Debug> Debug for Unwrap<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.deref().fmt(f)
    }
}

impl<T: ToString> ToString for Unwrap<T> {
    fn to_string(&self) -> String {
        self.deref().to_string()
    }
}

impl<T> ToRglica<T> for Unwrap<T> {
    fn to_rglica(&self) -> Rglica<T> {
        self.deref().to_rglica()
    }
}
