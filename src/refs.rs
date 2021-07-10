use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub type Shared<T> = Rc<RefCell<T>>;
pub type MutWeak<T> = Weak<RefCell<T>>;
pub type DynWeak<T> = Option<MutWeak<T>>;

pub fn make_shared<T>(val: T) -> Shared<T> {
    Rc::new(RefCell::new(val))
}

pub fn make_box<T>(val: T) -> Box<T> {
    Box::new(val)
}
