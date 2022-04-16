use std::{
    cell::RefCell,
    fmt::{Debug, Formatter},
    ops::DerefMut,
};

use crate::{ToRglica, Unwrap};

pub struct Event<T = ()> {
    subscriber: RefCell<Unwrap<dyn FnMut(T) + 'static>>,
}

impl<T> Event<T> {
    pub fn set<Obj: 'static>(&self, obj: &Obj, mut action: impl FnMut(T, &mut Obj) + 'static) {
        debug_assert!(
            self.subscriber.borrow().is_null(),
            "Event already has a subscriber"
        );
        let mut rglica = obj.to_rglica();
        self.subscriber
            .replace(Unwrap::from_box(Box::new(move |value| {
                action(value, rglica.deref_mut());
            })));
    }

    pub fn trigger(&self, value: T) {
        if self.subscriber.borrow().is_null() {
            dbg!("event triggered without subscriber");
            dbg!(std::any::type_name::<T>());
            return;
        }
        (self.subscriber.borrow_mut())(value)
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
