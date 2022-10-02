use std::{
    ops::{Deref, DerefMut},
    ptr::NonNull,
};

use crate::{address::Address, backtrace, RefCounters};

pub struct Weak<T: ?Sized> {
    pub(crate) address: usize,
    pub(crate) ptr:     Option<NonNull<T>>,
}

unsafe impl<T: ?Sized> Send for Weak<T> {}
unsafe impl<T: ?Sized> Sync for Weak<T> {}

impl<T: ?Sized> Copy for Weak<T> {}

impl<T: ?Sized> Clone for Weak<T> {
    fn clone(&self) -> Self {
        Self {
            address: self.address,
            ptr:     self.ptr,
        }
    }
}

impl<T: ?Sized> Weak<T> {
    pub fn from_ref(rf: &T) -> Self {
        let address = rf.address();
        assert!(
            RefCounters::exists(address),
            "Trying to get weak pointer for object which is not managed by reference counter."
        );
        let ptr = NonNull::new(rf as *const T as *mut T);
        assert!(ptr.is_some(), "Failed to get ptr from ref");
        Self { address, ptr }
    }

    pub fn addr(&self) -> usize {
        self.address
    }

    pub fn is_null(&self) -> bool {
        !self.is_ok()
    }

    pub fn is_ok(&self) -> bool {
        RefCounters::exists(self.address)
    }

    pub fn get(&mut self) -> Option<&mut T> {
        if self.is_ok() {
            self.deref_mut().into()
        } else {
            None
        }
    }

    fn check(&self) {
        if self.ptr.is_none() {
            error!(
                "Defererencing never initialized weak pointer: {}",
                std::any::type_name::<T>()
            );
            backtrace();
            panic!(
                "Defererencing never initialized weak pointer: {}",
                std::any::type_name::<T>()
            );
        }

        if !RefCounters::exists(self.address) {
            error!(
                "Defererencing already freed weak pointer: {}",
                std::any::type_name::<T>()
            );
            backtrace();
            panic!(
                "Defererencing already freed weak pointer: {}",
                std::any::type_name::<T>()
            );
        }
    }
}

impl<T: ?Sized> Deref for Weak<T> {
    type Target = T;
    fn deref(&self) -> &T {
        self.check();
        unsafe { self.ptr.unwrap().as_ref() }
    }
}

impl<T: ?Sized> DerefMut for Weak<T> {
    fn deref_mut(&mut self) -> &mut T {
        self.check();
        unsafe { self.ptr.unwrap().as_mut() }
    }
}

impl<T: ?Sized> Default for Weak<T> {
    fn default() -> Self {
        Self {
            address: 0,
            ptr:     None,
        }
    }
}

// TODO: Coerce
// impl<T, U> CoerceUnsized<Weak<U>> for Weak<T>
//     where
//         T: Unsize<U> + ?Sized,
//         U: ?Sized,
// {
// }