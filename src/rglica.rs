use crate::New;
use std::ops::{Deref, DerefMut};

pub struct Rglica<T: ?Sized> {
    ptr: *mut T,
}

impl<T: ?Sized> Rglica<T> {
    pub fn from_box(bx: &mut Box<T>) -> Self {
        Self { ptr: &mut **bx }
    }

    pub fn is_null(&self) -> bool {
        self.ptr.is_null()
    }

    pub fn is_ok(&self) -> bool {
        !self.is_null()
    }
}

impl<T: ?Sized> Deref for Rglica<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { self.ptr.as_ref().unwrap() }
    }
}

impl<T: ?Sized> DerefMut for Rglica<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.ptr.as_mut().unwrap() }
    }
}

impl<T: ?Sized> New for Rglica<T> {
    fn new() -> Self {
        Self {
            ptr: unsafe { std::mem::zeroed() },
        }
    }
}
