mod handle;

use std::{collections::HashMap, path::Path};

pub use handle::Handle;

use crate::misc::hash;

pub type DataStorage<T> = HashMap<u64, T>;

pub trait Managed: 'static + LoadFromPath + DataManager<Self> {}

pub trait LoadFromPath: Sized {
    fn load(path: &Path) -> Self;
}

pub trait DataManager<T: 'static + Managed> {
    fn path() -> &'static Path;
    fn set_path(path: &Path);

    fn storage() -> &'static mut DataStorage<T>;

    fn add_with_hash(hash: u64, resource: T) -> Handle<T> {
        let storage = Self::storage();
        debug_assert!(
            !storage.contains_key(&hash),
            "Object with such hash already exists"
        );
        storage.insert(hash, resource);
        hash.into()
    }

    fn get_by_hash(hash: u64) -> &'static T {
        &Self::storage()[&hash]
    }

    fn get(name: &str) -> Handle<T> {
        let hash = hash(name);
        Self::storage()
            .entry(hash)
            .or_insert_with(|| T::load(&Self::path().join(name)));
        hash.into()
    }
}
