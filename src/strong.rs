use std::{
    alloc::{dealloc, Layout},
    collections::HashMap,
    ops::Deref,
};

use crate::{address::Address, static_default};

pub type Strong<T> = Box<T>;

#[derive(Default)]
struct RefCounters {
    counters: HashMap<u64, (u64, Box<dyn FnOnce()>)>,
}
static_default!(RefCounters);

impl RefCounters {
    fn exists(addr: u64) -> bool {
        Self::get().counters.contains_key(&addr)
    }

    fn strong_count(addr: u64) -> u64 {
        Self::get()
            .counters
            .get(&addr)
            .expect("Failed to get strong count")
            .0
    }

    fn add_strong(addr: u64, dealloc_fn: impl FnOnce() + 'static) {
        if Self::exists(addr) {
            panic!("A");
        };
        Self::get().counters.insert(addr, (1, Box::new(dealloc_fn)));
    }

    fn increase_strong(addr: u64) {
        Self::get()
            .counters
            .get_mut(&addr)
            .expect("Failed to increase strong count")
            .0 += 1;
    }

    fn decrease_strong(addr: u64) {
        Self::get()
            .counters
            .get_mut(&addr)
            .expect("Failed to decrease strong count")
            .0 -= 1;
    }

    fn remove(addr: u64) {
        let counter = Self::get()
            .counters
            .remove(&addr)
            .expect("Removing non existing address");

        counter.1()
    }
}

pub struct Streng1<T: ?Sized> {
    address: u64,
    ptr:     *const T,
}

impl<T: Sized> Streng1<T> {
    fn new(val: T) -> Self {
        let dealloc_fn = Box::new(|ptr: *const T| unsafe {
            dealloc(ptr as *mut u8, Layout::new::<T>());
        });

        let val = Box::new(val);
        let address = val.deref().address();
        let ptr = Box::leak(val) as *const T;

        RefCounters::increase_strong(address);

        Self { address, ptr }
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
