use std::{ops::Deref, ptr::NonNull};

use crate::{Rglica, UnwrapBox};

pub type Weak<T> = Rglica<T>;

pub trait ToWeak<T: ?Sized> {
    fn weak(&self) -> Rglica<T>;
}

impl<T: ?Sized> ToWeak<T> for Box<T> {
    fn weak(&self) -> Rglica<T> {
        let ptr = NonNull::new(self.as_ref() as *const T as *mut T);
        debug_assert!(ptr.is_some(), "Failed to make Rglica from Box");
        Rglica {
            ptr: ptr.unwrap().into(),
        }
    }
}

impl<T: ?Sized> ToWeak<T> for &T {
    fn weak(&self) -> Rglica<T> {
        Rglica::from_ref(self)
    }
}

impl<T: ?Sized> ToWeak<T> for &mut T {
    fn weak(&self) -> Rglica<T> {
        Rglica::from_ref(self)
    }
}

impl<T: ?Sized> ToWeak<T> for UnwrapBox<T> {
    fn weak(&self) -> Rglica<T> {
        self.deref().weak()
    }
}
