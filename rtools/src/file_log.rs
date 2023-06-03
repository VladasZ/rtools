use std::{
    fs::{File, OpenOptions},
    io::Write,
    sync::{Mutex, OnceLock},
};

static LOG: OnceLock<FileLog> = OnceLock::new();

pub struct FileLog {
    file: Mutex<File>,
}

impl FileLog {
    fn init() -> Self {
        Self {
            file: OpenOptions::new()
                .write(true)
                .truncate(true)
                .open("logs.txt")
                .expect("Unable to create logs file")
                .into(),
        }
    }

    fn get() -> &'static Self {
        LOG.get_or_init(Self::init)
    }

    pub fn write(text: impl ToString) {
        Self::get()
            .file
            .lock()
            .unwrap()
            .write_all(format!("{}\n", text.to_string()).as_bytes())
            .expect("Failed to write log");
    }
}
