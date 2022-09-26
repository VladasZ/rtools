use std::{
    fs::{File, OpenOptions},
    io::Write,
};

use crate::static_init;

pub struct FileLog {
    file: File,
}

static_init!(FileLog);

impl FileLog {
    pub fn write(text: impl ToString) {
        Self::get()
            .file
            .write_all(format!("{}\n", text.to_string()).as_bytes())
            .expect("Failed to write log");
    }
}

impl Default for FileLog {
    fn default() -> Self {
        let file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open("logs.txt")
            .expect("Unable to create logs file");
        Self { file }
    }
}
