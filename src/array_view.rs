use std::fmt::Debug;

extern crate num_traits;

#[derive(Debug)]
pub struct ArrayView<T> {
    pub data: *const T,
    pub size: usize,
}

impl<T: num_traits::Num + Debug> ArrayView<T> {
    pub fn from_vector<VecT>(vector: &Vec<VecT>) -> Self {
        Self {
            data: &vector[0] as *const VecT as *const T,
            size: vector.len() * (std::mem::size_of::<VecT>() / std::mem::size_of::<T>()),
        }
    }

    pub fn from_array(array: &[T]) -> Self {
        Self { data: &array[0], size: array.len() }
    }

    pub fn print(&self) {
        unsafe {
            let mut ptr = self.data;
            for _ in 0..self.size {
                print!("{:?} ", *ptr);
                ptr = ptr.offset(1);
            }
            println!();
        }
    }
}
