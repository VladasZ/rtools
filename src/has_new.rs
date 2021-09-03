pub trait New {
    fn new() -> Self
    where
        Self: Sized;
}

pub fn new<T: New>() -> T {
    T::new()
}

impl<T: Default> New for T {
    fn new() -> Self {
        Self::default()
    }
}

pub trait Boxed {
    fn boxed() -> Box<Self> where Self: Sized;
}
