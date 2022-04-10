use std::marker::PhantomData;

use crate::data_manager::Managed;

pub struct Handle<T> {
    _hash: u64,
    _data: PhantomData<T>,
}

impl<T: Managed> Handle<T> {
    fn _get(&self) -> &T {
        todo!()
    }
}
