use std::{
    alloc::{dealloc, Layout},
    marker::Unsize,
    ops::{CoerceUnsized, Deref, DerefMut},
    ptr::NonNull,
};

use crate::{address::Address, RefCounters, ToWeak, Weak};

pub struct Strong<T: ?Sized> {
    address: usize,
    ptr:     *mut T,
}

impl<T: Sized + 'static> Strong<T> {
    pub fn new(val: T) -> Self {
        let val = Box::new(val);
        let address = val.deref().address();
        let ptr = Box::leak(val) as *mut T;

        RefCounters::add_strong(address, move || unsafe {
            dealloc(ptr as *mut u8, Layout::new::<T>());
        });

        Self { address, ptr }
    }
}

impl<T: ?Sized> Deref for Strong<T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { self.ptr.as_ref().unwrap() }
    }
}

impl<T: ?Sized> DerefMut for Strong<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.ptr.as_mut().unwrap() }
    }
}

impl<T: ?Sized> Clone for Strong<T> {
    fn clone(&self) -> Self {
        RefCounters::increase_strong(self.address);
        Self {
            address: self.address,
            ptr:     self.ptr,
        }
    }
}

impl<T: ?Sized> Drop for Strong<T> {
    fn drop(&mut self) {
        RefCounters::decrease_strong(self.address);
        if RefCounters::strong_count(self.address) == 0 {
            RefCounters::remove(self.address);
        }
    }
}

impl<T: ?Sized> ToWeak<T> for Strong<T> {
    fn weak(&self) -> Weak<T> {
        Weak {
            address: self.address,
            ptr:     NonNull::new(self.ptr).into(),
        }
    }
}

impl<T: Default + Sized + 'static> Default for Strong<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<T, U> CoerceUnsized<Strong<U>> for Strong<T>
where
    T: Unsize<U> + ?Sized,
    U: ?Sized,
{
}
