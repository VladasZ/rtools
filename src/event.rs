use std::fmt::{Debug, Formatter};

pub struct Event<T = ()> {
    subscriber: Option<Box<dyn FnMut(T) + 'static>>,
}

impl<T> Event<T> {
    pub fn subscribe(&mut self, action: impl FnMut(T) + 'static) {
        debug_assert!(self.subscriber.is_none(), "Event already has a subscriber");
        self.subscriber = Some(Box::new(action))
    }

    pub fn trigger(&mut self, value: T) {
        debug_assert!(
            self.subscriber.is_some(),
            "Event triggered without subscriber"
        );
        let sub = unsafe { self.subscriber.as_mut().unwrap_unchecked() };
        sub(value)
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
