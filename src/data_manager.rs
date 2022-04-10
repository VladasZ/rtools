use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

pub type DataStorage<T> = HashMap<String, T>;

pub trait LoadFromPath: Sized + Clone {
    fn load(path: &Path) -> Self;
}

pub trait DataManager<T: LoadFromPath> {
    fn path() -> PathBuf;
    fn set_path(path: &Path);
    fn storage(a: &mut dyn FnMut(&mut DataStorage<T>));

    fn get(name: &str) -> T {
        let mut resource: Option<T> = None;

        Self::storage(&mut |storage| {
            resource = if storage.contains_key(name) {
                storage[name].clone()
            } else {
                let resource = T::load(&Self::path().join(name));
                storage.insert(name.into(), resource.clone());
                resource
            }
            .into()
        });

        resource.unwrap()
    }
}
