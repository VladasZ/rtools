use std::{
    cell::RefCell,
    fmt::{Debug, Formatter},
    fs,
    ops::Deref,
    path::PathBuf,
};

use serde::{de::DeserializeOwned, Serialize};

use crate::platform::Platform;

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
    if Platform::MOBILE {
        dirs::document_dir()
    } else {
        dirs::home_dir()
    }
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

pub struct Stored<T: Wrappable> {
    name: &'static str,
    data: RefCell<T>,
}

impl<T: Wrappable> Stored<T> {
    pub fn new(name: &'static str) -> Self {
        Self {
            name,
            data: RefCell::new(T::default()),
        }
    }

    pub fn set(&self, val: impl Into<T>) {
        self.data.replace(val.into());
        set_value(&self.data, self.name)
    }

    pub fn get(&self) -> &T {
        self.data.replace(get_value(self.name));
        unsafe { self.data.as_ptr().as_ref().unwrap() }
    }

    pub fn reset(&self) {
        self.set(T::default())
    }
}

impl<T: Wrappable> Deref for Stored<T> {
    type Target = T;
    fn deref(&self) -> &T {
        self.get()
    }
}

impl<T: Wrappable + Debug> Debug for Stored<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.get().fmt(f)
    }
}
