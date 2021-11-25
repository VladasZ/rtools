use std::fmt::{Debug, Display};

use crate::Event;

pub struct Property<T> {
    data:       T,
    pub on_set: Event,
    pub on_get: Event,
}

impl<T> Property<T> {
    pub fn set(&mut self, value: T) {
        self.data = value;
        self.on_set.trigger(());
    }

    pub fn get(&mut self) -> &mut T {
        self.on_get.trigger(());
        &mut self.data
    }
}

impl<T: Copy> Property<T> {
    pub fn copy(&self) -> T {
        self.data
    }
}

impl<T> From<T> for Property<T> {
    fn from(data: T) -> Self {
        Self {
            data,
            on_set: Default::default(),
            on_get: Default::default(),
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
            on_get: Default::default(),
        }
    }
}

impl<T: Display> Display for Property<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.data.fmt(f)
    }
}
