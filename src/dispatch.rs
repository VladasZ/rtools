use std::{future::Future, sync::Mutex};

use tokio::spawn;

use crate::{static_storage, StaticStorage};

type Storage = Mutex<Vec<Box<dyn Fn()>>>;

static_storage!(Actions, Storage);

pub struct Dispatch;

impl Dispatch {
    pub fn dispatch<T: Copy + 'static>(
        fut: impl Future<Output = T> + Send + 'static,
        completion: impl Fn(T) + Send + 'static,
    ) {
        spawn(async {
            let val = fut.await;
            let data = Actions::get_mut().get_mut().unwrap();
            data.push(Box::new(move || completion(val)));
        });
    }

    pub fn call() {
        let data = Actions::get_mut().get_mut().unwrap();
        for action in data.iter() {
            action()
        }
        data.clear()
    }
}
