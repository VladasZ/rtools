
pub trait New {
    fn new() -> Self where Self: Sized;
}
