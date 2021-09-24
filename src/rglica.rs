use std::{
    fmt::{Debug, Formatter},
    ops::{Deref, DerefMut},
    ptr::NonNull,
};

use crate::New;

pub struct Rglica<T: ?Sized> {
    ptr: Option<NonNull<T>>,
}

impl<T: ?Sized> Clone for Rglica<T> {
    fn clone(&self) -> Rglica<T> { Self { ptr: self.ptr } }
}

impl<T: ?Sized> Rglica<T> {
    pub fn from_ref(rf: &T) -> Rglica<T> {
        Self {
            ptr: NonNull::new(rf as *const T as *mut T).unwrap().into(),
        }
    }

    pub fn is_null(&self) -> bool { self.ptr.is_none() }

    pub fn is_ok(&self) -> bool { self.ptr.is_some() }

    pub fn invalidate(&mut self) { self.ptr = None }
}

impl<T: ?Sized> Deref for Rglica<T> {
    type Target = T;
    fn deref(&self) -> &T { unsafe { self.ptr.unwrap().as_ref() } }
}

impl<T: ?Sized> DerefMut for Rglica<T> {
    fn deref_mut(&mut self) -> &mut T { unsafe { self.ptr.unwrap().as_mut() } }
}

impl<T: ?Sized> New for Rglica<T> {
    fn new() -> Rglica<T> { Self { ptr: None } }
}

pub trait ToRglica<T: ?Sized> {
    fn to_rglica(&self) -> Rglica<T>;
}

impl<T: ?Sized> ToRglica<T> for Box<T> {
    fn to_rglica(&self) -> Rglica<T> {
        Rglica {
            ptr: NonNull::new(self.as_ref() as *const T as *mut T)
                .unwrap()
                .into(),
        }
    }
}

impl<T: Debug> Debug for Rglica<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { self.deref().fmt(f) }
}
