use std::{
    fmt::{Debug, Formatter},
    fs,
    marker::PhantomData,
    path::PathBuf,
};

use serde::{de::DeserializeOwned, Serialize};

use crate::platform::Platform;

pub trait Wrappable: Serialize + DeserializeOwned + Send + Default {}
impl<T: Serialize + DeserializeOwned + Send + Default> Wrappable for T {}

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

fn set_value<T: Serialize>(value: T, key: &str) {
    let json = serde_json::to_string(&value).expect("Failed to serialize data");
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
    _p:   PhantomData<T>,
}

impl<T: Wrappable> Stored<T> {
    pub const fn new(name: &'static str) -> Self {
        Self {
            name,
            _p: PhantomData,
        }
    }

    pub fn set(&self, val: impl Into<T>) {
        let val = val.into();
        set_value(val, self.name)
    }

    pub fn get(&self) -> T {
        get_value(self.name)
    }

    pub fn reset(&self) {
        self.set(T::default())
    }
}

impl<T: Wrappable + Debug> Debug for Stored<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.get().fmt(f)
    }
}

#[cfg(test)]
mod test {
    use anyhow::Result;
    use tokio::spawn;

    use crate::{random::Random, Stored};

    static STORED: Stored<i32> = Stored::new("stored_test");

    fn check_send<T: Send>(_send: &T) {}
    fn check_sync<T: Sync>(_sync: &T) {}

    #[tokio::test]
    async fn stored() -> Result<()> {
        check_send(&STORED);
        check_sync(&STORED);

        STORED.reset();
        assert_eq!(STORED.get(), i32::default());

        for _ in 0..10 {
            let rand = i32::random();

            spawn(async move {
                STORED.set(rand);
            })
            .await?;

            spawn(async move {
                assert_eq!(STORED.get(), rand);
            })
            .await?;
        }

        Ok(())
    }
}
