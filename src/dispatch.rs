use crate::{static_storage, StaticStorage};

type Storage = Vec<Box<dyn Fn()>>;

static_storage!(Actions, Storage);

pub struct Dispatch;

impl Dispatch {
    pub fn dispatch(action: impl Fn() + 'static) {
        Actions::get_mut().push(Box::new(action));
    }

    pub fn call() {
        let data = Actions::get_mut();
        for action in data.iter() {
            action()
        }
        data.clear()
    }
}
