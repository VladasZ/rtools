use crate::refs::make_shared;
use crate::refs::{MutWeak, Shared};
use crate::HasNew;
use std::rc::Rc;

pub trait HasWeakSelf: HasNew {
    fn weak(&self) -> MutWeak<Self>
    where
        Self: Sized;
    fn set_weak(&mut self, weak: MutWeak<Self>);

    fn new_shared() -> Shared<Self>
    where
        Self: Sized,
    {
        let result = make_shared(Self::new());
        result
            .try_borrow_mut()
            .unwrap()
            .set_weak(Rc::downgrade(&result));
        result
    }
}
