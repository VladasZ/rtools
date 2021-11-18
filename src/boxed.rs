pub trait Boxed {
    fn boxed() -> Box<Self>
    where
        Self: Sized;
}

impl<T: Default> Boxed for T {
    fn boxed() -> Box<Self> {
        Box::new(Default::default())
    }
}
