use std::{future::Future, ops::DerefMut, sync::Mutex};

use lazy_static::lazy_static;
use tokio::spawn;

use crate::{misc::sleep, rglica::ToRglica, IntoF32};

type Storage = Mutex<Vec<Box<dyn FnOnce() + Send>>>;

lazy_static! {
    static ref STORAGE: Storage = Default::default();
}

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

    pub fn after<Obj: 'static>(
        obj: &Obj,
        delay: impl IntoF32,
        action: impl FnOnce(&mut Obj) + Send + 'static,
    ) {
        let mut rglica = obj.to_rglica();
        spawn(async move {
            sleep(delay);
            STORAGE
                .lock()
                .unwrap()
                .push(Box::new(move || action(rglica.deref_mut())));
        });
    }

    pub fn call() {
        let mut data = STORAGE.lock().unwrap();
        while let Some(action) = data.pop() {
            action()
        }
    }
}
