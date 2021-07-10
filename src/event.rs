use crate::HasNew;
use std::fmt::{Debug, Formatter};

pub struct Event<T> {
    subscribers: Vec<Box<dyn Fn(&T)>>,
}

impl<T> Event<T> {
    pub fn subscribe(&mut self, action: impl Fn(&T) + 'static) {
        self.subscribers.push(Box::new(action))
    }

    pub fn trigger(&self, value: &T) {
        for sub in &self.subscribers {
            sub(value)
        }
    }
}

impl<T> HasNew for Event<T> {
    fn new() -> Self {
        Event {
            subscribers: vec![],
        }
    }
}

impl<T> Debug for Event<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Event<{}> subscribers: {}",
            std::any::type_name::<T>(),
            self.subscribers.len()
        )
    }
}
