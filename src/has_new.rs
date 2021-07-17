pub trait New {
    fn new() -> Self
    where
        Self: Sized;
}

pub fn new<T: New>() -> T {
    T::new()
}
