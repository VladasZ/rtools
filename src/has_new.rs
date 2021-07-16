pub trait HasNew {
    fn new() -> Self
    where
        Self: Sized;
}

pub fn new<T: HasNew>() -> T {
    T::new()
}
