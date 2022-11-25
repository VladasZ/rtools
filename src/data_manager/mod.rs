mod handle;
mod managed;

use std::{collections::HashMap, path::Path};

pub use handle::Handle;
use refs::Own;

use crate::misc::hash;

pub type DataStorage<T> = HashMap<u64, Own<T>>;

pub trait Managed: 'static + LoadFromPath + DataManager<Self> {}

pub trait LoadFromPath: Sized {
    fn load(path: &Path) -> Self;
}

pub trait DataManager<T: Managed> {
    fn path() -> &'static Path;
    fn set_path(path: &Path);

    fn storage() -> &'static mut DataStorage<T>;

    fn add_with_hash(hash: u64, resource: T) -> Handle<T> {
        let storage = Self::storage();
        debug_assert!(
            !storage.contains_key(&hash),
            "Object with such hash already exists"
        );
        storage.insert(hash, Own::new(resource));
        hash.into()
    }

    fn handle_with_name(name: impl ToString) -> Option<Handle<T>> {
        Self::handle_with_hash(hash(name.to_string()))
    }

    fn handle_with_hash(hash: u64) -> Option<Handle<T>> {
        if Self::storage().contains_key(&hash) {
            Some(hash.into())
        } else {
            None
        }
    }

    fn remove_with_name(name: impl ToString) {
        Self::remove_with_hash(hash(name.to_string()))
    }

    fn remove_with_hash(hash: u64) {
        Self::storage().remove(&hash);
    }

    fn get_ref_by_hash(hash: u64) -> &'static T {
        Self::storage().get(&hash).unwrap()
    }

    fn get_ref_by_hash_mut(hash: u64) -> &'static mut T {
        Self::storage().get_mut(&hash).unwrap()
    }

    fn get(name: impl ToString) -> Handle<T> {
        let name = name.to_string();
        let hash = hash(&name);
        Self::storage()
            .entry(hash)
            .or_insert_with(|| Own::new(T::load(&Self::path().join(name))));
        hash.into()
    }
}
