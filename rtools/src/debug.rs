use std::{collections::BTreeMap, sync::Mutex};

static STORAGE: Mutex<BTreeMap<String, usize>> = Mutex::new(BTreeMap::new());

pub fn _impl_times(label: impl ToString, counter: usize, mut closure: impl FnMut()) {
    let label = label.to_string();
    let mut storage = STORAGE.lock().unwrap();

    let stored_counter = if let Some(counter) = storage.get_mut(&label) {
        counter
    } else {
        storage.insert(label.clone(), 0);
        storage.get_mut(&label).unwrap()
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
        rtools::_impl_times(format!("{}:{}", file!(), line!()), $n, $closure);
    }};
}

#[cfg(test)]
mod test {
    use crate::_impl_times;

    #[test]
    fn test() {
        let val = &mut 0;

        let mut inc = || {
            *val += 1;
        };

        _impl_times("a", 2, &mut inc);
        _impl_times("a", 2, &mut inc);
        _impl_times("a", 2, &mut inc);
        _impl_times("a", 2, &mut inc);
        _impl_times("a", 2, &mut inc);
        _impl_times("a", 2, &mut inc);
        _impl_times("a", 2, &mut inc);

        assert_eq!(*val, 2);
    }
}
