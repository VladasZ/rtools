use std::{collections::BTreeMap, sync::Mutex};

static STORAGE: Mutex<BTreeMap<String, usize>> = Mutex::new(BTreeMap::new());

pub fn impl_times(label: impl ToString, counter: usize, mut closure: impl FnMut()) {
    let label = label.to_string();
    let mut storage = STORAGE.lock().unwrap();

    let stored_counter = match storage.get_mut(&label) {
        Some(counter) => counter,
        None => {
            storage.insert(label.clone(), 0);
            storage.get_mut(&label).unwrap()
        }
    };

    if *stored_counter >= counter {
        return;
    }

    closure();

    *stored_counter += 1;
}

#[macro_export]
macro_rules! times {
    ($n:expr, $closure:expr) => {{
        rtools::impl_times(format!("{}:{}", file!(), line!()), $n, $closure);
    }};
}
