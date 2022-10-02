use std::collections::HashMap;

use crate::static_default;

#[derive(Default)]
pub(crate) struct RefCounters {
    counters: HashMap<usize, (u64, Box<dyn FnOnce()>)>,
}
static_default!(RefCounters);

impl RefCounters {
    pub(crate) fn exists(addr: usize) -> bool {
        Self::get().counters.contains_key(&addr)
    }

    pub(crate) fn strong_count(addr: usize) -> u64 {
        Self::get()
            .counters
            .get(&addr)
            .expect("Failed to get strong count")
            .0
    }

    pub(crate) fn add_strong(addr: usize, dealloc_fn: impl FnOnce() + 'static) {
        if Self::exists(addr) {
            Self::increase_strong(addr)
        } else {
            Self::get().counters.insert(addr, (1, Box::new(dealloc_fn)));
        }
    }

    pub(crate) fn increase_strong(addr: usize) {
        Self::get()
            .counters
            .get_mut(&addr)
            .expect("Failed to increase strong count")
            .0 += 1;
    }

    pub(crate) fn decrease_strong(addr: usize) {
        Self::get()
            .counters
            .get_mut(&addr)
            .expect("Failed to decrease strong count")
            .0 -= 1;
    }

    pub(crate) fn remove(addr: usize) {
        let counter = Self::get()
            .counters
            .remove(&addr)
            .expect("Removing non existing address");

        counter.1()
    }
}