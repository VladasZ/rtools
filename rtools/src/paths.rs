use std::{env, path::PathBuf};

use dirs::home_dir;

pub struct Paths {}

impl Paths {
    pub fn home() -> PathBuf {
        home_dir().unwrap_or_default()
    }

    pub fn pwd() -> PathBuf {
        env::current_dir().unwrap()
    }
}
