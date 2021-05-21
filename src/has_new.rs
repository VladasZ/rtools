
pub trait HasNew {
    fn new() -> Self where Self: Sized;
}
