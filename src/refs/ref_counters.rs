use std::collections::HashMap;

use crate::static_default;

#[derive(Default)]
pub(crate) struct RefCounters {
    counters: HashMap<u64, (u64, Box<dyn FnOnce()>)>,
}
static_default!(RefCounters);

impl RefCounters {
    fn exists(addr: u64) -> bool {
        Self::get().counters.contains_key(&addr)
    }

    pub(crate) fn strong_count(addr: u64) -> u64 {
        Self::get()
            .counters
            .get(&addr)
            .expect("Failed to get strong count")
            .0
    }

    pub(crate) fn add_strong(addr: u64, dealloc_fn: impl FnOnce() + 'static) {
        if Self::exists(addr) {
            Self::increase_strong(addr)
        } else {
            Self::get().counters.insert(addr, (1, Box::new(dealloc_fn)));
        }
    }

    pub(crate) fn increase_strong(addr: u64) {
        Self::get()
            .counters
            .get_mut(&addr)
            .expect("Failed to increase strong count")
            .0 += 1;
    }

    pub(crate) fn decrease_strong(addr: u64) {
        Self::get()
            .counters
            .get_mut(&addr)
            .expect("Failed to decrease strong count")
            .0 -= 1;
    }

    pub(crate) fn remove(addr: u64) {
        let counter = Self::get()
            .counters
            .remove(&addr)
            .expect("Removing non existing address");

        counter.1()
    }
}
