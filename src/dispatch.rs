use std::{future::Future, sync::Mutex};

use tokio::spawn;

use crate::{misc::sleep, IntoF32};

type Storage = Mutex<Vec<Box<dyn FnOnce() + Send>>>;

static STORAGE: Storage = Storage::new(Default::default());

pub struct Dispatch;

impl Dispatch {
    pub fn dispatch<T: Send + 'static>(
        fut: impl Future<Output = T> + Send + 'static,
        completion: impl FnOnce(T) + Send + 'static,
    ) {
        spawn(async {
            let val = fut.await;
            STORAGE
                .lock()
                .unwrap()
                .push(Box::new(move || completion(val)));
        });
    }

    pub fn after(delay: impl IntoF32, action: impl FnOnce() + Send + 'static) {
        spawn(async move {
            sleep(delay);
            STORAGE.lock().unwrap().push(Box::new(action));
        });
    }

    pub fn call() {
        let mut data = STORAGE.lock().unwrap();
        while let Some(action) = data.pop() {
            action()
        }
    }
}
