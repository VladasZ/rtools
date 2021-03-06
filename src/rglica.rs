use std::{
    fmt::{Debug, Formatter},
    ops::{Deref, DerefMut},
    ptr::NonNull,
};

use crate::{address::Address, backtrace, bytes::data_pointer};

pub struct Rglica<T: ?Sized> {
    pub ptr: Option<NonNull<T>>,
}

unsafe impl<T: ?Sized> Send for Rglica<T> {}
unsafe impl<T: ?Sized> Sync for Rglica<T> {}

impl<T: ?Sized> Copy for Rglica<T> {}

impl<T: ?Sized> Clone for Rglica<T> {
    fn clone(&self) -> Rglica<T> {
        Self { ptr: self.ptr }
    }
}

impl<T: ?Sized> Rglica<T> {
    pub fn from_ref(rf: &T) -> Rglica<T> {
        let ptr = NonNull::new(rf as *const T as *mut T);
        debug_assert!(ptr.is_some(), "Failed to cast ref to Rglica");
        Self {
            ptr: unsafe { ptr.unwrap_unchecked().into() },
        }
    }

    pub fn is_null(&self) -> bool {
        self.ptr.is_none()
    }

    pub fn is_ok(&self) -> bool {
        self.ptr.is_some()
    }

    pub fn invalidate(&mut self) {
        self.ptr = None
    }
}

impl<T: ?Sized> Deref for Rglica<T> {
    type Target = T;
    fn deref(&self) -> &T {
        if self.is_null() {
            error!("Null Rglica: {}", std::any::type_name::<T>());
            backtrace();
            panic!("Null Rglica: {}", std::any::type_name::<T>());
        }
        unsafe { self.ptr.unwrap_unchecked().as_ref() }
    }
}

impl<T: ?Sized> DerefMut for Rglica<T> {
    fn deref_mut(&mut self) -> &mut T {
        if self.is_null() {
            error!("Null Rglica: {}", std::any::type_name::<T>());
            backtrace();
            panic!("Null Rglica: {}", std::any::type_name::<T>());
        }
        unsafe { self.ptr.unwrap_unchecked().as_mut() }
    }
}

impl<T: ?Sized> Default for Rglica<T> {
    fn default() -> Rglica<T> {
        Self { ptr: None }
    }
}

impl<T: ?Sized> Rglica<T> {
    pub const fn const_default() -> Self {
        Self { ptr: None }
    }
}

pub trait ToRglica<T: ?Sized> {
    fn to_rglica(&self) -> Rglica<T>;
}

impl<T: ?Sized> ToRglica<T> for Box<T> {
    fn to_rglica(&self) -> Rglica<T> {
        let ptr = NonNull::new(self.as_ref() as *const T as *mut T);
        debug_assert!(ptr.is_some(), "Failed to make Rglica from Box");
        Rglica {
            ptr: unsafe { ptr.unwrap_unchecked().into() },
        }
    }
}

impl<T: ?Sized> ToRglica<T> for &T {
    fn to_rglica(&self) -> Rglica<T> {
        Rglica::from_ref(self)
    }
}

impl<T: ?Sized> ToRglica<T> for &mut T {
    fn to_rglica(&self) -> Rglica<T> {
        Rglica::from_ref(self)
    }
}

impl<T: ?Sized> Debug for Rglica<T> {
    default fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.ptr.fmt(f)
    }
}

impl<T: ?Sized + Debug> Debug for Rglica<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.is_null() {
            return self.ptr.fmt(f);
        }
        self.deref().fmt(f)
    }
}

impl<T: ?Sized> Address for Rglica<T> {
    fn address(&self) -> u64 {
        data_pointer(self.ptr)
    }
}
