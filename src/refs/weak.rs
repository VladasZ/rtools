use std::{ops::Deref, ptr::NonNull};

use crate::{Rglica, UnwrapBox};

pub type Weak<T> = Rglica<T>;

pub struct Woke1<T: ?Sized> {
    address: u64,
    ptr:     *mut T,
}
