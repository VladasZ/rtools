use std::fmt::{Debug, Formatter};

use crate::Unwrap;

pub struct Event<T = ()> {
    subscriber: Unwrap<dyn FnMut(T) + 'static>,
}

impl<T> Event<T> {
    pub fn subscribe(&mut self, action: impl FnMut(T) + 'static) {
        debug_assert!(self.subscriber.is_null(), "Event already has a subscriber");
        self.subscriber = Unwrap::from_box(Box::new(action));
    }

    pub fn trigger(&mut self, value: T) {
        (self.subscriber)(value)
    }
}

impl<T> Default for Event<T> {
    fn default() -> Self {
        Self {
            subscriber: Default::default(),
        }
    }
}

impl<T> Debug for Event<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Event<{}>", std::any::type_name::<T>(),)
    }
}
