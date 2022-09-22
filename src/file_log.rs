use std::fs::{File, OpenOptions};
use crate::static_get;
use std::io::Write;

pub struct FileLog {
    file: File
}

static_get!(FileLog);

impl FileLog {
    pub fn write(text: impl ToString) {
        Self::get().file.write_all(format!("{}\n", text.to_string()).as_bytes())
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
