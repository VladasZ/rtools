use std::any::Any;

pub trait AsAny {
    fn as_any(&self) -> &dyn Any;
}

// impl<T> AsAny for T {
//     fn as_any(&self) -> &dyn Any {
//         self
//     }
// }
