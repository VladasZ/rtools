use crate::refs::{Shared, MutWeak};
use crate::new::New;

pub trait HasWeakSelf : New {
    fn new_shared() -> Shared<Self> where Self: Sized;
    fn weak(&self) -> MutWeak<Self> where Self: Sized;
}
