use std::ops::Deref;

use crate::bytes::data_pointer;

pub trait Address {
    fn address(&self) -> usize;
}

impl<T: ?Sized> Address for Box<T> {
    fn address(&self) -> usize {
        data_pointer(self.deref())
    }
}

impl<T: ?Sized> Address for &T {
    fn address(&self) -> usize {
        data_pointer(*self)
    }
}

impl<T: ?Sized> Address for &mut T {
    fn address(&self) -> usize {
        data_pointer(*self as *const T)
    }
}
