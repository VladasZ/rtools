use std::{
    fs,
    ops::{Deref, DerefMut},
    path::PathBuf,
};

use serde::{de::DeserializeOwned, Serialize};

pub trait Wrappable: Serialize + DeserializeOwned + Default {}
impl<T: Serialize + DeserializeOwned + Default> Wrappable for T {}

fn executable_name() -> String {
    std::env::current_exe()
        .expect("Failed to get std::env::current_exe()")
        .file_name()
        .expect("Failed to get executable name")
        .to_string_lossy()
        .into()
}

fn storage_dir() -> PathBuf {
    dirs::home_dir()
        .expect("Failed to get home directory")
        .join(".".to_owned() + &executable_name())
}

fn set_value<T: Serialize>(value: &T, key: &str) {
    let json = serde_json::to_string(value).expect("Failed to serialize data");
    let dir = storage_dir();
    if !dir.exists() {
        fs::create_dir(&dir).expect("Failed to create dir")
    }
    fs::write(dir.join(key), json).expect("Failed to write to file");
}

fn get_value<T: Wrappable>(key: &str) -> T {
    let dir = storage_dir();
    let path = dir.join(key);
    if !dir.exists() {
        fs::create_dir(dir).expect("Failed to create dir");
    }
    if !path.exists() {
        let new = T::default();
        set_value(&new, key);
        return new;
    }
    let json = fs::read_to_string(path).expect("Failed to read file");
    serde_json::from_str(&json).expect("Failet to parse json")
}

pub struct Persistent<T: Wrappable> {
    name: &'static str,
    data: T,
}

impl<T: Wrappable> Persistent<T> {
    pub fn new(name: &'static str) -> Self {
        let mut new = Self {
            name,
            data: T::default(),
        };
        new.get();
        new
    }

    pub fn get(&mut self) {
        self.data = get_value(self.name)
    }

    pub fn store(&self) {
        set_value(&self.data, self.name)
    }
}

impl<T: Wrappable> Deref for Persistent<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T: Wrappable> DerefMut for Persistent<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}
