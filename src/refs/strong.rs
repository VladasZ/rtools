use std::{
    alloc::{dealloc, Layout},
    collections::HashMap,
    ops::{Deref, DerefMut},
};

use crate::{address::Address, static_default, RefCounters};

pub type Strong<T> = Box<T>;

pub struct Streng1<T: ?Sized> {
    address: u64,
    ptr:     *mut T,
}

impl<T: Sized + 'static> Streng1<T> {
    fn new(val: T) -> Self {
        let val = Box::new(val);
        let address = val.deref().address();
        let ptr = Box::leak(val) as *mut T;

        RefCounters::add_strong(address, move || unsafe {
            dealloc(ptr as *mut u8, Layout::new::<T>());
        });

        Self { address, ptr }
    }
}

impl<T: ?Sized> Deref for Streng1<T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { self.ptr.as_ref().unwrap() }
    }
}

impl<T: ?Sized> DerefMut for Streng1<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.ptr.as_mut().unwrap() }
    }
}

impl<T: ?Sized> Clone for Streng1<T> {
    fn clone(&self) -> Self {
        RefCounters::increase_strong(self.address);
        Self {
            address: self.address,
            ptr:     self.ptr,
        }
    }
}

impl<T: ?Sized> Drop for Streng1<T> {
    fn drop(&mut self) {
        RefCounters::decrease_strong(self.address);
        if RefCounters::strong_count(self.address) == 0 {
            RefCounters::remove(self.address);
        }
    }
}
