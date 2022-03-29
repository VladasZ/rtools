use std::{
    fmt::{Debug, Formatter},
    ops::{Deref, DerefMut},
};

use crate::{Rglica, ToRglica};

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
        self.value.as_ref().unwrap()
    }
}

impl<T> DerefMut for Unwrap<T> {
    fn deref_mut(&mut self) -> &mut T {
        self.value.as_mut().unwrap()
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
        self.value.as_ref().unwrap().fmt(f)
    }
}

impl<T: ToString> ToString for Unwrap<T> {
    fn to_string(&self) -> String {
        self.value.as_ref().unwrap().to_string()
    }
}

impl<T> ToRglica<T> for Unwrap<T> {
    fn to_rglica(&self) -> Rglica<T> {
        self.value.as_ref().unwrap().to_rglica()
    }
}
