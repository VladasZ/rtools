use std::fs;
use std::path::{Path, PathBuf};


struct File;

impl File {
    pub fn read(path: impl AsRef<Path>) -> String {
        fs::read_to_string(path).unwrap()
    }

    pub fn write(data: impl AsRef<String>, path: impl Into<PathBuf>) {

    }
}