use crate::New;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub type Shared<T> = Rc<RefCell<T>>;
pub type MutWeak<T> = Weak<RefCell<T>>;

pub fn make_shared<T>(val: T) -> Shared<T> {
    Rc::new(RefCell::new(val))
}

pub fn new_shared<T: New>() -> Shared<T> {
    make_shared(T::new())
}
