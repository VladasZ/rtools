use std::{future::Future, sync::Mutex};

use lazy_static::lazy_static;
use tokio::spawn;

use crate::{misc::sleep, IntoF32};

type Storage = Mutex<Vec<Box<dyn FnOnce() + Send>>>;

lazy_static! {
    static ref NEW_STORAGE: Storage = Default::default();
}

pub struct Dispatch;

impl Dispatch {
    pub fn dispatch<T: Send + 'static>(
        fut: impl Future<Output = T> + Send + 'static,
        completion: impl FnOnce(T) + Send + 'static,
    ) {
        spawn(async {
            let val = fut.await;
            let mut data = NEW_STORAGE.lock().unwrap();
            data.push(Box::new(move || completion(val)));
        });
    }

    pub fn after(delay: impl IntoF32, action: impl FnOnce() + Send + 'static) {
        spawn(async move {
            sleep(delay);
            let mut data = NEW_STORAGE.lock().unwrap();
            data.push(Box::new(action));
        });
    }

    pub fn call() {
        let mut data = NEW_STORAGE.lock().unwrap();
        while let Some(action) = data.pop() {
            action()
        }
    }
}
