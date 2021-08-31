use crate::refs::Shared;
use std::fmt::{Debug, Formatter};
use std::ops::{Deref, DerefMut};

pub struct Own<T: ?Sized> {
    pub pointer: Shared<T>,
}

impl<T: ?Sized> Own<T> {
    pub fn from(value: Shared<T>) -> Self {
        Self { pointer: value }
    }

    // pub fn from<Value: ?Sized>(value: Value) -> Self {
    //     let layout = Layout::new::<Value>();
    //     Self {
    //         pointer: unsafe {
    //             let ptr = alloc(layout) as *mut Value;
    //             *ptr = value;
    //             ptr as *mut u8 as *mut T
    //         },
    //         layout,
    //     }
    // }
}

// impl<T: ?Sized> Drop for Own<T> {
//     fn drop(&mut self) {
//         unsafe { dealloc(self.pointer as *mut u8, Own::<T>::LAYOUT) }
//     }
// }

impl<T: ?Sized> Deref for Own<T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { &*self.pointer.as_ptr() }
    }
}

impl<T: ?Sized> DerefMut for Own<T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.pointer.as_ptr() }
    }
}

impl<T: Debug> Debug for Own<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.deref().fmt(f)
    }
}
