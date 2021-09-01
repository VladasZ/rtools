use crate::New;
use std::fmt::{Debug, Formatter};

pub struct Event<T = ()> {
    subscribers: Vec<Box<dyn FnMut(&T)>>,
}

impl<T> Event<T> {
    pub fn subscribe(&mut self, action: impl FnMut(&T) + 'static) {
        self.subscribers.push(Box::new(action))
    }

    pub fn trigger(&mut self, value: &T) {
        for sub in &mut self.subscribers {
            sub(value)
        }
    }
}

impl<T> New for Event<T> {
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
