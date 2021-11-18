use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
};

pub struct Property<T> {
    data: T,
}

impl<T> Deref for Property<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> DerefMut for Property<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl<T> From<T> for Property<T> {
    fn from(data: T) -> Self {
        Self { data }
    }
}

impl<T: Debug> Debug for Property<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.data.fmt(f)
    }
}

impl<T: Default> Default for Property<T> {
    fn default() -> Self {
        Self { data: T::default() }
    }
}
