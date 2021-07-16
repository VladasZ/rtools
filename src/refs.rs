use std::cell::RefCell;
use std::rc::{Rc, Weak};
use crate::HasNew;

pub type Shared<T> = Rc<RefCell<T>>;
pub type MutWeak<T> = Weak<RefCell<T>>;
pub type DynWeak<T> = Option<MutWeak<T>>;

pub fn make_shared<T>(val: T) -> Shared<T> {
    Rc::new(RefCell::new(val))
}

pub fn new_shared<T: HasNew>() -> Shared<T> {
    make_shared(T::new())
}

pub fn make_box<T>(val: T) -> Box<T> {
    Box::new(val)
}
