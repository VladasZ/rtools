use std::{
    fmt::{Debug, Formatter},
    ops::{Deref, DerefMut},
};

use crate::{misc::backtrace, Rglica, ToRglica};

pub struct UnwrapBox<T: ?Sized> {
    value: Option<Box<T>>,
}

impl<T: ?Sized> UnwrapBox<T> {
    pub fn from_box(bx: Box<T>) -> Self {
        Self { value: bx.into() }
    }

    pub fn is_null(&self) -> bool {
        self.value.is_none()
    }
}

impl<T: ?Sized> Deref for UnwrapBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        match self.value.as_ref() {
            Some(rf) => rf,
            None => {
                backtrace();
                panic!("Invalid UnwrapBox<{}>", std::any::type_name::<T>());
            }
        }
    }
}

impl<T: ?Sized> DerefMut for UnwrapBox<T> {
    fn deref_mut(&mut self) -> &mut T {
        match self.value.as_mut() {
            Some(rf) => rf,
            None => {
                backtrace();
                panic!("Invalid UnwrapBox<{}>", std::any::type_name::<T>());
            }
        }
    }
}

impl<T: ?Sized> Default for UnwrapBox<T> {
    fn default() -> Self {
        Self { value: None }
    }
}

impl<T> From<T> for UnwrapBox<T> {
    fn from(value: T) -> Self {
        Self {
            value: Box::new(value).into(),
        }
    }
}

impl<T: ?Sized + Debug> Debug for UnwrapBox<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.deref().fmt(f)
    }
}

impl<T: ?Sized + ToString> ToString for UnwrapBox<T> {
    fn to_string(&self) -> String {
        self.deref().to_string()
    }
}

impl<T: ?Sized> ToRglica<T> for UnwrapBox<T> {
    fn to_rglica(&self) -> Rglica<T> {
        self.deref().to_rglica()
    }
}
