use std::fmt::{Debug, Formatter};

pub struct Event<T = ()> {
    subscriber:         Option<Box<dyn FnMut(T) + 'static>>,
    pub panic_if_empty: bool,
}

impl<T> Event<T> {
    pub fn subscribe(&mut self, action: impl FnMut(T) + 'static) {
        debug_assert!(self.subscriber.is_none(), "Event already has a subscriber");
        self.subscriber = Some(Box::new(action))
    }

    pub fn trigger(&mut self, value: T) {
        if self.subscriber.is_none() {
            if self.panic_if_empty {
                error!("Event triggered without subscriber");
                panic!("Event triggered without subscriber");
            } else {
                return;
            }
        }

        let sub = unsafe { self.subscriber.as_mut().unwrap_unchecked() };
        sub(value)
    }
}

impl<T> Default for Event<T> {
    fn default() -> Self {
        Self {
            subscriber:     Default::default(),
            panic_if_empty: true,
        }
    }
}

impl<T> Debug for Event<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Event<{}>", std::any::type_name::<T>(),)
    }
}
