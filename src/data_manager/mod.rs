mod handle;

use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

pub use handle::Handle;

use crate::misc::hash;

pub type DataStorage<T> = HashMap<u64, T>;

pub trait Managed: 'static + LoadFromPath + DataManager<Self> {}

pub trait LoadFromPath: Sized + Clone {
    fn load(path: &Path) -> Self;
}

pub trait DataManager<T: 'static + LoadFromPath> {
    fn path() -> PathBuf;
    fn set_path(path: &Path);

    fn storage() -> &'static mut DataStorage<T>;

    fn get(name: &str) -> &T {
        Self::storage()
            .entry(hash(name))
            .or_insert_with(|| T::load(&Self::path().join(name)))
    }
}
