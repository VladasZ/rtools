use std::{
    fmt::{Debug, Display},
    ops::{Deref, DerefMut},
};

use crate::Event;

pub struct Property<T> {
    data:       T,
    pub on_set: Event,
}

impl<T: Copy> Property<T> {
    pub fn copy(&self) -> T {
        self.data
    }

    pub fn set(&mut self, value: T) {
        self.data = value;
        self.on_set.trigger(());
    }
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
        Self {
            data,
            on_set: Default::default(),
        }
    }
}

impl<T: Debug> Debug for Property<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.data.fmt(f)
    }
}

impl<T: Default> Default for Property<T> {
    fn default() -> Self {
        Self {
            data:   T::default(),
            on_set: Default::default(),
        }
    }
}

impl<T: Display> Display for Property<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.data.fmt(f)
    }
}
