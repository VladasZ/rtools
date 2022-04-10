use std::{marker::PhantomData, ops::Deref};

use crate::data_manager::Managed;

#[derive(Debug)]
pub struct Handle<T: Managed> {
    hash:  u64,
    _data: PhantomData<T>,
}

impl<T: Managed> Handle<T> {
    pub fn is_ok(&self) -> bool {
        !self.is_null()
    }

    pub fn is_null(&self) -> bool {
        self.hash == u64::MAX
    }

    pub fn get(&self) -> Option<&T> {
        if self.is_null() {
            return None;
        }
        T::get_by_hash(self.hash).into()
    }
}

impl<T: Managed> Deref for Handle<T> {
    type Target = T;
    fn deref(&self) -> &T {
        debug_assert!(self.is_ok());
        T::get_by_hash(self.hash)
    }
}

impl<T: Managed> Default for Handle<T> {
    fn default() -> Self {
        Self {
            hash:  u64::MAX,
            _data: Default::default(),
        }
    }
}

impl<T: Managed> From<u64> for Handle<T> {
    fn from(hash: u64) -> Self {
        Self {
            hash,
            _data: Default::default(),
        }
    }
}

impl<T: Managed> Copy for Handle<T> {}

impl<T: Managed> Clone for Handle<T> {
    fn clone(&self) -> Self {
        Self {
            hash:  self.hash,
            _data: Default::default(),
        }
    }
}
