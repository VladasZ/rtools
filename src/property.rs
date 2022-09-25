use std::fmt::Debug;

use crate::Event;

#[derive(Default)]
pub struct Property<T> {
    data:       T,
    pub on_set: Event<T>,
    pub on_get: Event,
}

impl<T: 'static + Clone> Property<T> {
    pub fn set(&mut self, value: T) {
        self.data = value.clone();
        self.on_set.trigger(value);
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
