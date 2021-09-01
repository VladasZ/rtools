use crate::refs::Shared;
use crate::New;
use std::ops::{Deref, DerefMut};
use std::ptr::NonNull;

pub struct Rglica<T: ?Sized> {
    ptr: Option<NonNull<T>>,
}

impl<T: ?Sized> Rglica<T> {
    pub fn from_ptr(ptr: *mut T) -> Self {
        Self {
            ptr: NonNull::new(ptr).unwrap().into(),
        }
    }

    pub fn from_ref(rf: &mut T) -> Self {
        Self {
            ptr: NonNull::new(rf).unwrap().into(),
        }
    }

    pub fn from_shared(sh: Shared<T>) -> Self {
        Self {
            ptr: NonNull::new(sh.borrow_mut().deref_mut()).unwrap().into(),
        }
    }

    pub fn from_box(bx: &mut Box<T>) -> Self {
        Self {
            ptr: NonNull::new(bx.as_mut()).unwrap().into(),
        }
    }

    pub fn is_null(&self) -> bool {
        self.ptr.is_none()
    }

    pub fn is_ok(&self) -> bool {
        self.ptr.is_some()
    }
}

impl<T: ?Sized> Deref for Rglica<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { self.ptr.unwrap().as_ref() }
    }
}

impl<T: ?Sized> DerefMut for Rglica<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.ptr.unwrap().as_mut() }
    }
}

impl<T: ?Sized> New for Rglica<T> {
    fn new() -> Self {
        Self { ptr: None }
    }
}
