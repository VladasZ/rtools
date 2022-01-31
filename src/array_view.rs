use std::{fmt::Debug, mem::size_of};

use num_traits::Num;

#[derive(Debug)]
pub struct ArrayView<T> {
    pub data: *const T,
    pub size: usize,
}

impl<T: Num + Debug> ArrayView<T> {
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

impl<T, const N: usize> From<&'static [T; N]> for ArrayView<T> {
    fn from(arr: &'static [T; N]) -> Self {
        Self {
            data: &arr[0],
            size: N,
        }
    }
}

impl<T: Num, Value> From<&Vec<Value>> for ArrayView<T> {
    fn from(vector: &Vec<Value>) -> Self {
        Self {
            data: &vector[0] as *const Value as *const T,
            size: vector.len() * (size_of::<Value>() / size_of::<T>()),
        }
    }
}
